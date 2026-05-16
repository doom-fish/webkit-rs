import Foundation
import WebKit

final class WKNavigationBox: NSObject {
    let navigation: WKNavigation

    init(navigation: WKNavigation) {
        self.navigation = navigation
        super.init()
    }
}

@_cdecl("wk_navigation_release")
public func wk_navigation_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}
