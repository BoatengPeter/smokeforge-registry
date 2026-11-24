import Foundation
import Observation

@MainActor
@Observable
public final class ThemeStore {
    public private(set) var mode: ThemeMode
    private let defaults: UserDefaults
    private let key: String

    public init(defaults: UserDefaults = .standard, key: String = "ui.theme") {
        self.defaults = defaults
        self.key = key
        self.mode = ThemeMode(rawValue: defaults.string(forKey: key) ?? "") ?? .system
    }

    public func set(_ mode: ThemeMode) {
        self.mode = mode
        defaults.set(mode.rawValue, forKey: key)
    }
}
