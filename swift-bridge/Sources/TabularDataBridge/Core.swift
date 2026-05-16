import CoreFoundation
import Foundation
import TabularData

public let TDR_OK: Int32 = 0
public let TDR_INVALID_ARGUMENT: Int32 = -1
public let TDR_FRAMEWORK_ERROR: Int32 = -2

final class TDDataFrameBox: NSObject {
    var frame: DataFrame

    init(frame: DataFrame) {
        self.frame = frame
        super.init()
    }
}

@inline(__always)
public func td_retain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func td_borrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func td_box(_ ptr: UnsafeMutableRawPointer?) -> TDDataFrameBox? {
    guard let ptr else {
        return nil
    }
    let box: TDDataFrameBox = td_borrow(ptr)
    return box
}

@_cdecl("td_object_release")
public func td_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
func td_string(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

func td_codable_json_string<T: Encodable>(_ value: T) -> String {
    let encoder = JSONEncoder()
    encoder.outputFormatting = [.sortedKeys]
    do {
        let data = try encoder.encode(value)
        return String(data: data, encoding: .utf8) ?? "null"
    } catch {
        return "null"
    }
}

@inline(__always)
func td_write_error(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = td_string(message)
}

func td_invalid_argument(_ message: String) -> NSError {
    NSError(domain: "tabulardata-rs", code: Int(TDR_INVALID_ARGUMENT), userInfo: [
        NSLocalizedDescriptionKey: message,
    ])
}

func td_character(_ value: String, fieldName: String) throws -> Character {
    guard let character = value.first, value.count == 1 else {
        throw td_invalid_argument("\(fieldName) must be a single character")
    }
    return character
}

func td_json_safe(_ value: Any?) -> Any {
    guard let value else {
        return NSNull()
    }

    switch value {
    case let dict as [String: Any]:
        return dict.mapValues { td_json_safe($0) }
    case let dict as [String: Any?]:
        return dict.mapValues { td_json_safe($0) }
    case let dict as NSDictionary:
        var object: [String: Any] = [:]
        for (key, value) in dict {
            object[String(describing: key)] = td_json_safe(value)
        }
        return object
    case let array as [Any]:
        return array.map { td_json_safe($0) }
    case let array as [Any?]:
        return array.map { td_json_safe($0) }
    case let array as NSArray:
        return array.map { td_json_safe($0) }
    case let date as Date:
        return date.timeIntervalSince1970
    case let data as Data:
        return data.base64EncodedString()
    case let number as NSNumber:
        if CFGetTypeID(number) == CFBooleanGetTypeID() {
            return number.boolValue
        }
        return number
    case let string as String:
        return string
    case _ as NSNull:
        return NSNull()
    default:
        return String(describing: value)
    }
}

func td_json_string(_ value: Any) -> String {
    let safe = td_json_safe(value)

    func encode(_ object: Any) -> String? {
        guard JSONSerialization.isValidJSONObject(object) else {
            return nil
        }
        do {
            let data = try JSONSerialization.data(withJSONObject: object, options: [.sortedKeys])
            return String(data: data, encoding: .utf8)
        } catch {
            return nil
        }
    }

    if let encoded = encode(safe) {
        return encoded
    }
    if let encodedScalar = encode([safe]) {
        return String(encodedScalar.dropFirst().dropLast())
    }
    return "null"
}

enum TDJSONValue: Codable {
    case string(String)
    case int(Int64)
    case double(Double)
    case bool(Bool)
    case object([String: TDJSONValue])
    case array([TDJSONValue])
    case null

    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if container.decodeNil() {
            self = .null
        } else if let value = try? container.decode(Bool.self) {
            self = .bool(value)
        } else if let value = try? container.decode(Int64.self) {
            self = .int(value)
        } else if let value = try? container.decode(Double.self) {
            self = .double(value)
        } else if let value = try? container.decode(String.self) {
            self = .string(value)
        } else if let value = try? container.decode([String: TDJSONValue].self) {
            self = .object(value)
        } else if let value = try? container.decode([TDJSONValue].self) {
            self = .array(value)
        } else {
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "unsupported JSON value")
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .string(let value):
            try container.encode(value)
        case .int(let value):
            try container.encode(value)
        case .double(let value):
            try container.encode(value)
        case .bool(let value):
            try container.encode(value)
        case .object(let value):
            try container.encode(value)
        case .array(let value):
            try container.encode(value)
        case .null:
            try container.encodeNil()
        }
    }

    var foundationObject: Any {
        switch self {
        case .string(let value):
            return value
        case .int(let value):
            return value
        case .double(let value):
            return value
        case .bool(let value):
            return value
        case .object(let value):
            return value.mapValues(\.foundationObject)
        case .array(let value):
            return value.map(\.foundationObject)
        case .null:
            return NSNull()
        }
    }
}

