import AppKit
import Foundation
import WebKit

let WK_OK: Int32 = 0
let WK_INVALID_ARGUMENT: Int32 = -1
let WK_TIMED_OUT: Int32 = -3
let WK_FRAMEWORK_ERROR: Int32 = -5
let WK_UNKNOWN: Int32 = -99

@inline(__always)
func wkCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func wkRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func wkBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as type: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func wkRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
func wkOnMain<T>(_ work: () -> T) -> T {
    if Thread.isMainThread {
        return work()
    }
    return DispatchQueue.main.sync(execute: work)
}

@_cdecl("wk_string_free")
public func wk_string_free(_ ptr: UnsafeMutablePointer<CChar>?) {
    guard let ptr else { return }
    free(ptr)
}

@_cdecl("wk_bytes_free")
public func wk_bytes_free(_ ptr: UnsafeMutablePointer<UInt8>?, _ len: Int) {
    guard let ptr else { return }
    ptr.deallocate()
}

@_cdecl("wk_run_loop_pump")
public func wk_run_loop_pump(_ seconds: Double) {
    let interval = max(0.001, seconds)
    wkOnMain {
        _ = RunLoop.main.run(mode: .default, before: Date(timeIntervalSinceNow: interval))
    }
}

@_cdecl("wk_init_app")
public func wk_init_app() {
    wkOnMain {
        _ = NSApplication.shared
        NSApp.setActivationPolicy(.prohibited)
    }
}

func wkWait(timeoutSeconds: Double = 30.0, isDone: () -> Bool) -> Bool {
    let deadline = Date(timeIntervalSinceNow: timeoutSeconds)
    while !isDone() {
        if Date() >= deadline {
            return false
        }
        wkOnMain {
            _ = RunLoop.main.run(mode: .default, before: Date(timeIntervalSinceNow: 0.02))
        }
    }
    return true
}
