// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "WebKitBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "WebKitBridge",
            type: .static,
            targets: ["WebKitBridge"])
    ],
    targets: [
        .target(
            name: "WebKitBridge",
            path: "Sources/WebKitBridge",
            publicHeadersPath: "include")
    ]
)
