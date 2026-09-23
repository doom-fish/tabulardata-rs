import Foundation
import TabularData

private struct TDGroupBySpecPayload: Codable {
    var columns: [String]
    var time_unit: String?
}

private struct TDGroupAggregationPayload: Codable {
    var kind: String
    var column: String?
    var value_type: String?
    var order: String?
    var quantile: Double?
}

private func td_calendar_component(_ raw: String) throws -> Calendar.Component {
    switch raw {
    case "year":
        return .year
    case "month":
        return .month
    case "day":
        return .day
    case "hour":
        return .hour
    case "minute":
        return .minute
    case "second":
        return .second
    case "week_of_year":
        return .weekOfYear
    default:
        throw td_invalid_argument("unsupported time unit '\(raw)'")
    }
}

private func td_validate_grouping(frame: DataFrame, spec: TDGroupBySpecPayload) throws {
    guard (1...3).contains(spec.columns.count) else {
        throw td_invalid_argument("grouping supports between one and three columns")
    }
    try td_require_columns(spec.columns, in: frame)
    try td_require_unique_columns(spec.columns, context: "the grouping")
    if let timeUnit = spec.time_unit {
        _ = try td_calendar_component(timeUnit)
        guard spec.columns.count == 1 else {
            throw td_invalid_argument("time-based grouping requires exactly one column")
        }
        let type = try td_column_type(spec.columns[0], in: frame)
        guard type == Date.self else {
            throw td_invalid_argument(
                "time-based grouping needs a Date column; '\(spec.columns[0])' holds \(td_type_label(type)) values"
            )
        }
    } else {
        try td_require_scalar_columns(spec.columns, in: frame, purpose: "grouping")
    }
}

private func td_grouping(
    frame: DataFrame,
    spec: TDGroupBySpecPayload
) throws -> any RowGroupingProtocol {
    try td_validate_grouping(frame: frame, spec: spec)
    if let timeUnit = spec.time_unit {
        return frame.grouped(by: spec.columns[0], timeUnit: try td_calendar_component(timeUnit))
    }

    switch spec.columns.count {
    case 1:
        return frame.grouped(by: spec.columns[0])
    case 2:
        return frame.grouped(by: spec.columns[0], spec.columns[1])
    default:
        return frame.grouped(by: spec.columns[0], spec.columns[1], spec.columns[2])
    }
}

private func td_value_type_matches(_ hint: String?, _ type: Any.Type) -> Bool {
    switch hint {
    case nil:
        return true
    case "int", "double":
        return td_is_integer_type(type) || td_is_floating_type(type)
    case "string":
        return type == String.self
    case "bool":
        return type == Bool.self
    case "date":
        return type == Date.self
    default:
        return false
    }
}

private func td_floating_source(
    frame: DataFrame,
    column: String,
    type: Any.Type,
    kind: String
) throws -> DataFrame {
    if td_is_floating_type(type) {
        return frame
    }
    guard type == Int.self else {
        throw td_invalid_argument(
            "\(kind) aggregates need an Int, Double, or Float column; '\(column)' holds \(td_type_label(type)) values"
        )
    }
    var converted = frame
    converted.transformColumn(column) { (value: Int) -> Double? in Double(value) }
    return converted
}

