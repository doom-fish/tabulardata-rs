import Foundation
import TabularData

public let TDR_OK: Int32 = 0
public let TDR_INVALID_ARGUMENT: Int32 = -1
public let TDR_FRAMEWORK_ERROR: Int32 = -2

@inline(__always)
public func td_retain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func td_borrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
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

@inline(__always)
func td_write_error(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = td_string(message)
}

func td_json_safe(_ value: Any) -> Any {
    switch value {
    case let dict as [String: Any]:
        return dict.mapValues(td_json_safe)
    case let dict as NSDictionary:
        var object: [String: Any] = [:]
        for (key, value) in dict {
            object[String(describing: key)] = td_json_safe(value)
        }
        return object
    case let array as [Any]:
        return array.map(td_json_safe)
    case let array as NSArray:
        return array.map(td_json_safe)
    case let date as Date:
        return date.timeIntervalSince1970
    case let number as NSNumber:
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
        throw NSError(domain: "tabulardata-rs", code: Int(TDR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "missing JSON payload",
        ])
    }

    let data = Data(String(cString: cString).utf8)
    return try JSONDecoder().decode(T.self, from: data)
}
