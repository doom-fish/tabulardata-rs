import Foundation
import TabularData

private struct TDJSONDateParseStrategyPayload: Codable {
    var kind: String
    var value: String?
}

private struct TDJSONDateWriteStrategyPayload: Codable {
    var kind: String
    var value: String?
}

private enum TDJSONTypePayload: String, Codable {
    case integer
    case boolean
    case double
    case date
    case string
    case array
    case object
}

private struct TDJSONReadingOptionsPayload: Codable {
    var date_parse_strategies: [TDJSONDateParseStrategyPayload]
}

private struct TDJSONReadRequestPayload: Codable {
    var columns: [String]?
    var types: [String: TDJSONTypePayload]
    var options: TDJSONReadingOptionsPayload
}

private struct TDJSONWritingOptionsPayload: Codable {
    var sort_keys: Bool
    var pretty_print: Bool
    var date_strategy: TDJSONDateWriteStrategyPayload?
}

private func td_json_type(_ type: TDJSONTypePayload) -> JSONType {
    switch type {
    case .integer:
        return .integer
    case .boolean:
        return .boolean
    case .double:
        return .double
    case .date:
        return .date
    case .string:
        return .string
    case .array:
        return .array
    case .object:
        return .object
    }
}

private func td_json_date_parser(_ payload: TDJSONDateParseStrategyPayload) -> (String) -> Date? {
    switch payload.kind {
    case "iso8601":
        let formatter = ISO8601DateFormatter()
        return { formatter.date(from: $0) }
    case "rfc3339":
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        let fallback = ISO8601DateFormatter()
        fallback.formatOptions = [.withInternetDateTime]
        return { formatter.date(from: $0) ?? fallback.date(from: $0) }
    case "ymd":
        let formatter = DateFormatter()
        formatter.locale = Locale(identifier: "en_US_POSIX")
        formatter.dateFormat = "yyyy-MM-dd"
        return { formatter.date(from: $0) }
    case "custom_format":
        let formatter = DateFormatter()
        formatter.locale = Locale(identifier: "en_US_POSIX")
        formatter.dateFormat = payload.value ?? ""
        return { formatter.date(from: $0) }
    default:
        return { _ in nil }
    }
}

private func td_json_reading_options(_ payload: TDJSONReadingOptionsPayload) -> JSONReadingOptions {
    var options = JSONReadingOptions()
    options.dateParsers = payload.date_parse_strategies.map(td_json_date_parser)
    return options
}

@available(macOS 13.0, *)
private func td_json_writing_options(_ payload: TDJSONWritingOptionsPayload) -> JSONWritingOptions {
    var options = JSONWritingOptions()
    options.sortKeys = payload.sort_keys
    options.prettyPrint = payload.pretty_print

    if let strategy = payload.date_strategy {
        switch strategy.kind {
        case "iso8601":
            let formatter = ISO8601DateFormatter()
            options.dateFormatter = { formatter.string(from: $0) }
        case "custom_format":
            let formatter = DateFormatter()
            formatter.locale = Locale(identifier: "en_US_POSIX")
            formatter.dateFormat = strategy.value ?? ""
            options.dateFormatter = { formatter.string(from: $0) }
        default:
            break
        }
    }

    return options
}

@_cdecl("td_dataframe_from_json_file")
public func td_dataframe_from_json_file(
    _ path: UnsafePointer<CChar>?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    guard let path else {
        td_write_error(errorOut, "path must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let request = try td_decode_json(requestJSON, as: TDJSONReadRequestPayload.self)
        let frame = try DataFrame(
            contentsOfJSONFile: URL(fileURLWithPath: String(cString: path)),
            columns: request.columns,
            types: request.types.mapValues(td_json_type),
            options: td_json_reading_options(request.options)
        )
        outFrame.pointee = td_retain(TDDataFrameBox(frame: frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_from_json_data")
public func td_dataframe_from_json_data(
    _ jsonData: UnsafePointer<CChar>?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    guard let jsonData else {
        td_write_error(errorOut, "JSON data must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let request = try td_decode_json(requestJSON, as: TDJSONReadRequestPayload.self)
        let frame = try DataFrame(
            jsonData: Data(String(cString: jsonData).utf8),
            columns: request.columns,
            types: request.types.mapValues(td_json_type),
            options: td_json_reading_options(request.options)
        )
        outFrame.pointee = td_retain(TDDataFrameBox(frame: frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_write_json")
public func td_dataframe_write_json(
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
        let payload = try td_decode_json(optionsJSON, as: TDJSONWritingOptionsPayload.self)
        guard #available(macOS 13.0, *) else {
            throw td_invalid_argument("JSON writing requires macOS 13 or newer")
        }
        try frame.writeJSON(
            to: URL(fileURLWithPath: String(cString: path)),
            options: td_json_writing_options(payload)
        )
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_json_data_json")
public func td_dataframe_json_data_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }

    do {
        let payload = try td_decode_json(optionsJSON, as: TDJSONWritingOptionsPayload.self)
        guard #available(macOS 13.0, *) else {
            throw td_invalid_argument("JSON writing requires macOS 13 or newer")
        }
        let data = try frame.jsonRepresentation(options: td_json_writing_options(payload))
        return td_string(td_codable_json_string(Array(data)))
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return nil
    }
}
