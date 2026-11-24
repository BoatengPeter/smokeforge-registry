// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "SettingsFeature",
    defaultLocalization: "en",
    platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(name: "SettingsFeature", targets: ["SettingsFeature"]),
    ],
    dependencies: [
        .package(name: "Core", path: "../Core"),
        .package(name: "DesignSystem", path: "../DesignSystem"),
    ],
    targets: [
        .target(
            name: "SettingsFeature",
            dependencies: [
                .product(name: "Core", package: "Core"),
                .product(name: "DesignSystem", package: "DesignSystem"),
            ],
            resources: [.process("Resources")],
        ),
    ]
)
