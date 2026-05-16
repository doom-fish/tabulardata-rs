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

private func td_grouping(
    frame: DataFrame,
    spec: TDGroupBySpecPayload
) throws -> any RowGroupingProtocol {
    if let timeUnit = spec.time_unit {
        guard spec.columns.count == 1 else {
            throw td_invalid_argument("time-based grouping requires exactly one column")
        }
        return frame.grouped(by: spec.columns[0], timeUnit: try td_calendar_component(timeUnit))
    }

    switch spec.columns.count {
    case 1:
        return frame.grouped(by: spec.columns[0])
    case 2:
        return frame.grouped(by: spec.columns[0], spec.columns[1])
    case 3:
        return frame.grouped(by: spec.columns[0], spec.columns[1], spec.columns[2])
    default:
        throw td_invalid_argument("grouping supports between one and three columns")
    }
}

private func td_group_aggregate(
    _ grouping: any RowGroupingProtocol,
    aggregate: TDGroupAggregationPayload
) throws -> DataFrame {
    let order = aggregate.order.map { td_sort_order($0) }
    switch aggregate.kind {
    case "counts":
        return grouping.counts(order: order)
    case "sum":
        guard let column = aggregate.column, let valueType = aggregate.value_type else {
            throw td_invalid_argument("sum aggregates require column and value_type")
        }
        switch valueType {
        case "int":
            return grouping.sums(column, Int.self, order: order)
        case "double":
            return grouping.sums(column, Double.self, order: order)
        default:
            throw td_invalid_argument("sum aggregates support int and double columns")
        }
    case "mean":
        guard let column = aggregate.column else {
            throw td_invalid_argument("mean aggregates require a column")
        }
        return grouping.means(column, Double.self, order: order)
    case "quantile":
        guard let column = aggregate.column, let quantile = aggregate.quantile else {
            throw td_invalid_argument("quantile aggregates require column and quantile")
        }
        return grouping.quantiles(column, Double.self, quantile: quantile, order: order)
    case "minimum":
        guard let column = aggregate.column, let valueType = aggregate.value_type else {
            throw td_invalid_argument("minimum aggregates require column and value_type")
        }
        switch valueType {
        case "string":
            return grouping.minimums(column, String.self, order: order)
        case "int":
            return grouping.minimums(column, Int.self, order: order)
        case "double":
            return grouping.minimums(column, Double.self, order: order)
        case "date":
            return grouping.minimums(column, Date.self, order: order)
        default:
            throw td_invalid_argument("minimum aggregates support string, int, double, and date columns")
        }
    case "maximum":
        guard let column = aggregate.column, let valueType = aggregate.value_type else {
            throw td_invalid_argument("maximum aggregates require column and value_type")
        }
        switch valueType {
        case "string":
            return grouping.maximums(column, String.self, order: order)
        case "int":
            return grouping.maximums(column, Int.self, order: order)
        case "double":
            return grouping.maximums(column, Double.self, order: order)
        case "date":
            return grouping.maximums(column, Date.self, order: order)
        default:
            throw td_invalid_argument("maximum aggregates support string, int, double, and date columns")
        }
    default:
        throw td_invalid_argument("unsupported group aggregate '\(aggregate.kind)'")
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
        let grouped = try td_grouping(frame: frame, spec: spec)
        outFrame.pointee = td_retain(TDDataFrameBox(frame: try td_group_aggregate(grouped, aggregate: aggregate)))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return TDR_FRAMEWORK_ERROR
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
        let grouped = try td_grouping(frame: frame, spec: spec)
        let slice: DataFrame.Slice?
        switch keys.count {
        case 1:
            slice = grouped[keys[0].cellObject]
        case 2:
            slice = grouped[keys[0].cellObject, keys[1].cellObject]
        case 3:
            slice = grouped[keys[0].cellObject, keys[1].cellObject, keys[2].cellObject]
        default:
            throw td_invalid_argument("group lookup supports between one and three keys")
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
        return TDR_FRAMEWORK_ERROR
    }
}
