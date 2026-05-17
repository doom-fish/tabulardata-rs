import Foundation
import TabularData

private struct TDDateParseStrategyPayload: Codable {
    var kind: String
    var value: String?
}

private enum TDCSVType: String, Codable {
    case integer
    case boolean
    case float
    case double
    case date
    case string
    case data
}

private struct TDCSVReadingOptionsPayload: Codable {
    var has_header_row: Bool
    var nil_encodings: [String]
    var true_encodings: [String]
    var false_encodings: [String]
    var floating_point_type: TDCSVType
    var date_parse_strategies: [TDDateParseStrategyPayload]
    var ignores_empty_lines: Bool
    var uses_quoting: Bool
    var uses_escaping: Bool
    var delimiter: String
    var escape_character: String
}

private struct TDCSVReadRequestPayload: Codable {
    var columns: [String]?
    var rows: [Int]?
    var types: [String: TDCSVType]
    var options: TDCSVReadingOptionsPayload
}

private func td_csv_type(_ type: TDCSVType) -> CSVType {
    switch type {
    case .integer:
        return .integer
    case .boolean:
        return .boolean
    case .float:
        return .float
    case .double:
        return .double
    case .date:
        return .date
    case .string:
        return .string
    case .data:
        return .data
    }
}

private func td_date_parser(_ payload: TDDateParseStrategyPayload) -> (String) -> Date? {
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

private func td_csv_reading_options(_ payload: TDCSVReadingOptionsPayload) throws -> CSVReadingOptions {
    var options = CSVReadingOptions(
        hasHeaderRow: payload.has_header_row,
        nilEncodings: Set(payload.nil_encodings),
        trueEncodings: Set(payload.true_encodings),
        falseEncodings: Set(payload.false_encodings),
        floatingPointType: td_csv_type(payload.floating_point_type),
        ignoresEmptyLines: payload.ignores_empty_lines,
        usesQuoting: payload.uses_quoting,
        usesEscaping: payload.uses_escaping,
        delimiter: try td_character(payload.delimiter, fieldName: "delimiter"),
        escapeCharacter: try td_character(payload.escape_character, fieldName: "escape_character")
    )
    options.dateParsers = payload.date_parse_strategies.map(td_date_parser)
    return options
}

@_cdecl("td_dataframe_from_csv")
public func td_dataframe_from_csv(
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
        let request = try td_decode_json(requestJSON, as: TDCSVReadRequestPayload.self)
        let rowRange: Range<Int>? = {
            guard let rows = request.rows else { return nil }
            guard rows.count == 2, rows[0] <= rows[1] else {
                return nil
            }
            return rows[0] ..< rows[1]
        }()
        if request.rows != nil && rowRange == nil {
            throw td_invalid_argument("rows must contain exactly two ascending bounds")
        }
        let frame = try DataFrame(
            contentsOfCSVFile: URL(fileURLWithPath: String(cString: path)),
            columns: request.columns,
            rows: rowRange,
            types: request.types.mapValues(td_csv_type),
            options: td_csv_reading_options(request.options)
        )
        outFrame.pointee = td_retain(TDDataFrameBox(frame: frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_from_csv_data")
public func td_dataframe_from_csv_data(
    _ data: UnsafePointer<CChar>?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    guard let data else {
        td_write_error(errorOut, "CSV data must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let request = try td_decode_json(requestJSON, as: TDCSVReadRequestPayload.self)
        let rowRange: Range<Int>? = {
            guard let rows = request.rows else { return nil }
            guard rows.count == 2, rows[0] <= rows[1] else {
                return nil
            }
            return rows[0] ..< rows[1]
        }()
        if request.rows != nil && rowRange == nil {
            throw td_invalid_argument("rows must contain exactly two ascending bounds")
        }
        let frame = try DataFrame(
            csvData: Data(String(cString: data).utf8),
            columns: request.columns,
            rows: rowRange,
            types: request.types.mapValues(td_csv_type),
            options: td_csv_reading_options(request.options)
        )
        outFrame.pointee = td_retain(TDDataFrameBox(frame: frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}
