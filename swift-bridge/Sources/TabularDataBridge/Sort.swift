import Foundation
import TabularData

private struct TDSortKeyPayload: Codable {
    var column: String
    var order: String
}

func td_sort_order(_ raw: String) -> Order {
    raw == "descending" ? .descending : .ascending
}

private struct TDSortColumn {
    let descending: Bool
    let values: [TDAnyValue]
    let column: AnyColumn
}

private func td_compare_rows(_ lhs: Int, _ rhs: Int, keys: [TDSortColumn]) -> ComparisonResult {
    for key in keys {
        let left = key.values[lhs]
        let right = key.values[rhs]
        if td_any_value_equal(left, right) {
            continue
        }
        let comparison = td_any_value_compare(left, right)
            ?? String(describing: key.column[lhs] ?? "").compare(String(describing: key.column[rhs] ?? ""))
        if comparison == .orderedSame {
            continue
        }
        if key.descending {
            return comparison == .orderedAscending ? .orderedDescending : .orderedAscending
        }
        return comparison
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
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let keys = try td_decode_json(sortJSON, as: [TDSortKeyPayload].self)
        guard !keys.isEmpty else {
            throw td_invalid_argument("at least one sort key is required")
        }
        try td_require_columns(keys.map(\.column), in: frame)
        let sortColumns = keys.map { key -> TDSortColumn in
            let column = frame[key.column]
            return TDSortColumn(
                descending: key.order == "descending",
                values: column.map { TDAnyValue.fromFoundation($0) },
                column: column
            )
        }
        let order = frame.rows.indices.sorted { lhs, rhs in
            let comparison = td_compare_rows(lhs, rhs, keys: sortColumns)
            if comparison == .orderedSame {
                return lhs < rhs
            }
            return comparison == .orderedAscending
        }
        var sorted = td_empty_frame(like: frame)
        for index in order {
            sorted.append(row: frame.rows[index])
        }
        outFrame.pointee = td_retain(TDDataFrameBox(frame: sorted))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return td_status(for: error)
    }
}
