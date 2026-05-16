import Foundation
import TabularData

private func td_make_empty_column(name: String, sample: TDAnyValue) throws -> AnyColumn {
    switch sample {
    case .null, .string:
        return Column<String>(name: name, capacity: 0).eraseToAnyColumn()
    case .int:
        return Column<Int>(name: name, capacity: 0).eraseToAnyColumn()
    case .double:
        return Column<Double>(name: name, capacity: 0).eraseToAnyColumn()
    case .bool:
        return Column<Bool>(name: name, capacity: 0).eraseToAnyColumn()
    case .date:
        return Column<Date>(name: name, capacity: 0).eraseToAnyColumn()
    case .data:
        return Column<Data>(name: name, capacity: 0).eraseToAnyColumn()
    case .array, .object:
        throw td_invalid_argument("from_rows supports scalar, date, and data cell values only")
    }
}

private func td_frame_from_payload_rows(_ rows: [TDAnyRowPayload]) throws -> DataFrame {
    guard !rows.isEmpty else {
        return DataFrame()
    }

    let columnNames = Array(Set(rows.flatMap { $0.values.keys })).sorted()
    let columns = try columnNames.map { columnName -> AnyColumn in
        let sample = rows.compactMap { $0.values[columnName] }.first(where: { $0 != .null }) ?? .null
        return try td_make_empty_column(name: columnName, sample: sample)
    }

    var frame = DataFrame(columns: columns)
    for row in rows {
        frame.append(valuesByColumn: td_row_dictionary(row))
    }
    return frame
}

@_cdecl("td_dataframe_from_rows_json")
public func td_dataframe_from_rows_json(
    _ rowsJSON: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    do {
        let payload = try td_decode_json(rowsJSON, as: [TDAnyRowPayload].self)
        let frame = try td_frame_from_payload_rows(payload)
        outFrame.pointee = td_retain(TDDataFrameBox(frame: frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_row_json")
public func td_dataframe_row_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }
    guard box.frame.rows.indices.contains(index) else {
        td_write_error(errorOut, "row index out of bounds")
        return nil
    }
    return td_string(td_codable_json_string(td_row_payload(box.frame.rows[index])))
}

@_cdecl("td_dataframe_any_rows_json")
public func td_dataframe_any_rows_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return nil
    }
    let rows = box.frame.rows.map(td_row_payload)
    return td_string(td_codable_json_string(rows))
}

@_cdecl("td_dataframe_append_row_json")
public func td_dataframe_append_row_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ rowJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let row = try td_decode_json(rowJSON, as: TDAnyRowPayload.self)
        box.frame.append(valuesByColumn: td_row_dictionary(row))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_insert_row_json")
public func td_dataframe_insert_row_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ rowJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        guard index >= 0, index <= box.frame.rows.count else {
            throw td_invalid_argument("row index out of bounds")
        }
        let row = try td_decode_json(rowJSON, as: TDAnyRowPayload.self)
        let columnNames = box.frame.columns.map(\.name)
        var rows = box.frame.rows.map { td_row_dictionary($0, columnNames: columnNames) }
        rows.insert(td_row_dictionary(row), at: index)
        box.frame = td_frame(from: rows, like: box.frame)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}

@_cdecl("td_dataframe_replace_row_json")
public func td_dataframe_replace_row_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ rowJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        guard box.frame.rows.indices.contains(index) else {
            throw td_invalid_argument("row index out of bounds")
        }
        let row = try td_decode_json(rowJSON, as: TDAnyRowPayload.self)
        let columnNames = box.frame.columns.map(\.name)
        var rows = box.frame.rows.map { td_row_dictionary($0, columnNames: columnNames) }
        rows[index] = td_row_dictionary(row)
        box.frame = td_frame(from: rows, like: box.frame)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return TDR_FRAMEWORK_ERROR
    }
}
