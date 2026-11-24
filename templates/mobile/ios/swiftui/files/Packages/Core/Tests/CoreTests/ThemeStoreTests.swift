import Foundation
import Testing
@testable import Core

@MainActor
struct ThemeStoreTests {
    @Test
    func defaultsToSystemAndPersistsSelection() {
        let suite = "ThemeStoreTests.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suite)!
        defer { defaults.removePersistentDomain(forName: suite) }

        let store = ThemeStore(defaults: defaults)
        #expect(store.mode == .system)

        store.set(.dark)

        let restored = ThemeStore(defaults: defaults)
        #expect(restored.mode == .dark)
    }
}
