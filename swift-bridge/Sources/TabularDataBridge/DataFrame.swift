import Foundation
import TabularData

struct TDColumnPayload: Codable {
    var name: String
    var kind: String
    var values: [TDJSONValue]
}

struct TDCSVReadingOptionsPayload: Codable {
    var has_header_row: Bool
    var delimiter: String
    var ignores_empty_lines: Bool
    var uses_quoting: Bool
    var uses_escaping: Bool
}

struct TDCSVWritingOptionsPayload: Codable {
    var includes_header: Bool
    var nil_encoding: String
    var true_encoding: String
    var false_encoding: String
    var newline: String
    var delimiter: String
}

private final class TDDataFrameBox: NSObject {
    var frame: DataFrame

    init(frame: DataFrame) {
        self.frame = frame
        super.init()
    }
}

private func td_box(_ ptr: UnsafeMutableRawPointer?) -> TDDataFrameBox? {
    guard let ptr else {
        return nil
    }
    let box: TDDataFrameBox = td_borrow(ptr)
    return box
}

private func td_invalid_argument(_ message: String) -> NSError {
    NSError(domain: "tabulardata-rs", code: Int(TDR_INVALID_ARGUMENT), userInfo: [
        NSLocalizedDescriptionKey: message,
    ])
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

private func td_character(_ value: String, fieldName: String) throws -> Character {
    guard let character = value.first, value.count == 1 else {
        throw td_invalid_argument("\(fieldName) must be a single character")
    }
    return character
}

private func td_csv_reading_options(_ payload: TDCSVReadingOptionsPayload) throws -> CSVReadingOptions {
    CSVReadingOptions(
        hasHeaderRow: payload.has_header_row,
        ignoresEmptyLines: payload.ignores_empty_lines,
        usesQuoting: payload.uses_quoting,
        usesEscaping: payload.uses_escaping,
        delimiter: try td_character(payload.delimiter, fieldName: "delimiter")
    )
}

private func td_csv_writing_options(_ payload: TDCSVWritingOptionsPayload) throws -> CSVWritingOptions {
    CSVWritingOptions(
        includesHeader: payload.includes_header,
        nilEncoding: payload.nil_encoding,
        trueEncoding: payload.true_encoding,
        falseEncoding: payload.false_encoding,
        newline: payload.newline,
        delimiter: try td_character(payload.delimiter, fieldName: "delimiter")
    )
}

private func td_join_kind(_ raw: Int32) -> JoinKind {
    switch raw {
    case 1:
        return .left
    case 2:
        return .right
    case 3:
        return .full
    default:
        return .inner
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

private func td_row_objects(_ frame: DataFrame) -> [[String: Any]] {
    let columnNames = frame.columns.map(\.name)
    return frame.rows.map { row in
        var object: [String: Any] = [:]
        for columnName in columnNames {
            object[columnName] = td_json_safe(row[columnName] ?? NSNull())
        }
        return object
    }
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

@_cdecl("td_dataframe_from_csv")
public func td_dataframe_from_csv(
    _ path: UnsafePointer<CChar>?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    guard let path else {
        td_write_error(errorOut, "path must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let options = try td_decode_json(optionsJSON, as: TDCSVReadingOptionsPayload.self)
        let frame = try DataFrame(
            contentsOfCSVFile: URL(fileURLWithPath: String(cString: path)),
            options: td_csv_reading_options(options)
        )
        outFrame.pointee = td_retain(TDDataFrameBox(frame: frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
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

@_cdecl("td_dataframe_summary")
public func td_dataframe_summary(
    _ framePtr: UnsafeMutableRawPointer?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    outFrame.pointee = td_retain(TDDataFrameBox(frame: frame.summary()))
    return TDR_OK
}

@_cdecl("td_dataframe_joined")
public func td_dataframe_joined(
    _ framePtr: UnsafeMutableRawPointer?,
    _ otherPtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ joinKind: Int32,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame,
          let other = td_box(otherPtr)?.frame,
          let columnNamePtr = columnName
    else {
        td_write_error(errorOut, "data frames and column name must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    let columnName = String(cString: columnNamePtr)
    guard frame.indexOfColumn(columnName) != nil else {
        td_write_error(errorOut, "left frame is missing column '\(columnName)'")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }
    guard other.indexOfColumn(columnName) != nil else {
        td_write_error(errorOut, "right frame is missing column '\(columnName)'")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    outFrame.pointee = td_retain(
        TDDataFrameBox(frame: frame.joined(other, on: columnName, kind: td_join_kind(joinKind)))
    )
    return TDR_OK
}

@_cdecl("td_dataframe_write_csv")
public func td_dataframe_write_csv(
    _ framePtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame, let path else {
        td_write_error(errorOut, "data frame and path must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let options = try td_decode_json(optionsJSON, as: TDCSVWritingOptionsPayload.self)
        try frame.writeCSV(
            to: URL(fileURLWithPath: String(cString: path)),
            options: td_csv_writing_options(options)
        )
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}
