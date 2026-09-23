import Foundation
import TabularData

private struct TDRandomSplitPayload: Codable {
    var proportion: Double
    var seed: Int?
}

private struct TDStratifiedSplitPayload: Codable {
    var columns: [String]
    var proportion: Double
    var random_seed: Int?
}

private func td_validate_split_proportion(_ proportion: Double) throws {
    guard proportion >= 0.0, proportion <= 1.0 else {
        throw td_invalid_argument("split proportion must be between 0 and 1")
    }
}

private func td_edge_split(_ frame: DataFrame, proportion: Double) -> (DataFrame, DataFrame)? {
    if proportion == 0 {
        return (td_empty_frame(like: frame), frame)
    }
    if proportion == 1 {
        return (frame, td_empty_frame(like: frame))
    }
    return nil
}

@_cdecl("td_dataframe_random_split")
public func td_dataframe_random_split(
    _ framePtr: UnsafeMutableRawPointer?,
    _ splitJSON: UnsafePointer<CChar>?,
    _ outLeft: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outRight: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outLeft.pointee = nil
        outRight.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let payload = try td_decode_json(splitJSON, as: TDRandomSplitPayload.self)
        try td_validate_split_proportion(payload.proportion)
        let split: (DataFrame, DataFrame)
        if let edge = td_edge_split(frame, proportion: payload.proportion) {
            split = edge
        } else {
            let slices = frame.randomSplit(by: payload.proportion, seed: payload.seed)
            split = (DataFrame(slices.0), DataFrame(slices.1))
        }
        outLeft.pointee = td_retain(TDDataFrameBox(frame: split.0))
        outRight.pointee = td_retain(TDDataFrameBox(frame: split.1))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outLeft.pointee = nil
        outRight.pointee = nil
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_stratified_split_json")
public func td_dataframe_stratified_split_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ splitJSON: UnsafePointer<CChar>?,
    _ outLeft: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outRight: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outLeft.pointee = nil
        outRight.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let payload = try td_decode_json(splitJSON, as: TDStratifiedSplitPayload.self)
        try td_validate_split_proportion(payload.proportion)
        guard (1...3).contains(payload.columns.count) else {
            throw td_invalid_argument("stratified split supports between one and three columns")
        }
        try td_require_columns(payload.columns, in: frame)
        try td_require_unique_columns(payload.columns, context: "the stratified split")
        try td_require_scalar_columns(payload.columns, in: frame, purpose: "a stratified split")
        let split: (DataFrame, DataFrame)
        if let edge = td_edge_split(frame, proportion: payload.proportion) {
            split = edge
        } else {
            switch payload.columns.count {
            case 1:
                split = frame.stratifiedSplit(
                    on: payload.columns[0],
                    by: payload.proportion,
                    randomSeed: payload.random_seed
                )
            case 2:
                split = frame.stratifiedSplit(
                    on: payload.columns[0],
                    payload.columns[1],
                    by: payload.proportion,
                    randomSeed: payload.random_seed
                )
            case 3:
                split = frame.stratifiedSplit(
                    on: payload.columns[0],
                    payload.columns[1],
                    payload.columns[2],
                    by: payload.proportion,
                    randomSeed: payload.random_seed
                )
            default:
                throw td_invalid_argument("stratified split supports between one and three columns")
            }
        }
        outLeft.pointee = td_retain(TDDataFrameBox(frame: split.0))
        outRight.pointee = td_retain(TDDataFrameBox(frame: split.1))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outLeft.pointee = nil
        outRight.pointee = nil
        return td_status(for: error)
    }
}
