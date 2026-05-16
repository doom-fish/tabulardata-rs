import Foundation
import TabularData

private struct TDDateWriteStrategyPayload: Codable {
    var kind: String
    var value: String?
}

private struct TDCSVWritingOptionsPayload: Codable {
    var includes_header: Bool
    var date_strategy: TDDateWriteStrategyPayload?
    var nil_encoding: String
    var true_encoding: String
    var false_encoding: String
    var newline: String
    var delimiter: String
}

private func td_csv_writing_options(_ payload: TDCSVWritingOptionsPayload) throws -> CSVWritingOptions {
    var options = CSVWritingOptions(
        includesHeader: payload.includes_header,
        nilEncoding: payload.nil_encoding,
        trueEncoding: payload.true_encoding,
        falseEncoding: payload.false_encoding,
        newline: payload.newline,
        delimiter: try td_character(payload.delimiter, fieldName: "delimiter")
    )

    if let strategy = payload.date_strategy {
        switch strategy.kind {
        case "iso8601":
            if #available(macOS 12.3, *) {
                let formatter = ISO8601DateFormatter()
                options.dateFormatter = { formatter.string(from: $0) }
            } else {
                options.dateFormat = "yyyy-MM-dd'T'HH:mm:ssXXXXX"
            }
        case "custom_format":
            if #available(macOS 12.3, *) {
                let formatter = DateFormatter()
                formatter.locale = Locale(identifier: "en_US_POSIX")
                formatter.dateFormat = strategy.value ?? ""
                options.dateFormatter = { formatter.string(from: $0) }
            } else {
                options.dateFormat = strategy.value
            }
        default:
            break
        }
    }

    return options
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

@_cdecl("td_dataframe_csv_string")
public func td_dataframe_csv_string(
    _ framePtr: UnsafeMutableRawPointer?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }

    do {
        let options = try td_decode_json(optionsJSON, as: TDCSVWritingOptionsPayload.self)
        let data = try frame.csvRepresentation(options: td_csv_writing_options(options))
        guard let string = String(data: data, encoding: .utf8) else {
            throw td_invalid_argument("CSV data was not valid UTF-8")
        }
        return td_string(td_codable_json_string(string))
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return nil
    }
}
