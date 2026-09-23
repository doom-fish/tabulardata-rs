import Combine
import Foundation
import TabularData

private struct TDColumnCodingRequestPayload: Codable {
    var column: String
    var element_type: String
    var codec: String
}

private func td_coding_element_type(_ name: String) throws -> Any.Type {
    switch name {
    case "string":
        return String.self
    case "int":
        return Int.self
    case "double":
        return Double.self
    case "bool":
        return Bool.self
    case "date":
        return Date.self
    case "data":
        return Data.self
    default:
        throw td_invalid_argument("unsupported element type '\(name)'")
    }
}

private func td_encode_column(
    frame: inout DataFrame,
    request: TDColumnCodingRequestPayload
) throws {
    let columnType = try td_column_type(request.column, in: frame)
    let requested = try td_coding_element_type(request.element_type)
    guard columnType == requested else {
        throw td_invalid_argument(
            "column '\(request.column)' holds \(td_type_label(columnType)) values, not \(td_type_label(requested))"
        )
    }
    switch (request.element_type, request.codec) {
    case ("string", "json"):
        try frame.encodeColumn(request.column, String.self, using: JSONEncoder())
    case ("string", "property_list"):
        try frame.encodeColumn(request.column, String.self, using: PropertyListEncoder())
    case ("int", "json"):
        try frame.encodeColumn(request.column, Int.self, using: JSONEncoder())
    case ("int", "property_list"):
        try frame.encodeColumn(request.column, Int.self, using: PropertyListEncoder())
    case ("double", "json"):
        try frame.encodeColumn(request.column, Double.self, using: JSONEncoder())
    case ("double", "property_list"):
        try frame.encodeColumn(request.column, Double.self, using: PropertyListEncoder())
    case ("bool", "json"):
        try frame.encodeColumn(request.column, Bool.self, using: JSONEncoder())
    case ("bool", "property_list"):
        try frame.encodeColumn(request.column, Bool.self, using: PropertyListEncoder())
    case ("date", "json"):
        try frame.encodeColumn(request.column, Date.self, using: JSONEncoder())
    case ("date", "property_list"):
        try frame.encodeColumn(request.column, Date.self, using: PropertyListEncoder())
    case ("data", "json"):
        try frame.encodeColumn(request.column, Data.self, using: JSONEncoder())
    case ("data", "property_list"):
        try frame.encodeColumn(request.column, Data.self, using: PropertyListEncoder())
    default:
        throw td_invalid_argument("unsupported encoding combination")
    }
}

private func td_decode_column(
    frame: inout DataFrame,
    request: TDColumnCodingRequestPayload
) throws {
    let columnType = try td_column_type(request.column, in: frame)
    _ = try td_coding_element_type(request.element_type)
    guard columnType == Data.self else {
        throw td_invalid_argument(
            "column '\(request.column)' holds \(td_type_label(columnType)) values; only encoded Data columns can be decoded"
        )
    }
    switch (request.element_type, request.codec) {
    case ("string", "json"):
        try frame.decode(String.self, inColumn: request.column, using: JSONDecoder())
    case ("string", "property_list"):
        try frame.decode(String.self, inColumn: request.column, using: PropertyListDecoder())
    case ("int", "json"):
        try frame.decode(Int.self, inColumn: request.column, using: JSONDecoder())
    case ("int", "property_list"):
        try frame.decode(Int.self, inColumn: request.column, using: PropertyListDecoder())
    case ("double", "json"):
        try frame.decode(Double.self, inColumn: request.column, using: JSONDecoder())
    case ("double", "property_list"):
        try frame.decode(Double.self, inColumn: request.column, using: PropertyListDecoder())
    case ("bool", "json"):
        try frame.decode(Bool.self, inColumn: request.column, using: JSONDecoder())
    case ("bool", "property_list"):
        try frame.decode(Bool.self, inColumn: request.column, using: PropertyListDecoder())
    case ("date", "json"):
        try frame.decode(Date.self, inColumn: request.column, using: JSONDecoder())
    case ("date", "property_list"):
        try frame.decode(Date.self, inColumn: request.column, using: PropertyListDecoder())
    case ("data", "json"):
        try frame.decode(Data.self, inColumn: request.column, using: JSONDecoder())
    case ("data", "property_list"):
        try frame.decode(Data.self, inColumn: request.column, using: PropertyListDecoder())
    default:
        throw td_invalid_argument("unsupported decoding combination")
    }
}

@_cdecl("td_dataframe_encode_column_json")
public func td_dataframe_encode_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let request = try td_decode_json(requestJSON, as: TDColumnCodingRequestPayload.self)
        try td_encode_column(frame: &box.frame, request: request)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_decode_column_json")
public func td_dataframe_decode_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let request = try td_decode_json(requestJSON, as: TDColumnCodingRequestPayload.self)
        try td_decode_column(frame: &box.frame, request: request)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}
