import Foundation
import WebKit

func wkMakePDFConfiguration(
    hasRect: Bool,
    x: Double,
    y: Double,
    width: Double,
    height: Double,
    allowTransparentBackground: Bool
) -> WKPDFConfiguration {
    let configuration = WKPDFConfiguration()
    if hasRect {
        configuration.rect = CGRect(x: x, y: y, width: width, height: height)
    }
    if #available(macOS 14.0, *) {
        configuration.allowTransparentBackground = allowTransparentBackground
    }
    return configuration
}
