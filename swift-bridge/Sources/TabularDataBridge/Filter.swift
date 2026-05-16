import Foundation
import TabularData

private final class TDFilterBox: Codable {
    var filter: TDFilterPayload

    init(filter: TDFilterPayload) {
        self.filter = filter
    }
}

private struct TDFilterPayload: Codable {
    var kind: String
    var column: String?
    var op: String?
    var value: TDAnyValue?
    var lower: TDAnyValue?
    var upper: TDAnyValue?
    var values: [TDAnyValue]?
    var filters: [TDFilterPayload]?
    var filter: TDFilterBox?
}

private func td_matches(_ row: DataFrame.Row, filter: TDFilterPayload) throws -> Bool {
    switch filter.kind {
    case "compare":
        guard let column = filter.column, let op = filter.op, let value = filter.value else {
            throw td_invalid_argument("compare filters require column, op, and value")
        }
        let lhs = TDAnyValue.fromFoundation(row[column])
        switch op {
        case "eq":
            return td_any_value_equal(lhs, value)
        case "ne":
            return !td_any_value_equal(lhs, value)
        case "lt":
            return td_any_value_compare(lhs, value) == .orderedAscending
        case "lte":
            let comparison = td_any_value_compare(lhs, value)
            return comparison == .orderedAscending || comparison == .orderedSame
        case "gt":
            return td_any_value_compare(lhs, value) == .orderedDescending
        case "gte":
            let comparison = td_any_value_compare(lhs, value)
            return comparison == .orderedDescending || comparison == .orderedSame
        default:
            throw td_invalid_argument("unsupported comparison operator '\(op)'")
        }
    case "between":
        guard let column = filter.column, let lower = filter.lower, let upper = filter.upper else {
            throw td_invalid_argument("between filters require column, lower, and upper")
        }
        let lhs = TDAnyValue.fromFoundation(row[column])
        let lowerComparison = td_any_value_compare(lhs, lower)
        let upperComparison = td_any_value_compare(lhs, upper)
        return (lowerComparison == .orderedDescending || lowerComparison == .orderedSame)
            && (upperComparison == .orderedAscending || upperComparison == .orderedSame)
    case "in":
        guard let column = filter.column, let values = filter.values else {
            throw td_invalid_argument("in filters require column and values")
        }
        let lhs = TDAnyValue.fromFoundation(row[column])
        return values.contains { td_any_value_equal(lhs, $0) }
    case "contains":
        guard let column = filter.column, let value = filter.value else {
            throw td_invalid_argument("contains filters require column and value")
        }
        return td_any_value_contains(TDAnyValue.fromFoundation(row[column]), value)
    case "is_null":
        guard let column = filter.column else {
            throw td_invalid_argument("is_null filters require a column")
        }
        return TDAnyValue.fromFoundation(row[column]) == .null
    case "is_not_null":
        guard let column = filter.column else {
            throw td_invalid_argument("is_not_null filters require a column")
        }
        return TDAnyValue.fromFoundation(row[column]) != .null
    case "and":
        guard let filters = filter.filters else {
            throw td_invalid_argument("and filters require nested filters")
        }
        return try filters.allSatisfy { try td_matches(row, filter: $0) }
    case "or":
        guard let filters = filter.filters else {
            throw td_invalid_argument("or filters require nested filters")
        }
        return try filters.contains { try td_matches(row, filter: $0) }
    case "not":
        guard let nested = filter.filter else {
            throw td_invalid_argument("not filters require a nested filter")
        }
        return try !td_matches(row, filter: nested.filter)
    default:
        throw td_invalid_argument("unsupported filter kind '\(filter.kind)'")
    }
}

@_cdecl("td_dataframe_filter_json")
public func td_dataframe_filter_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ filterJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let filter = try td_decode_json(filterJSON, as: TDFilterPayload.self)
        let slice = try box.frame.filter { try td_matches($0, filter: filter) }
        outFrame.pointee = td_retain(TDDataFrameBox(frame: DataFrame(slice)))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return TDR_FRAMEWORK_ERROR
    }
}
