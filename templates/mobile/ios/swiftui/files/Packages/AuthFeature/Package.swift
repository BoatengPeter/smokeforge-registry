// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "AuthFeature",
    defaultLocalization: "en",
    platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(name: "AuthFeature", targets: ["AuthFeature"]),
    ],
    dependencies: [
        .package(name: "Core", path: "../Core"),
        .package(name: "DesignSystem", path: "../DesignSystem"),
        .package(name: "TestSupport", path: "../TestSupport"),
    ],
    targets: [
        .target(
            name: "AuthFeature",
            resources: [.process("Resources")],
            dependencies: [
                .product(name: "Core", package: "Core"),
                .product(name: "DesignSystem", package: "DesignSystem"),
            ],
        ),
        .testTarget(
            name: "AuthFeatureTests",
            dependencies: [
                .target(name: "AuthFeature"),
                .product(name: "Core", package: "Core"),
                .product(name: "TestSupport", package: "TestSupport"),
            ],
        ),
    ]
)