private func td_group_aggregate(
    frame: DataFrame,
    spec: TDGroupBySpecPayload,
    aggregate: TDGroupAggregationPayload
) throws -> DataFrame {
    let order = aggregate.order.map { td_sort_order($0) }
    if aggregate.kind == "counts" {
        return try td_grouping(frame: frame, spec: spec).counts(order: order)
    }
    guard ["sum", "mean", "quantile", "minimum", "maximum"].contains(aggregate.kind) else {
        throw td_invalid_argument("unsupported group aggregate '\(aggregate.kind)'")
    }
    guard let column = aggregate.column else {
        throw td_invalid_argument("\(aggregate.kind) aggregates require a column")
    }
    let type = try td_column_type(column, in: frame)
    guard td_value_type_matches(aggregate.value_type, type) else {
        throw td_invalid_argument(
            "column '\(column)' holds \(td_type_label(type)) values, not \(aggregate.value_type ?? "")"
        )
    }

    switch aggregate.kind {
    case "sum":
        guard aggregate.value_type != nil else {
            throw td_invalid_argument("sum aggregates require column and value_type")
        }
        let grouping = try td_grouping(frame: frame, spec: spec)
        if type == Int.self {
            return grouping.sums(column, Int.self, order: order)
        }
        if type == Double.self {
            return grouping.sums(column, Double.self, order: order)
        }
        if type == Float.self {
            return grouping.sums(column, Float.self, order: order)
        }
        throw td_invalid_argument(
            "sum aggregates need an Int, Double, or Float column; '\(column)' holds \(td_type_label(type)) values"
        )
    case "mean":
        let source = try td_floating_source(frame: frame, column: column, type: type, kind: "mean")
        let grouping = try td_grouping(frame: source, spec: spec)
        if type == Float.self {
            return grouping.means(column, Float.self, order: order)
        }
        return grouping.means(column, Double.self, order: order)
    case "quantile":
        guard let quantile = aggregate.quantile else {
            throw td_invalid_argument("quantile aggregates require column and quantile")
        }
        guard quantile >= 0, quantile <= 1 else {
            throw td_invalid_argument("quantile must be between 0 and 1")
        }
        let source = try td_floating_source(frame: frame, column: column, type: type, kind: "quantile")
        let grouping = try td_grouping(frame: source, spec: spec)
        if type == Float.self {
            return grouping.quantiles(column, Float.self, quantile: Float(quantile), order: order)
        }
        return grouping.quantiles(column, Double.self, quantile: quantile, order: order)
    default:
        guard aggregate.value_type != nil else {
            throw td_invalid_argument("\(aggregate.kind) aggregates require column and value_type")
        }
        let grouping = try td_grouping(frame: frame, spec: spec)
        let minimum = aggregate.kind == "minimum"
        if type == String.self {
            return minimum
                ? grouping.minimums(column, String.self, order: order)
                : grouping.maximums(column, String.self, order: order)
        }
        if type == Int.self {
            return minimum
                ? grouping.minimums(column, Int.self, order: order)
                : grouping.maximums(column, Int.self, order: order)
        }
        if type == Double.self {
            return minimum
                ? grouping.minimums(column, Double.self, order: order)
                : grouping.maximums(column, Double.self, order: order)
        }
        if type == Float.self {
            return minimum
                ? grouping.minimums(column, Float.self, order: order)
                : grouping.maximums(column, Float.self, order: order)
        }
        if type == Date.self {
            return minimum
                ? grouping.minimums(column, Date.self, order: order)
                : grouping.maximums(column, Date.self, order: order)
        }
        throw td_invalid_argument(
            "\(aggregate.kind) aggregates support String, Int, Double, Float, and Date columns; '\(column)' holds \(td_type_label(type)) values"
        )
    }
}

@_cdecl("td_dataframe_group_aggregate_json")
public func td_dataframe_group_aggregate_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ groupJSON: UnsafePointer<CChar>?,
    _ aggregateJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let spec = try td_decode_json(groupJSON, as: TDGroupBySpecPayload.self)
        let aggregate = try td_decode_json(aggregateJSON, as: TDGroupAggregationPayload.self)
        let result = try td_group_aggregate(frame: frame, spec: spec, aggregate: aggregate)
        outFrame.pointee = td_retain(TDDataFrameBox(frame: result))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_group_slice_json")
public func td_dataframe_group_slice_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ groupJSON: UnsafePointer<CChar>?,
    _ keysJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let spec = try td_decode_json(groupJSON, as: TDGroupBySpecPayload.self)
        let keys = try td_decode_json(keysJSON, as: [TDAnyValue].self)
        try td_validate_grouping(frame: frame, spec: spec)
        guard keys.count == spec.columns.count else {
            throw td_invalid_argument(
                "group lookup needs \(spec.columns.count) key(s), got \(keys.count)"
            )
        }
        let typedKeys: [Any?] = try zip(spec.columns, keys).map { column, key in
            let type = spec.time_unit == nil ? try td_column_type(column, in: frame) : Int.self
            return try td_cell_value(key, as: type, column: column)
        }
        let grouped = try td_grouping(frame: frame, spec: spec)
        let slice: DataFrame.Slice?
        switch typedKeys.count {
        case 1:
            slice = grouped[typedKeys[0]]
        case 2:
            slice = grouped[typedKeys[0], typedKeys[1]]
        default:
            slice = grouped[typedKeys[0], typedKeys[1], typedKeys[2]]
        }
        if let slice {
            outFrame.pointee = td_retain(TDDataFrameBox(frame: DataFrame(slice)))
        } else {
            outFrame.pointee = nil
        }
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return td_status(for: error)
    }
}
