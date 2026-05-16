import Foundation
import WebKit

final class WKRustDownloadDelegate: NSObject, WKDownloadDelegate {
    var destinationDirectory: String
    var events: [[String: Any]] = []
    var finalDestinationPath: String?
    var lastResumeData: Data?

    init(destinationDirectory: String) {
        self.destinationDirectory = destinationDirectory
        super.init()
    }

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        wkDrainEvents(&events)
    }

    private func emit(_ payload: [String: Any]) {
        events.append(payload)
    }

    private func uniqueDestinationURL(for suggestedFilename: String) -> URL {
        let directoryURL = URL(fileURLWithPath: destinationDirectory, isDirectory: true)
        try? FileManager.default.createDirectory(at: directoryURL, withIntermediateDirectories: true)
        var candidate = directoryURL.appendingPathComponent(suggestedFilename)
        let base = candidate.deletingPathExtension().lastPathComponent
        let ext = candidate.pathExtension
        var attempt = 1
        while FileManager.default.fileExists(atPath: candidate.path) {
            let suffix = "-\(attempt)"
            let fileName = ext.isEmpty ? base + suffix : base + suffix + "." + ext
            candidate = directoryURL.appendingPathComponent(fileName)
            attempt += 1
        }
        return candidate
    }

    func download(
        _ download: WKDownload,
        decideDestinationUsing response: URLResponse,
        suggestedFilename: String,
        completionHandler: @escaping (URL?) -> Void
    ) {
        let destinationURL = uniqueDestinationURL(for: suggestedFilename)
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
        decisionHandler(.allow)
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
    wkRelease(ptr)
}

@_cdecl("wk_download_copy_original_request_url")
public func wk_download_copy_original_request_url(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKDownloadBox = wkBorrow(ptr)
    return wkCString(box.download.originalRequest?.url?.absoluteString ?? "")
}

@_cdecl("wk_download_is_user_initiated")
public func wk_download_is_user_initiated(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKDownloadBox = wkBorrow(ptr)
    if #available(macOS 15.2, *) {
        return box.download.isUserInitiated
    }
    return false
}

@_cdecl("wk_download_copy_events_json")
public func wk_download_copy_events_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return wkCString("[]") }
    let box: WKDownloadBox = wkBorrow(ptr)
    return box.delegate.drainEvents()
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
