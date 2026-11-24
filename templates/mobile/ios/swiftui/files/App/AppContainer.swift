import Core
import Data
import Foundation
import Networking
import Storage

@MainActor
final class AppContainer {
    let config: AppConfig
    let session: SessionStore
    let authRepository: AuthRepositoryImpl
    let systemRepository: SystemRepositoryImpl
    let themeStore: ThemeStore

    init(config: AppConfig = .from()) {
        let tokenStore = KeychainTokenStore(
            service: (Bundle.main.bundleIdentifier ?? "com.example.app") + ".session",
        )
        let api = APIClient(baseURL: config.baseURL)
        let session = SessionStore()

        self.config = config
        self.session = session
        self.authRepository = AuthRepositoryImpl(api: api, tokens: tokenStore, session: session)
        self.systemRepository = SystemRepositoryImpl(api: api, tokens: tokenStore)
        self.themeStore = ThemeStore()
    }

    static let live = AppContainer()
}
