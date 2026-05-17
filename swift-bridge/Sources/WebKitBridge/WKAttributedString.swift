import Foundation
import WebKit

private func wkJSONCompatibleValue(_ value: Any) -> Any {
    switch value {
    case is NSNull:
        return NSNull()
    case let string as String:
        return string
    case let number as NSNumber:
        return number
    case let url as URL:
        return url.absoluteString
    case let date as Date:
        return ISO8601DateFormatter().string(from: date)
    case let array as [Any]:
        return array.map(wkJSONCompatibleValue)
    case let dictionary as [AnyHashable: Any]:
        var converted: [String: Any] = [:]
        for (key, element) in dictionary {
            converted[String(describing: key)] = wkJSONCompatibleValue(element)
        }
        return converted
    default:
        return String(describing: value)
    }
}

private func wkAttributedStringOptions(from jsonCString: UnsafePointer<CChar>?) -> [NSAttributedString.DocumentReadingOptionKey: Any] {
    guard let dictionary = wkJSONObject(from: jsonCString) as? [String: Any] else {
        return [:]
    }

    var options: [NSAttributedString.DocumentReadingOptionKey: Any] = [:]
    if let readAccessURL = dictionary["readAccessURL"] as? String {
        options[.readAccessURL] = URL(fileURLWithPath: readAccessURL)
    }
    if let baseURL = dictionary["baseURL"] as? String {
        if let url = URL(string: baseURL) {
            options[.baseURL] = url
        } else {
            options[.baseURL] = URL(fileURLWithPath: baseURL)
        }
    }
    if let timeout = dictionary["timeoutSeconds"] as? NSNumber {
        options[.timeout] = timeout.doubleValue
    }
    if let multiplier = dictionary["textSizeMultiplier"] as? NSNumber {
        options[.textSizeMultiplier] = multiplier.doubleValue
    }
    if let encodingName = dictionary["textEncodingName"] as? String {
        options[.textEncodingName] = encodingName
    }
    if let characterEncoding = dictionary["characterEncoding"] as? NSNumber {
        options[.characterEncoding] = characterEncoding.uintValue
    }
    return options
}

private func wkAttributedStringDocumentAttributes(
    _ attributes: [NSAttributedString.DocumentAttributeKey: Any]?
) -> [String: Any] {
    guard let attributes else {
        return [:]
    }

    var dictionary: [String: Any] = [:]
    for (key, value) in attributes {
        dictionary[key.rawValue] = wkJSONCompatibleValue(value)
    }
    return dictionary
}

final class WKAttributedStringBox: NSObject {
    let attributedString: NSAttributedString
    let documentAttributes: [String: Any]

    init(attributedString: NSAttributedString, documentAttributes: [NSAttributedString.DocumentAttributeKey: Any]?) {
        self.attributedString = attributedString
        self.documentAttributes = wkAttributedStringDocumentAttributes(documentAttributes)
        super.init()
    }
}

private func wkSetAttributedStringResult(
    _ attributedString: NSAttributedString?,
    _ documentAttributes: [NSAttributedString.DocumentAttributeKey: Any]?,
    _ error: Error?,
    _ outAttributedString: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if let error {
        outErr?.pointee = wkCString(error.localizedDescription)
        return WK_FRAMEWORK_ERROR
    }
    guard let attributedString else {
        outErr?.pointee = wkCString("attributed string result was nil")
        return WK_UNKNOWN
    }
    outAttributedString?.pointee = wkRetain(
        WKAttributedStringBox(attributedString: attributedString, documentAttributes: documentAttributes)
    )
    return WK_OK
}

@_cdecl("wk_attributed_string_release")
public func wk_attributed_string_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_attributed_string_copy_string")
public func wk_attributed_string_copy_string(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKAttributedStringBox = wkBorrow(ptr)
    return wkCString(box.attributedString.string)
}

@_cdecl("wk_attributed_string_copy_document_attributes_json")
public func wk_attributed_string_copy_document_attributes_json(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return wkCString("{}") }
    let box: WKAttributedStringBox = wkBorrow(ptr)
    return wkCString(wkJSONString(box.documentAttributes))
}

