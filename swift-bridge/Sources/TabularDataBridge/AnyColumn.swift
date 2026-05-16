import Foundation
import TabularData

@_cdecl("td_dataframe_any_column_json")
public func td_dataframe_any_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
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

    return td_string(td_codable_json_string(td_any_column_payload(box.frame[name])))
}
