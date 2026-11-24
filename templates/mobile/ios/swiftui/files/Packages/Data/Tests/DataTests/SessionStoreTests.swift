import Core
import Testing
@testable import Data

@MainActor
struct SessionStoreTests {
    @Test
    func startsLoadingAndCanSignOut() {
        let store = SessionStore()
        #expect(store.state == .loading)

        store.set(.signedOut)
        #expect(store.state == .signedOut)
    }
}
