import Foundation
import WebKit

func wkMakeUserScript(
    source: String,
    injectionTime: Int32,
    mainFrameOnly: Bool,
    contentWorldName: String?
) -> WKUserScript {
    let time: WKUserScriptInjectionTime = injectionTime == 0 ? .atDocumentStart : .atDocumentEnd
    if #available(macOS 11.0, *), let contentWorldName, !contentWorldName.isEmpty {
        return WKUserScript(
            source: source,
            injectionTime: time,
            forMainFrameOnly: mainFrameOnly,
            in: .world(name: contentWorldName)
        )
    }
    return WKUserScript(
        source: source,
        injectionTime: time,
        forMainFrameOnly: mainFrameOnly
    )
}
