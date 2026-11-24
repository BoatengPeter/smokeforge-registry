import SwiftUI

public enum SmokeColor {
    public static let ink = Color(red: 0.055, green: 0.059, blue: 0.067)
    public static let chalk = Color(red: 0.906, green: 0.910, blue: 0.918)
    public static let signal = Color(red: 0.302, green: 0.553, blue: 1.0)
    public static let graphite = Color(red: 0.141, green: 0.149, blue: 0.169)
}

public struct SmokeTheme: ViewModifier {
    public init() {}

    public func body(content: Content) -> some View {
        content
            .tint(SmokeColor.signal)
            .background(SmokeColor.ink.opacity(0.02))
    }
}

public extension View {
    func smokeTheme() -> some View {
        modifier(SmokeTheme())
    }
}
