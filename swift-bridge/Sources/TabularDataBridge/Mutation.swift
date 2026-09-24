import Foundation
import TabularData

func td_insert_column(_ payload: TDColumnPayload, at position: Int, into frame: inout DataFrame) throws {
    guard position <= frame.columns.count else {
        throw td_invalid_argument(
            "column index \(position) is out of bounds for a data frame with \(frame.columns.count) columns"
        )
    }
    try td_require_available_name(payload.name, in: frame)
    guard frame.columns.isEmpty || payload.values.count == frame.rows.count else {
        throw td_invalid_argument(
            "column '\(payload.name)' has \(payload.values.count) values but the data frame has \(frame.rows.count) rows"
        )
    }
    frame.insert(column: try td_make_any_column(payload), at: position)
}

private func td_element_column(name: String, elements: [Any?]) throws -> AnyColumn {
    let values = elements.map { TDAnyValue.fromFoundation($0) }
    var column = try td_inferred_column(name: name, values: values)
    for value in values {
        column.append(try td_cell_value(value, as: column.wrappedElementType, column: name))
    }
    return column
}

@_cdecl("td_dataframe_mask_rows")
public func td_dataframe_mask_rows(
    _ framePtr: UnsafeMutableRawPointer?,
    _ mask: UnsafePointer<Bool>?,
    _ length: UInt,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    guard let frame = td_box(framePtr)?.frame else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    let rowCount = frame.rows.count
    guard length == UInt(clamping: rowCount) else {
        td_write_error(errorOut, "row mask length \(length) does not match row count \(rowCount)")
        return TDR_INVALID_ARGUMENT
    }
    var flags: [Bool] = []
    if rowCount > 0 {
        guard let mask else {
            td_write_error(errorOut, "row mask must not be null")
            return TDR_INVALID_ARGUMENT
        }
        flags = Array(UnsafeBufferPointer(start: mask, count: rowCount))
    }
    outFrame.pointee = td_retain(TDDataFrameBox(frame: DataFrame(frame[flags])))
    return TDR_OK
}

@_cdecl("td_dataframe_insert_column_json")
public func td_dataframe_insert_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ index: UInt,
    _ columnJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr) else {
        td_write_error(errorOut, "data frame must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let payload = try td_decode_json(columnJSON, as: TDColumnPayload.self)
        try td_insert_column(payload, at: Int(clamping: index), into: &box.frame)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_replace_column_json")
public func td_dataframe_replace_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ columnJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr), let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let index = try td_column_index(String(cString: columnName), in: box.frame)
        let target = box.frame.columns[index].name
        let payload = try td_decode_json(columnJSON, as: TDColumnPayload.self)
        try td_require_available_name(payload.name, in: box.frame, replacing: index)
        guard payload.values.count == box.frame.rows.count else {
            throw td_invalid_argument(
                "column '\(payload.name)' has \(payload.values.count) values but the data frame has \(box.frame.rows.count) rows"
            )
        }
        box.frame.replaceColumn(target, with: try td_make_any_column(payload))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_remove_column_json")
public func td_dataframe_remove_column_json(
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
        let column = box.frame.columns[index]
        guard let payload = td_string(try td_codable_json(td_any_column_payload(column))) else {
            throw td_framework_error("could not allocate the removed column")
        }
        box.frame.removeColumn(column.name)
        outColumnJSON.pointee = payload
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_transform_column_json")
public func td_dataframe_transform_column_json(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ valuesJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = td_box(framePtr), let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let index = try td_column_index(String(cString: columnName), in: box.frame)
        let column = box.frame.columns[index]
        let values = try td_decode_json(valuesJSON, as: [TDAnyValue].self)
        guard values.count == column.count else {
            throw td_invalid_argument(
                "column '\(column.name)' has \(column.count) rows but the transform produced \(values.count) values"
            )
        }
        var replacement = column.prototype.makeColumn(capacity: values.count)
        for value in values {
            replacement.append(try td_cell_value(value, as: column.wrappedElementType, column: column.name))
        }
        box.frame.replaceColumn(column.name, with: replacement)
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}

@_cdecl("td_dataframe_exploding_column")
public func td_dataframe_exploding_column(
    _ framePtr: UnsafeMutableRawPointer?,
    _ columnName: UnsafePointer<CChar>?,
    _ outFrame: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outFrame.pointee = nil
    guard let frame = td_box(framePtr)?.frame, let columnName else {
        td_write_error(errorOut, "data frame and column name must not be null")
        return TDR_INVALID_ARGUMENT
    }

    do {
        let index = try td_column_index(String(cString: columnName), in: frame)
        let column = frame.columns[index]
        guard column.wrappedElementType == [Any?].self else {
            throw td_invalid_argument(
                "only array columns can be exploded; '\(column.name)' holds \(td_type_label(column.wrappedElementType)) values"
            )
        }
        let elements = column.assumingType([Any?].self).flatMap { $0 ?? [] }
        let elementColumn = try td_element_column(name: column.name, elements: elements)
        var exploded = frame.explodingColumn(column.name, [Any?].self)
        guard exploded.rows.count == elements.count else {
            throw td_framework_error(
                "exploding '\(column.name)' produced \(exploded.rows.count) rows for \(elements.count) elements"
            )
        }
        exploded.replaceColumn(column.name, with: elementColumn)
        var result = td_empty_frame(like: frame)
        result.replaceColumn(column.name, with: elementColumn.prototype.makeColumn(capacity: 0))
        let sameSchema = exploded.columns.count == result.columns.count
            && zip(exploded.columns, result.columns).allSatisfy {
                $0.name == $1.name && $0.wrappedElementType == $1.wrappedElementType
            }
        guard sameSchema else {
            throw td_framework_error("exploding '\(column.name)' changed the other columns of the data frame")
        }
        result.append(rowsOf: exploded)
        outFrame.pointee = td_retain(TDDataFrameBox(frame: result))
        return TDR_OK
    } catch {
        td_write_error(errorOut, error.localizedDescription)
        return td_status(for: error)
    }
}
