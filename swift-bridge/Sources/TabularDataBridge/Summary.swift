import Foundation
import TabularData

@_cdecl("td_dataframe_summary")
public func td_dataframe_summary(
    _ framePtr: UnsafeMutableRawPointer?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    outFrame.pointee = td_retain(TDDataFrameBox(frame: frame.summary()))
    return TDR_OK
}

@_cdecl("td_dataframe_summary_columns")
public func td_dataframe_summary_columns(
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
        let summary = box.frame.selecting(columnNames: columns).summary()
        outFrame.pointee = td_retain(TDDataFrameBox(frame: summary))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_summary_indices")
public func td_dataframe_summary_indices(
    _ framePtr: UnsafeMutableRawPointer?,
    _ indicesJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let indices = try td_decode_json(indicesJSON, as: [Int].self)
        let names = try indices.map { index -> String in
            guard box.frame.columns.indices.contains(index) else {
                throw td_invalid_argument("summary column index out of bounds")
            }
            return box.frame.columns[index].name
        }
        let summary = box.frame.selecting(columnNames: names).summary()
        outFrame.pointee = td_retain(TDDataFrameBox(frame: summary))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return TDR_FRAMEWORK_ERROR
    }
}
