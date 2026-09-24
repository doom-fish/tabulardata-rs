import Foundation
import TabularData

@_cdecl("td_dataframe_column_slice_json")
public func td_dataframe_column_slice_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ start: UInt,
    _ end: UInt,
    _ outSliceJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outSliceJSON.pointee = nil
    guard let box = td_box(framePtr), let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let column = box.frame.columns[try td_column_index(String(cString: columnName), in: box.frame)]
        let lowerBound = min(Int(clamping: start), column.count)
        let upperBound = max(lowerBound, min(Int(clamping: end), column.count))
        let slice = column[lowerBound ..< upperBound]
        let indices = Array(lowerBound ..< upperBound)
        outSliceJSON.pointee = td_string(
            td_codable_json_string(td_column_slice_payload(slice, contiguous: true, indices: indices))
        )
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_column_mask_json")
public func td_dataframe_column_mask_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ maskJSON: UnsafePointer<CChar>?,
    _ outSliceJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outSliceJSON.pointee = nil
    guard let box = td_box(framePtr), let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let column = box.frame.columns[try td_column_index(String(cString: columnName), in: box.frame)]
        let mask = try td_decode_json(maskJSON, as: [Bool].self)
        guard mask.count == column.count else {
            throw td_invalid_argument("mask length \(mask.count) does not match column length \(column.count)")
        }
        let slice = column[mask]
        let indices = mask.enumerated().compactMap { $0.element ? $0.offset : nil }
        outSliceJSON.pointee = td_string(
            td_codable_json_string(td_column_slice_payload(slice, contiguous: false, indices: indices))
        )
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}
