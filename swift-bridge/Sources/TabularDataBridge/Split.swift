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
        let split = frame.randomSplit(by: payload.proportion, seed: payload.seed)
        outLeft.pointee = td_retain(TDDataFrameBox(frame: DataFrame(split.0)))
        outRight.pointee = td_retain(TDDataFrameBox(frame: DataFrame(split.1)))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outLeft.pointee = nil
        outRight.pointee = nil
        return TDR_FRAMEWORK_ERROR
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
        let split: (DataFrame, DataFrame)
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
        outLeft.pointee = td_retain(TDDataFrameBox(frame: split.0))
        outRight.pointee = td_retain(TDDataFrameBox(frame: split.1))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outLeft.pointee = nil
        outRight.pointee = nil
        return TDR_FRAMEWORK_ERROR
    }
}