func td_decode_json<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let cString else {
        throw td_invalid_argument("missing JSON payload")
    }

    let data = Data(String(cString: cString).utf8)
    return try JSONDecoder().decode(T.self, from: data)
}

enum TDAnyValue: Codable, Equatable {
    case null
    case string(String)
    case int(Int64)
    case double(Double)
    case bool(Bool)
    case date(Double)
    case data(String)
    case array([TDAnyValue])
    case object([String: TDAnyValue])

    private enum CodingKeys: String, CodingKey {
        case kind
        case value
    }

    private enum Kind: String, Codable {
        case null
        case string
        case int
        case double
        case bool
        case date
        case data
        case array
        case object
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        let kind = try container.decode(Kind.self, forKey: .kind)
        switch kind {
        case .null:
            self = .null
        case .string:
            self = .string(try container.decode(String.self, forKey: .value))
        case .int:
            self = .int(try container.decode(Int64.self, forKey: .value))
        case .double:
            self = .double(try container.decode(Double.self, forKey: .value))
        case .bool:
            self = .bool(try container.decode(Bool.self, forKey: .value))
        case .date:
            self = .date(try container.decode(Double.self, forKey: .value))
        case .data:
            self = .data(try container.decode(String.self, forKey: .value))
        case .array:
            self = .array(try container.decode([TDAnyValue].self, forKey: .value))
        case .object:
            self = .object(try container.decode([String: TDAnyValue].self, forKey: .value))
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        switch self {
        case .null:
            try container.encode(Kind.null, forKey: .kind)
        case .string(let value):
            try container.encode(Kind.string, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .int(let value):
            try container.encode(Kind.int, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .double(let value):
            try container.encode(Kind.double, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .bool(let value):
            try container.encode(Kind.bool, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .date(let value):
            try container.encode(Kind.date, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .data(let value):
            try container.encode(Kind.data, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .array(let value):
            try container.encode(Kind.array, forKey: .kind)
            try container.encode(value, forKey: .value)
        case .object(let value):
            try container.encode(Kind.object, forKey: .kind)
            try container.encode(value, forKey: .value)
        }
    }

    static func fromFoundation(_ value: Any?) -> TDAnyValue {
        guard let value else {
            return .null
        }

        switch value {
        case let dict as [String: Any]:
            return .object(dict.mapValues { Self.fromFoundation($0) })
        case let dict as [String: Any?]:
            return .object(dict.mapValues { Self.fromFoundation($0) })
        case let dict as NSDictionary:
            var object: [String: TDAnyValue] = [:]
            for (key, value) in dict {
                object[String(describing: key)] = Self.fromFoundation(value)
            }
            return .object(object)
        case let array as [Any]:
            return .array(array.map { Self.fromFoundation($0) })
        case let array as [Any?]:
            return .array(array.map { Self.fromFoundation($0) })
        case let array as NSArray:
            return .array(array.map { Self.fromFoundation($0) })
        case let date as Date:
            return .date(date.timeIntervalSince1970)
        case let data as Data:
            return .data(data.base64EncodedString())
        case let number as NSNumber:
            return td_any_number(number)
        case let string as String:
            return .string(string)
        case _ as NSNull:
            return .null
        default:
            return .string(String(describing: value))
        }
    }

    var numericValue: Double? {
        switch self {
        case .int(let value):
            return Double(value)
        case .double(let value), .date(let value):
            return value
        default:
            return nil
        }
    }

    var foundationObject: Any {
        switch self {
        case .null:
            return NSNull()
        case .string(let value):
            return value
        case .int(let value):
            return Int(exactly: value) ?? value
        case .double(let value):
            return value
        case .bool(let value):
            return value
        case .date(let value):
            return Date(timeIntervalSince1970: value)
        case .data(let value):
            return Data(base64Encoded: value) ?? Data()
        case .array(let values):
            return values.map(\.foundationObject)
        case .object(let value):
            return value.mapValues(\.foundationObject)
        }
    }

    var cellObject: Any? {
        switch self {
        case .null:
            return nil
        default:
            return foundationObject
        }
    }
}

private func td_any_number(_ number: NSNumber) -> TDAnyValue {
    if CFGetTypeID(number) == CFBooleanGetTypeID() {
        return .bool(number.boolValue)
    }
    let typeEncoding = String(cString: number.objCType)
    if ["c", "i", "s", "l", "q", "C", "I", "S", "L", "Q"].contains(typeEncoding) {
        return .int(number.int64Value)
    }
    return .double(number.doubleValue)
}

func td_any_value_equal(_ lhs: TDAnyValue, _ rhs: TDAnyValue) -> Bool {
    if let left = lhs.numericValue, let right = rhs.numericValue {
        return left == right
    }
    return lhs == rhs
}

func td_any_value_compare(_ lhs: TDAnyValue, _ rhs: TDAnyValue) -> ComparisonResult? {
    switch (lhs, rhs) {
    case (.string(let left), .string(let right)), (.data(let left), .data(let right)):
        return left.compare(right)
    case (.bool(let left), .bool(let right)):
        if left == right { return .orderedSame }
        return left ? .orderedDescending : .orderedAscending
    default:
        if let left = lhs.numericValue, let right = rhs.numericValue {
            if left == right { return .orderedSame }
            return left < right ? .orderedAscending : .orderedDescending
        }
        return nil
    }
}

func td_any_value_contains(_ haystack: TDAnyValue, _ needle: TDAnyValue) -> Bool {
    switch haystack {
    case .string(let value):
        if case .string(let substring) = needle {
            return value.contains(substring)
        }
        if case .data(let substring) = needle {
            return value.contains(substring)
        }
        return false
    case .array(let values):
        return values.contains { td_any_value_equal($0, needle) }
    case .object(let value):
        if case .string(let key) = needle {
            return value.keys.contains(key)
        }
        return false
    default:
        return false
    }
}

struct TDAnyColumnPayload: Codable {
    var name: String
    var type_name: String
    var missing_count: Int
    var values: [TDAnyValue]
}

struct TDColumnSlicePayload: Codable {
    var name: String
    var type_name: String
    var missing_count: Int
    var values: [TDAnyValue]
    var contiguous: Bool
    var indices: [Int]
}

struct TDAnyRowPayload: Codable {
    var index: Int?
    var values: [String: TDAnyValue]
}

func td_type_name(_ type: Any.Type) -> String {
    String(describing: type)
}

func td_any_column_payload(_ column: AnyColumn) -> TDAnyColumnPayload {
    TDAnyColumnPayload(
        name: column.name,
        type_name: td_type_name(column.wrappedElementType),
        missing_count: column.missingCount,
        values: Array(column).map { TDAnyValue.fromFoundation($0) }
    )
}

func td_column_slice_payload(
    name: String,
    typeName: String,
    values: [Any?],
    contiguous: Bool,
    indices: [Int]
) -> TDColumnSlicePayload {
    TDColumnSlicePayload(
        name: name,
        type_name: typeName,
        missing_count: values.filter { $0 == nil }.count,
        values: values.map { TDAnyValue.fromFoundation($0) },
        contiguous: contiguous,
        indices: indices
    )
}

func td_column_slice_payload(
    _ slice: AnyColumnSlice,
    contiguous: Bool,
    indices: [Int]
) -> TDColumnSlicePayload {
    td_column_slice_payload(
        name: slice.name,
        typeName: td_type_name(slice.wrappedElementType),
        values: Array(slice),
        contiguous: contiguous,
        indices: indices
    )
}

func td_row_payload(_ row: DataFrame.Row) -> TDAnyRowPayload {
    let columnNames = row.base.columns.map(\.name)
    var values: [String: TDAnyValue] = [:]
    for columnName in columnNames {
        values[columnName] = TDAnyValue.fromFoundation(row[columnName])
    }
    return TDAnyRowPayload(index: row.index, values: values)
}

func td_row_dictionary(_ payload: TDAnyRowPayload) -> [String: Any?] {
    payload.values.mapValues(\.cellObject)
}

func td_row_dictionary(_ row: DataFrame.Row, columnNames: [String]) -> [String: Any?] {
    var object: [String: Any?] = [:]
    for columnName in columnNames {
        object[columnName] = row[columnName]
    }
    return object
}

func td_row_objects(_ frame: DataFrame) -> [[String: Any]] {
    let columnNames = frame.columns.map(\.name)
    return frame.rows.map { row in
        var object: [String: Any] = [:]
        for columnName in columnNames {
            object[columnName] = td_json_safe(row[columnName])
        }
        return object
    }
}

func td_empty_frame(like frame: DataFrame) -> DataFrame {
    let columns = frame.columns.map { $0.prototype.makeColumn(capacity: 0) }
    return DataFrame(columns: columns)
}

func td_frame(from rowDictionaries: [[String: Any?]], like frame: DataFrame) -> DataFrame {
    var result = td_empty_frame(like: frame)
    for row in rowDictionaries {
        result.append(valuesByColumn: row)
    }
    return result
}
