import Foundation
import TabularData

private struct TDSortKeyPayload: Codable {
    var column: String
    var order: String
}

func td_sort_order(_ raw: String) -> Order {
    raw == "descending" ? .descending : .ascending
}

private func td_compare_rows(
    _ lhs: DataFrame.Row,
    _ rhs: DataFrame.Row,
    keys: [TDSortKeyPayload]
) -> ComparisonResult {
    for key in keys {
        let left = TDAnyValue.fromFoundation(lhs[key.column])
        let right = TDAnyValue.fromFoundation(rhs[key.column])
        if td_any_value_equal(left, right) {
            continue
        }
        let comparison = td_any_value_compare(left, right)
            ?? String(describing: lhs[key.column] ?? "").compare(String(describing: rhs[key.column] ?? ""))
        switch key.order {
        case "descending":
            switch comparison {
            case .orderedAscending:
                return .orderedDescending
            case .orderedDescending:
                return .orderedAscending
            case .orderedSame:
                continue
            @unknown default:
                continue
            }
        default:
            if comparison != .orderedSame {
                return comparison
            }
        }
    }
    return .orderedSame
}

@_cdecl("td_dataframe_sort_json")
public func td_dataframe_sort_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ sortJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let keys = try td_decode_json(sortJSON, as: [TDSortKeyPayload].self)
        guard !keys.isEmpty else {
            throw td_invalid_argument("at least one sort key is required")
        }
        let columnNames = box.frame.columns.map(\.name)
        let rows = Array(box.frame.rows.enumerated())
        let sorted = rows.sorted { lhs, rhs in
            let comparison = td_compare_rows(lhs.element, rhs.element, keys: keys)
            if comparison == .orderedSame {
                return lhs.offset < rhs.offset
            }
            return comparison == .orderedAscending
        }
        let rowDictionaries = sorted.map { td_row_dictionary($0.element, columnNames: columnNames) }
        outFrame.pointee = td_retain(TDDataFrameBox(frame: td_frame(from: rowDictionaries, like: box.frame)))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return TDR_FRAMEWORK_ERROR
    }
}
