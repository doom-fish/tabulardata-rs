// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "TabularDataBridge",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "TabularDataBridge",
            type: .static,
            targets: ["TabularDataBridge"]
        )
    ],
    targets: [
        .target(
            name: "TabularDataBridge",
            path: "Sources/TabularDataBridge",
            publicHeadersPath: "include"
        )
    ]
)