@_cdecl("wk_attributed_string_load_html_request")
public func wk_attributed_string_load_html_request(
    _ requestUrl: UnsafePointer<CChar>?,
    _ optionsJson: UnsafePointer<CChar>?,
    _ outAttributedString: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let requestUrl,
          let url = URL(string: String(cString: requestUrl))
    else {
        outErr?.pointee = wkCString("missing or invalid HTML request URL")
        return WK_INVALID_ARGUMENT
    }

    let request = URLRequest(url: url)
    let options = wkAttributedStringOptions(from: optionsJson)
    let (status, result, error): (Int32, (NSAttributedString, [NSAttributedString.DocumentAttributeKey: Any]?)?, String?) =
        wkWaitForAsync { completion in
            DispatchQueue.main.async {
                NSAttributedString.loadFromHTML(request: request, options: options) {
                    attributedString,
                    attributes,
                    loadError in
                    if let loadError {
                        completion(nil, loadError.localizedDescription)
                    } else if let attributedString {
                        completion((attributedString, attributes), nil)
                    } else {
                        completion(nil, "attributed string result was nil")
                    }
                }
            }
        }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    return wkSetAttributedStringResult(result?.0, result?.1, nil, outAttributedString, outErr)
}

@_cdecl("wk_attributed_string_load_html_file")
public func wk_attributed_string_load_html_file(
    _ fileUrl: UnsafePointer<CChar>?,
    _ optionsJson: UnsafePointer<CChar>?,
    _ outAttributedString: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let fileUrl else {
        outErr?.pointee = wkCString("missing HTML file URL")
        return WK_INVALID_ARGUMENT
    }

    let url = URL(fileURLWithPath: String(cString: fileUrl))
    let options = wkAttributedStringOptions(from: optionsJson)
    let (status, result, error): (Int32, (NSAttributedString, [NSAttributedString.DocumentAttributeKey: Any]?)?, String?) =
        wkWaitForAsync { completion in
            DispatchQueue.main.async {
                NSAttributedString.loadFromHTML(fileURL: url, options: options) {
                    attributedString,
                    attributes,
                    loadError in
                    if let loadError {
                        completion(nil, loadError.localizedDescription)
                    } else if let attributedString {
                        completion((attributedString, attributes), nil)
                    } else {
                        completion(nil, "attributed string result was nil")
                    }
                }
            }
        }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    return wkSetAttributedStringResult(result?.0, result?.1, nil, outAttributedString, outErr)
}

@_cdecl("wk_attributed_string_load_html_string")
public func wk_attributed_string_load_html_string(
    _ string: UnsafePointer<CChar>?,
    _ optionsJson: UnsafePointer<CChar>?,
    _ outAttributedString: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let string else {
        outErr?.pointee = wkCString("missing HTML string")
        return WK_INVALID_ARGUMENT
    }

    let options = wkAttributedStringOptions(from: optionsJson)
    let html = String(cString: string)
    let (status, result, error): (Int32, (NSAttributedString, [NSAttributedString.DocumentAttributeKey: Any]?)?, String?) =
        wkWaitForAsync { completion in
            DispatchQueue.main.async {
                NSAttributedString.loadFromHTML(string: html, options: options) {
                    attributedString,
                    attributes,
                    loadError in
                    if let loadError {
                        completion(nil, loadError.localizedDescription)
                    } else if let attributedString {
                        completion((attributedString, attributes), nil)
                    } else {
                        completion(nil, "attributed string result was nil")
                    }
                }
            }
        }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    return wkSetAttributedStringResult(result?.0, result?.1, nil, outAttributedString, outErr)
}

@_cdecl("wk_attributed_string_load_html_data")
public func wk_attributed_string_load_html_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ optionsJson: UnsafePointer<CChar>?,
    _ outAttributedString: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let bytes else {
        outErr?.pointee = wkCString("missing HTML data")
        return WK_INVALID_ARGUMENT
    }

    let data = Data(bytes: bytes, count: len)
    let options = wkAttributedStringOptions(from: optionsJson)
    let (status, result, error): (Int32, (NSAttributedString, [NSAttributedString.DocumentAttributeKey: Any]?)?, String?) =
        wkWaitForAsync { completion in
            DispatchQueue.main.async {
                NSAttributedString.loadFromHTML(data: data, options: options) {
                    attributedString,
                    attributes,
                    loadError in
                    if let loadError {
                        completion(nil, loadError.localizedDescription)
                    } else if let attributedString {
                        completion((attributedString, attributes), nil)
                    } else {
                        completion(nil, "attributed string result was nil")
                    }
                }
            }
        }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    return wkSetAttributedStringResult(result?.0, result?.1, nil, outAttributedString, outErr)
}
