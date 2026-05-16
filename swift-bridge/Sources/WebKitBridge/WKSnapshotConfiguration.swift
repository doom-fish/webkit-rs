import Foundation
import WebKit

func wkMakeSnapshotConfiguration(
    hasRect: Bool,
    x: Double,
    y: Double,
    width: Double,
    height: Double,
    hasSnapshotWidth: Bool,
    snapshotWidth: Double,
    afterScreenUpdates: Bool
) -> WKSnapshotConfiguration {
    let configuration = WKSnapshotConfiguration()
    if hasRect {
        configuration.rect = CGRect(x: x, y: y, width: width, height: height)
    }
    if hasSnapshotWidth {
        configuration.snapshotWidth = NSNumber(value: snapshotWidth)
    }
    if #available(macOS 10.15, *) {
        configuration.afterScreenUpdates = afterScreenUpdates
    }
    return configuration
}
