import Foundation
import TabularData

func td_inferred_column(name: String, values: [TDAnyValue]) throws -> AnyColumn {
    var kinds = Set(values.map(\.kindName))
    kinds.remove("null")
    if kinds == ["int", "double"] {
        kinds = ["double"]
    }
    guard kinds.count <= 1 else {
        throw td_invalid_argument(
            "column '\(name)' mixes \(kinds.sorted().joined(separator: " and ")) values"
        )
    }
    switch kinds.first ?? "string" {
    case "string":
        return Column<String>(name: name, capacity: 0).eraseToAnyColumn()
    case "int":
        return Column<Int>(name: name, capacity: 0).eraseToAnyColumn()
    case "double":
        return Column<Double>(name: name, capacity: 0).eraseToAnyColumn()
    case "bool":
        return Column<Bool>(name: name, capacity: 0).eraseToAnyColumn()
    case "date":
        return Column<Date>(name: name, capacity: 0).eraseToAnyColumn()
    case "data":
        return Column<Data>(name: name, capacity: 0).eraseToAnyColumn()
    case "array":
        return Column<[Any?]>(name: name, capacity: 0).eraseToAnyColumn()
    case "object":
        return Column<[String: Any?]>(name: name, capacity: 0).eraseToAnyColumn()
    default:
        throw td_invalid_argument("column '\(name)' holds values of an unsupported kind")
    }
}

private func td_frame_from_payload_rows(_ rows: [TDAnyRowPayload]) throws -> DataFrame {
    let columnNames = Array(Set(rows.flatMap { $0.values.keys })).sorted()
    guard !columnNames.isEmpty else {
        return DataFrame()
    }

    let columns = try columnNames.map { columnName in
        try td_inferred_column(name: columnName, values: rows.map { $0.values[columnName] ?? .null })
    }

    var frame = DataFrame(columns: columns)
    for row in rows {
        frame.append(valuesByColumn: try td_typed_row(row.values, for: frame))
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
        return td_status(for: error)
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
        box.frame.append(valuesByColumn: try td_typed_row(row.values, for: box.frame))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
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
        let single = try td_single_row_frame(row.values, like: box.frame)
        box.frame.insert(row: single.rows[0], at: index)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
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
        let single = try td_single_row_frame(row.values, like: box.frame)
        box.frame.removeRow(at: index)
        box.frame.insert(row: single.rows[0], at: index)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_append_empty_row")
public func td_dataframe_append_empty_row(
    _ framePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    box.frame.appendEmptyRow()
    _ = errorOut
    return TDR_OK
}

@_cdecl("td_dataframe_remove_row")
public func td_dataframe_remove_row(
    _ framePtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    guard box.frame.rows.indices.contains(index) else {
        td_write_error(errorOut, "row index out of bounds")
        return TDR_INVALID_ARGUMENT
    }

    box.frame.removeRow(at: index)
    return TDR_OK
}
