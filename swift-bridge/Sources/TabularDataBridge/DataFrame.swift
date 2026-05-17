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

    func optionalDate() throws -> Date? {
        switch self {
        case .null:
            return nil
        case .double(let value):
            return Date(timeIntervalSince1970: value)
        case .int(let value):
            return Date(timeIntervalSince1970: Double(value))
        default:
            throw td_invalid_argument("date columns must contain timestamps or nulls")
        }
    }

    func optionalData() throws -> Data? {
        switch self {
        case .null:
            return nil
        case .string(let value):
            return Data(base64Encoded: value) ?? Data(value.utf8)
        default:
            throw td_invalid_argument("data columns must contain base64 strings or nulls")
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
    case "date":
        return Column(name: payload.name, contents: try payload.values.map { try $0.optionalDate() })
            .eraseToAnyColumn()
    case "data":
        return Column(name: payload.name, contents: try payload.values.map { try $0.optionalData() })
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
    if column.wrappedElementType == Date.self {
        let values = Array(column.assumingType(Date.self)).map { value -> Any in
            value.map(\.timeIntervalSince1970) ?? NSNull()
        }
        return ["name": column.name, "kind": "date", "values": values]
    }
    if column.wrappedElementType == Data.self {
        let values = Array(column.assumingType(Data.self)).map { value -> Any in
            value.map { $0.base64EncodedString() } ?? NSNull()
        }
        return ["name": column.name, "kind": "data", "values": values]
    }
    throw td_invalid_argument("unsupported column type: \(String(describing: column.wrappedElementType))")
}

private func td_normalize_column_type_name(_ value: String) -> String {
    value.replacingOccurrences(of: "Swift.", with: "").lowercased()
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

@_cdecl("td_dataframe_index_of_column")
public func td_dataframe_index_of_column(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ outFound: UnsafeMutablePointer<Int32>,
    _ outIndex: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame, let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        outFound.pointee = 0
        outIndex.pointee = 0
        return TDR_INVALID_ARGUMENT
    }

    if let index = frame.indexOfColumn(String(cString: columnName)) {
        outFound.pointee = 1
        outIndex.pointee = index
    } else {
        outFound.pointee = 0
        outIndex.pointee = 0
    }
    return TDR_OK
}

@_cdecl("td_dataframe_contains_column")
public func td_dataframe_contains_column(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ outContains: UnsafeMutablePointer<Int32>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame, let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        outContains.pointee = 0
        return TDR_INVALID_ARGUMENT
    }

    outContains.pointee = frame.indexOfColumn(String(cString: columnName)) == nil ? 0 : 1
    return TDR_OK
}

@_cdecl("td_dataframe_contains_column_type")
public func td_dataframe_contains_column_type(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ columnType: UnsafePointer<CChar>?,
    _ outContains: UnsafeMutablePointer<Int32>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame, let columnName, let columnType else {
        td_write_error(errorOut, "data frame, column name, and column type must not be null")
        outContains.pointee = 0
        return TDR_INVALID_ARGUMENT
    }

    let name = String(cString: columnName)
    guard let index = frame.indexOfColumn(name) else {
        outContains.pointee = 0
        return TDR_OK
    }

    let wrappedType = td_normalize_column_type_name(String(describing: frame.columns[index].wrappedElementType))
    let requestedType = td_normalize_column_type_name(String(cString: columnType))
    outContains.pointee = wrappedType == requestedType ? 1 : 0
    return TDR_OK
}

@_cdecl("td_dataframe_column_names_for_alias_json")
public func td_dataframe_column_names_for_alias_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ alias: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let frame = td_box(framePtr)?.frame, let alias else {
        td_write_error(errorOut, "data frame and alias must not be null")
        return nil
    }

    return td_string(td_codable_json_string(frame.columnNames(forAlias: String(cString: alias))))
}

@_cdecl("td_dataframe_add_alias")
public func td_dataframe_add_alias(
    _ framePtr: UnsafeMutableRawPointer?,
    _ alias: UnsafePointer<CChar>?,
    _ columnName: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr), let alias, let columnName else {
        td_write_error(errorOut, "data frame, alias, and column name must not be null")
        return TDR_INVALID_ARGUMENT
    }

    let name = String(cString: columnName)
    guard box.frame.indexOfColumn(name) != nil else {
        td_write_error(errorOut, "there is no column named '\(name)'")
        return TDR_INVALID_ARGUMENT
    }

    box.frame.addAlias(String(cString: alias), forColumn: name)
    return TDR_OK
}

@_cdecl("td_dataframe_remove_alias")
public func td_dataframe_remove_alias(
    _ framePtr: UnsafeMutableRawPointer?,
    _ alias: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr), let alias else {
        td_write_error(errorOut, "data frame and alias must not be null")
        return TDR_INVALID_ARGUMENT
    }

    box.frame.removeAlias(String(cString: alias))
    _ = errorOut
    return TDR_OK
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
