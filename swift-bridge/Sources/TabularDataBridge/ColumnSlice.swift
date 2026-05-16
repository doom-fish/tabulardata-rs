import Foundation
import TabularData

@_cdecl("td_dataframe_column_slice_json")
public func td_dataframe_column_slice_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ start: Int,
    _ end: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let box = td_box(framePtr), let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return nil
    }

    let name = String(cString: columnName)
    guard box.frame.indexOfColumn(name) != nil else {
        td_write_error(errorOut, "there is no column named '\(name)'")
        return nil
    }

    let column = box.frame[name]
    let lowerBound = max(0, min(start, column.count))
    let upperBound = max(lowerBound, min(end, column.count))
    let slice = column[lowerBound ..< upperBound]
    let indices = Array(lowerBound ..< upperBound)
    return td_string(td_codable_json_string(td_column_slice_payload(slice, contiguous: true, indices: indices)))
}

@_cdecl("td_dataframe_column_mask_json")
public func td_dataframe_column_mask_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ maskJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let box = td_box(framePtr), let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return nil
    }

    do {
        let name = String(cString: columnName)
        guard box.frame.indexOfColumn(name) != nil else {
            throw td_invalid_argument("there is no column named '\(name)'")
        }
        let column = box.frame[name]
        let mask = try td_decode_json(maskJSON, as: [Bool].self)
        guard mask.count == column.count else {
            throw td_invalid_argument("mask length must match column length")
        }
        let slice = column[mask]
        let indices = mask.enumerated().compactMap { $0.element ? $0.offset : nil }
        return td_string(td_codable_json_string(td_column_slice_payload(slice, contiguous: false, indices: indices)))
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return nil
    }
}
