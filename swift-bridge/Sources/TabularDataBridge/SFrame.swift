import Foundation
import TabularData

private struct TDSFrameReadRequestPayload: Codable {
    var columns: [String]?
    var rows: [Int]?
}

@_cdecl("td_dataframe_from_sframe_directory")
public func td_dataframe_from_sframe_directory(
    _ path: UnsafePointer<CChar>?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    guard let path else {
        td_write_error(errorOut, "path must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let request = try td_decode_json(requestJSON, as: TDSFrameReadRequestPayload.self)
        let rowRange: Range<Int>? = {
            guard let rows = request.rows else { return nil }
            guard rows.count == 2, rows[0] <= rows[1] else {
                return nil
            }
            return rows[0] ..< rows[1]
        }()
        if request.rows != nil && rowRange == nil {
            throw td_invalid_argument("rows must contain exactly two ascending bounds")
        }
        let frame = try DataFrame(
            contentsOfSFrameDirectory: URL(fileURLWithPath: String(cString: path)),
            columns: request.columns,
            rows: rowRange
        )
        outFrame.pointee = td_retain(TDDataFrameBox(frame: frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}
