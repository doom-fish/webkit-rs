import Foundation
import WebKit

func wkBackForwardListSnapshot(_ list: WKBackForwardList) -> [[String: Any]] {
    var snapshot: [[String: Any]] = []

    var backIndex = -1
    while let item = list.item(at: backIndex) {
        snapshot.insert(wkBackForwardListItemDictionary(item, relativeIndex: backIndex), at: 0)
        backIndex -= 1
    }
    if let currentItem = list.item(at: 0) {
        snapshot.append(wkBackForwardListItemDictionary(currentItem, relativeIndex: 0))
    }
    var forwardIndex = 1
    while let item = list.item(at: forwardIndex) {
        snapshot.append(wkBackForwardListItemDictionary(item, relativeIndex: forwardIndex))
        forwardIndex += 1
    }

    return snapshot
}
