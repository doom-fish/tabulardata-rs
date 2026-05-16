import Foundation
import TabularData

@_cdecl("td_dataframe_slice_rows")
public func td_dataframe_slice_rows(
    _ framePtr: UnsafeMutableRawPointer?,
    _ start: Int,
    _ end: Int,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    let lowerBound = max(0, min(start, box.frame.rows.count))
    let upperBound = max(lowerBound, min(end, box.frame.rows.count))
    outFrame.pointee = td_retain(TDDataFrameBox(frame: DataFrame(box.frame[lowerBound ..< upperBound])))
    _ = errorOut
    return TDR_OK
}

@_cdecl("td_dataframe_prefix_rows")
public func td_dataframe_prefix_rows(
    _ framePtr: UnsafeMutableRawPointer?,
    _ length: Int,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }
    outFrame.pointee = td_retain(TDDataFrameBox(frame: DataFrame(frame.prefix(max(0, length)))))
    return TDR_OK
}

@_cdecl("td_dataframe_suffix_rows")
public func td_dataframe_suffix_rows(
    _ framePtr: UnsafeMutableRawPointer?,
    _ length: Int,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }
    outFrame.pointee = td_retain(TDDataFrameBox(frame: DataFrame(frame.suffix(max(0, length)))))
    return TDR_OK
}

@_cdecl("td_dataframe_select_columns_json")
public func td_dataframe_select_columns_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnsJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let columns = try td_decode_json(columnsJSON, as: [String].self)
        outFrame.pointee = td_retain(TDDataFrameBox(frame: box.frame.selecting(columnNames: columns)))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return TDR_FRAMEWORK_ERROR
    }
}
