import Foundation
import WebKit

func wkSanitizedDownloadFilename(_ suggestedFilename: String) -> String {
    let forbidden = CharacterSet(charactersIn: "/\\:").union(.controlCharacters).union(.newlines)
    var name = String(String.UnicodeScalarView(suggestedFilename.unicodeScalars.map {
        forbidden.contains($0) ? "_" : $0
    }))
    name = name.trimmingCharacters(in: .whitespaces)
    while name.hasPrefix(".") {
        name.removeFirst()
    }
    name = name.trimmingCharacters(in: .whitespaces)
    while name.utf8.count > 240 {
        name.removeLast()
    }
    return name.isEmpty ? "download" : name
}

final class WKRustDownloadDelegate: NSObject, WKDownloadDelegate {
    var destinationDirectory: String
    var redirectPolicy: WKDownload.RedirectPolicy = .allow
    let events = WKRustEventQueue()
    var finalDestinationPath: String?
    var lastResumeData: Data?

    init(destinationDirectory: String) {
        self.destinationDirectory = destinationDirectory
        super.init()
    }

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        events.drain()
    }

    private func emit(_ payload: [String: Any]) {
        events.append(payload)
    }

    private func uniqueDestinationURL(for suggestedFilename: String) -> URL? {
        let directoryURL = URL(fileURLWithPath: destinationDirectory, isDirectory: true).standardizedFileURL
        try? FileManager.default.createDirectory(at: directoryURL, withIntermediateDirectories: true)
        let fileName = wkSanitizedDownloadFilename(suggestedFilename)
        var candidate = directoryURL.appendingPathComponent(fileName, isDirectory: false)
        let base = candidate.deletingPathExtension().lastPathComponent
        let ext = candidate.pathExtension
        var attempt = 1
        while FileManager.default.fileExists(atPath: candidate.path) {
            let suffix = "-\(attempt)"
            let uniqueName = ext.isEmpty ? base + suffix : base + suffix + "." + ext
            candidate = directoryURL.appendingPathComponent(uniqueName, isDirectory: false)
            attempt += 1
        }
        guard candidate.standardizedFileURL.deletingLastPathComponent().path == directoryURL.path else {
            return nil
        }
        return candidate
    }

    func download(
        _ download: WKDownload,
        decideDestinationUsing response: URLResponse,
        suggestedFilename: String,
        completionHandler: @escaping (URL?) -> Void
    ) {
        guard let destinationURL = uniqueDestinationURL(for: suggestedFilename) else {
            emit([
                "kind": "destinationRejected",
                "error": "the suggested file name does not resolve inside the destination directory",
                "suggestedFilename": suggestedFilename,
                "originalRequestURL": download.originalRequest?.url?.absoluteString ?? ""
            ])
            completionHandler(nil)
            return
        }
        finalDestinationPath = destinationURL.path
        emit([
            "kind": "destination",
            "suggestedFilename": suggestedFilename,
            "destination": destinationURL.path,
            "originalRequestURL": download.originalRequest?.url?.absoluteString ?? ""
        ])
        completionHandler(destinationURL)
    }

    func downloadDidFinish(_ download: WKDownload) {
        emit([
            "kind": "finish",
            "destination": finalDestinationPath ?? NSNull(),
            "originalRequestURL": download.originalRequest?.url?.absoluteString ?? ""
        ])
    }

    func download(_ download: WKDownload, didFailWithError error: Error, resumeData: Data?) {
        lastResumeData = resumeData
        emit([
            "kind": "fail",
            "error": error.localizedDescription,
            "destination": finalDestinationPath ?? NSNull(),
            "hasResumeData": resumeData != nil,
            "originalRequestURL": download.originalRequest?.url?.absoluteString ?? ""
        ])
    }

    func download(
        _ download: WKDownload,
        willPerformHTTPRedirection response: HTTPURLResponse,
        newRequest request: URLRequest,
        decisionHandler: @escaping (WKDownload.RedirectPolicy) -> Void
    ) {
        emit([
            "kind": "redirect",
            "statusCode": response.statusCode,
            "url": request.url?.absoluteString ?? ""
        ])
        decisionHandler(redirectPolicy)
    }
}

final class WKDownloadBox: NSObject {
    let download: WKDownload
    let delegate: WKRustDownloadDelegate

    init(download: WKDownload, destinationDirectory: String) {
        self.download = download
        self.delegate = WKRustDownloadDelegate(destinationDirectory: destinationDirectory)
        super.init()
        self.download.delegate = delegate
    }
}

@_cdecl("wk_download_release")
public func wk_download_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkReleaseOnMain(ptr)
}

@_cdecl("wk_download_copy_original_request_url")
public func wk_download_copy_original_request_url(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKDownloadBox = wkBorrow(ptr)
    return wkCString(wkOnMain { box.download.originalRequest?.url?.absoluteString ?? "" })
}

@_cdecl("wk_download_is_user_initiated")
public func wk_download_is_user_initiated(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKDownloadBox = wkBorrow(ptr)
    return wkOnMain {
        if #available(macOS 15.2, *) {
            return box.download.isUserInitiated
        }
        return false
    }
}

@_cdecl("wk_download_copy_events_json")
public func wk_download_copy_events_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return WKRustEventQueue().drain() }
    let box: WKDownloadBox = wkBorrow(ptr)
    return wkOnMain { box.delegate.drainEvents() }
}

@_cdecl("wk_download_set_redirect_policy")
public func wk_download_set_redirect_policy(_ ptr: UnsafeMutableRawPointer?, _ rawValue: Int32) {
    guard let ptr else { return }
    let box: WKDownloadBox = wkBorrow(ptr)
    wkOnMain {
        box.delegate.redirectPolicy = rawValue == 0 ? .cancel : .allow
    }
}

@_cdecl("wk_download_get_redirect_policy")
public func wk_download_get_redirect_policy(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return 1 }
    let box: WKDownloadBox = wkBorrow(ptr)
    return wkOnMain {
        switch box.delegate.redirectPolicy {
        case .cancel:
            return 0
        case .allow:
            return 1
        @unknown default:
            return 1
        }
    }
}

@_cdecl("wk_download_cancel")
public func wk_download_cancel(
    _ ptr: UnsafeMutableRawPointer?,
    _ outResumeData: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outResumeDataLen: UnsafeMutablePointer<Int>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing download")
        return WK_INVALID_ARGUMENT
    }

    let box: WKDownloadBox = wkBorrow(ptr)
    let (status, data, error) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.download.cancel { resumeData in
                completion(resumeData, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    if let data {
        wkSetBytes(data, outResumeData, outResumeDataLen)
    }
    return status
}
