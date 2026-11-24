// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "TestSupport",
    platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(name: "TestSupport", targets: ["TestSupport"]),
    ],
    dependencies: [
        .package(name: "Core", path: "../Core"),
    ],
    targets: [
        .target(
            name: "TestSupport",
            dependencies: [
                .product(name: "Core", package: "Core"),
            ],
        ),
    ]
)
