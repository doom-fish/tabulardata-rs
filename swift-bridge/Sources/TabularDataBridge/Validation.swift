import Foundation
import TabularData

func td_require_columns<S: Sequence>(
    _ names: S,
    in frame: DataFrame,
    label: String = "the data frame"
) throws where S.Element == String {
    var missing: [String] = []
    for name in names where frame.indexOfColumn(name) == nil && !missing.contains(name) {
        missing.append(name)
    }
    guard missing.isEmpty else {
        let list = missing.map { "'\($0)'" }.joined(separator: ", ")
        let noun = missing.count == 1 ? "column named" : "columns named"
        throw td_invalid_argument("\(label) has no \(noun) \(list)")
    }
}

func td_require_unique_columns(_ names: [String], context: String) throws {
    var seen = Set<String>()
    for name in names where !seen.insert(name).inserted {
        throw td_invalid_argument("\(context) lists column '\(name)' more than once")
    }
}

func td_column_type(
    _ name: String,
    in frame: DataFrame,
    label: String = "the data frame"
) throws -> Any.Type {
    guard let index = frame.indexOfColumn(name) else {
        throw td_invalid_argument("\(label) has no column named '\(name)'")
    }
    return frame.columns[index].wrappedElementType
}

func td_type_label(_ type: Any.Type) -> String {
    String(describing: type)
}

func td_is_integer_type(_ type: Any.Type) -> Bool {
    type == Int.self || type == Int64.self || type == Int32.self || type == Int16.self
        || type == Int8.self || type == UInt.self || type == UInt64.self || type == UInt32.self
        || type == UInt16.self || type == UInt8.self
}

func td_is_floating_type(_ type: Any.Type) -> Bool {
    type == Double.self || type == Float.self
}

func td_is_scalar_type(_ type: Any.Type) -> Bool {
    td_is_integer_type(type) || td_is_floating_type(type) || type == String.self
        || type == Bool.self || type == Date.self || type == Data.self
}

func td_require_scalar_columns(_ names: [String], in frame: DataFrame, purpose: String) throws {
    for name in names {
        let type = try td_column_type(name, in: frame)
        guard td_is_scalar_type(type) else {
            throw td_invalid_argument(
                "\(purpose) needs columns of scalar values; '\(name)' holds \(td_type_label(type)) values"
            )
        }
    }
}

private func td_integer(_ number: Int64, as type: Any.Type) -> Any? {
    if type == Int.self { return Int(exactly: number) }
    if type == Int64.self { return number }
    if type == Int32.self { return Int32(exactly: number) }
    if type == Int16.self { return Int16(exactly: number) }
    if type == Int8.self { return Int8(exactly: number) }
    if type == UInt.self { return UInt(exactly: number) }
    if type == UInt64.self { return UInt64(exactly: number) }
    if type == UInt32.self { return UInt32(exactly: number) }
    if type == UInt16.self { return UInt16(exactly: number) }
    if type == UInt8.self { return UInt8(exactly: number) }
    return nil
}

func td_cell_value(_ value: TDAnyValue, as type: Any.Type, column: String) throws -> Any? {
    if case .null = value {
        return nil
    }
    let mismatch = td_invalid_argument(
        "column '\(column)' holds \(td_type_label(type)) values and cannot store a \(value.kindName) value"
    )
    switch value {
    case let .string(text) where type == String.self:
        return text
    case let .bool(flag) where type == Bool.self:
        return flag
    case let .int(number):
        if let integer = td_integer(number, as: type) {
            return integer
        }
        if type == Double.self {
            return Double(number)
        }
        if type == Float.self {
            return Float(number)
        }
        if type == Date.self {
            return Date(timeIntervalSince1970: Double(number))
        }
        throw mismatch
    case let .double(number):
        if type == Double.self {
            return number
        }
        if type == Float.self {
            return Float(number)
        }
        if type == Date.self {
            return Date(timeIntervalSince1970: number)
        }
        if let exact = Int64(exactly: number), let integer = td_integer(exact, as: type) {
            return integer
        }
        throw mismatch
    case let .date(seconds) where type == Date.self:
        return Date(timeIntervalSince1970: seconds)
    case let .data(encoded) where type == Data.self:
        guard let data = Data(base64Encoded: encoded) else {
            throw td_invalid_argument("column '\(column)' received data that is not valid base64")
        }
        return data
    case let .array(values) where type == [Any?].self:
        return values.map(\.cellObject)
    case let .object(values) where type == [String: Any?].self:
        return values.mapValues(\.cellObject)
    default:
        throw mismatch
    }
}

func td_typed_row(_ values: [String: TDAnyValue], for frame: DataFrame) throws -> [String: Any?] {
    let names = Set(frame.columns.map(\.name))
    let unknown = values.keys.filter { !names.contains($0) }.sorted()
    guard unknown.isEmpty else {
        let list = unknown.map { "'\($0)'" }.joined(separator: ", ")
        throw td_invalid_argument("the data frame has no column named \(list)")
    }
    guard !frame.columns.isEmpty else {
        throw td_invalid_argument("cannot add a row to a data frame without columns")
    }
    var row: [String: Any?] = [:]
    for column in frame.columns {
        let cell = try td_cell_value(
            values[column.name] ?? .null,
            as: column.wrappedElementType,
            column: column.name
        )
        row.updateValue(cell, forKey: column.name)
    }
    return row
}

func td_single_row_frame(_ values: [String: TDAnyValue], like frame: DataFrame) throws -> DataFrame {
    let row = try td_typed_row(values, for: frame)
    var single = td_empty_frame(like: frame)
    single.append(valuesByColumn: row)
    return single
}
