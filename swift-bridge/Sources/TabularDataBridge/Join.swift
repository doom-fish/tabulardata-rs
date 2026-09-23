import Foundation
import TabularData

private struct TDJoinColumnsPayload: Codable {
    var left: String
    var right: String
}

private struct TDJoinRequestPayload: Codable {
    var columns: TDJoinColumnsPayload
    var kind: String
}

private func td_join_kind(_ raw: String) -> JoinKind {
    switch raw {
    case "left":
        return .left
    case "right":
        return .right
    case "full":
        return .full
    default:
        return .inner
    }
}

@_cdecl("td_dataframe_join_json")
public func td_dataframe_join_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ otherPtr: UnsafeMutableRawPointer?,
    _ joinJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let frame = td_box(framePtr)?.frame, let other = td_box(otherPtr)?.frame else {
        td_write_error(errorOut, "data frames must not be null")
        outFrame.pointee = nil
        return TDR_INVALID_ARGUMENT
    }

    do {
        let request = try td_decode_json(joinJSON, as: TDJoinRequestPayload.self)
        let leftType = try td_column_type(request.columns.left, in: frame, label: "the left frame")
        let rightType = try td_column_type(request.columns.right, in: other, label: "the right frame")
        guard leftType == rightType else {
            throw td_invalid_argument(
                "join columns must have the same type: '\(request.columns.left)' holds \(td_type_label(leftType)) values and '\(request.columns.right)' holds \(td_type_label(rightType)) values"
            )
        }
        guard td_is_scalar_type(leftType) else {
            throw td_invalid_argument(
                "join columns must hold scalar values; '\(request.columns.left)' holds \(td_type_label(leftType)) values"
            )
        }
        let joined: DataFrame
        if request.columns.left == request.columns.right {
            joined = frame.joined(other, on: request.columns.left, kind: td_join_kind(request.kind))
        } else {
            joined = frame.joined(
                other,
                on: (left: request.columns.left, right: request.columns.right),
                kind: td_join_kind(request.kind)
            )
        }
        outFrame.pointee = td_retain(TDDataFrameBox(frame: joined))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        outFrame.pointee = nil
        return td_status(for: error)
    }
}
