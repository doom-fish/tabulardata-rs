import Foundation
import TabularData

@_cdecl("td_dataframe_any_column_json")
public func td_dataframe_any_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ outColumnJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outColumnJSON.pointee = nil
    guard let box = td_box(framePtr), let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let index = try td_column_index(String(cString: columnName), in: box.frame)
        outColumnJSON.pointee = td_string(td_codable_json_string(td_any_column_payload(box.frame.columns[index])))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}
