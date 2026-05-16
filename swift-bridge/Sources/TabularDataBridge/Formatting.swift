import Foundation
import TabularData

private struct TDFormattingOptionsPayload: Codable {
    var maximum_line_width: Int?
    var maximum_cell_width: Int?
    var maximum_row_count: Int?
    var includes_column_types: Bool?
    var includes_row_indices: Bool?
    var includes_row_and_column_counts: Bool?
    var locale: String?
}

private func td_formatting_options(_ payload: TDFormattingOptionsPayload) throws -> FormattingOptions {
    var options = FormattingOptions()

    if let value = payload.maximum_line_width {
        options.maximumLineWidth = value
    }
    if let value = payload.maximum_cell_width {
        options.maximumCellWidth = value
    }
    if let value = payload.maximum_row_count {
        options.maximumRowCount = value
    }
    if let value = payload.includes_column_types {
        options.includesColumnTypes = value
    }
    if let value = payload.includes_row_indices {
        guard #available(macOS 14.0, *) else {
            throw td_invalid_argument("row index formatting requires macOS 14 or newer")
        }
        options.includesRowIndices = value
    }
    if let value = payload.includes_row_and_column_counts {
        guard #available(macOS 14.0, *) else {
            throw td_invalid_argument("row/column count formatting requires macOS 14 or newer")
        }
        options.includesRowAndColumnCounts = value
    }
    if let locale = payload.locale {
        guard #available(macOS 12.3, *) else {
            throw td_invalid_argument("locale-aware formatting requires macOS 12.3 or newer")
        }
        options.locale = Locale(identifier: locale)
    }

    return options
}

@_cdecl("td_dataframe_description")
public func td_dataframe_description(
    _ framePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }

    return td_string(td_codable_json_string(frame.description))
}

@_cdecl("td_dataframe_format_json")
public func td_dataframe_format_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }

    do {
        let payload = try td_decode_json(optionsJSON, as: TDFormattingOptionsPayload.self)
        return td_string(td_codable_json_string(frame.description(options: try td_formatting_options(payload))))
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return nil
    }
}
