import Foundation
import TabularData

struct TDColumnPayload: Codable {
    var name: String
    var kind: String
    var values: [TDJSONValue]
}

private extension TDJSONValue {
    func optionalString() throws -> String? {
        switch self {
        case .null:
            return nil
        case .string(let value):
            return value
        default:
            throw td_invalid_argument("string columns must contain strings or nulls")
        }
    }

    func optionalInt() throws -> Int? {
        switch self {
        case .null:
            return nil
        case .int(let value):
            guard let value = Int(exactly: value) else {
                throw td_invalid_argument("integer values must fit in Swift.Int")
            }
            return value
        default:
            throw td_invalid_argument("int columns must contain integers or nulls")
        }
    }

    func optionalDouble() throws -> Double? {
        switch self {
        case .null:
            return nil
        case .double(let value):
            return value
        case .int(let value):
            return Double(value)
        default:
            throw td_invalid_argument("double columns must contain numbers or nulls")
        }
    }

    func optionalBool() throws -> Bool? {
        switch self {
        case .null:
            return nil
        case .bool(let value):
            return value
        default:
            throw td_invalid_argument("bool columns must contain booleans or nulls")
        }
    }
}

private func td_make_any_column(_ payload: TDColumnPayload) throws -> AnyColumn {
    switch payload.kind {
    case "string":
        return Column(name: payload.name, contents: try payload.values.map { try $0.optionalString() })
            .eraseToAnyColumn()
    case "int":
        return Column(name: payload.name, contents: try payload.values.map { try $0.optionalInt() })
            .eraseToAnyColumn()
    case "double":
        return Column(name: payload.name, contents: try payload.values.map { try $0.optionalDouble() })
            .eraseToAnyColumn()
    case "bool":
        return Column(name: payload.name, contents: try payload.values.map { try $0.optionalBool() })
            .eraseToAnyColumn()
    default:
        throw td_invalid_argument("unsupported column kind: \(payload.kind)")
    }
}

private func td_column_object(_ column: AnyColumn) throws -> [String: Any] {
    if column.wrappedElementType == String.self {
        let values = Array(column.assumingType(String.self)).map { $0 as Any? ?? NSNull() }
        return ["name": column.name, "kind": "string", "values": values]
    }
    if column.wrappedElementType == Int.self {
        let values = Array(column.assumingType(Int.self)).map { value -> Any in
            value.map { Int64($0) } ?? NSNull()
        }
        return ["name": column.name, "kind": "int", "values": values]
    }
    if column.wrappedElementType == Double.self {
        let values = Array(column.assumingType(Double.self)).map { value -> Any in
            value ?? NSNull()
        }
        return ["name": column.name, "kind": "double", "values": values]
    }
    if column.wrappedElementType == Bool.self {
        let values = Array(column.assumingType(Bool.self)).map { value -> Any in
            value ?? NSNull()
        }
        return ["name": column.name, "kind": "bool", "values": values]
    }
    throw td_invalid_argument("unsupported column type: \(String(describing: column.wrappedElementType))")
}

@_cdecl("td_dataframe_new")
public func td_dataframe_new(
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = td_retain(TDDataFrameBox(frame: DataFrame()))
    _ = errorOut
    return TDR_OK
}

@_cdecl("td_dataframe_shape")
public func td_dataframe_shape(
    _ framePtr: UnsafeMutableRawPointer?,
    _ outRows: UnsafeMutablePointer<Int>,
    _ outColumns: UnsafeMutablePointer<Int>
) {
    guard let frame = td_box(framePtr)?.frame else {
        outRows.pointee = 0
        outColumns.pointee = 0
        return
    }

    outRows.pointee = frame.shape.rows
    outColumns.pointee = frame.shape.columns
}

@_cdecl("td_dataframe_column_names_json")
public func td_dataframe_column_names_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }

    return td_string(td_json_string(frame.columns.map(\.name)))
}

@_cdecl("td_dataframe_append_column")
public func td_dataframe_append_column(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let payload = try td_decode_json(columnJSON, as: TDColumnPayload.self)
        box.frame.append(column: try td_make_any_column(payload))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_rename_column")
public func td_dataframe_rename_column(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ newName: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr), let columnNamePtr = columnName, let newNamePtr = newName else {
        td_write_error(errorOut, "data frame and column names must not be null")
        return TDR_INVALID_ARGUMENT
    }

    let columnName = String(cString: columnNamePtr)
    guard box.frame.indexOfColumn(columnName) != nil else {
        td_write_error(errorOut, "there is no column named '\(columnName)'")
        return TDR_INVALID_ARGUMENT
    }

    box.frame.renameColumn(columnName, to: String(cString: newNamePtr))
    return TDR_OK
}

@_cdecl("td_dataframe_column_json")
public func td_dataframe_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let box = td_box(framePtr), let columnNamePtr = columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return nil
    }

    let columnName = String(cString: columnNamePtr)
    guard box.frame.indexOfColumn(columnName) != nil else {
        td_write_error(errorOut, "there is no column named '\(columnName)'")
        return nil
    }

    do {
        return td_string(try td_json_string(td_column_object(box.frame[columnName])))
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return nil
    }
}

@_cdecl("td_dataframe_rows_json")
public func td_dataframe_rows_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }

    return td_string(td_json_string(td_row_objects(frame)))
}
