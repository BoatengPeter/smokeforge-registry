// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "Data",
    platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(name: "Data", targets: ["Data"]),
    ],
    dependencies: [
        .package(name: "Core", path: "../Core"),
        .package(name: "Networking", path: "../Networking"),
        .package(name: "Storage", path: "../Storage"),
    ],
    targets: [
        .target(
            name: "Data",
            dependencies: [
                .product(name: "Core", package: "Core"),
                .product(name: "Networking", package: "Networking"),
                .product(name: "Storage", package: "Storage"),
            ],
        ),
        .testTarget(
            name: "DataTests",
            dependencies: [
                .target(name: "Data"),
                .product(name: "Core", package: "Core"),
            ],
        ),
    ]
)
