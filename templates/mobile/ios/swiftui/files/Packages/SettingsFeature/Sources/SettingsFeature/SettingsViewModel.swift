import Core
import Observation

@MainActor
@Observable
public final class SettingsViewModel {
    public let themeStore: ThemeStore
    public let appInfo: AppInfo
    public private(set) var isSigningOut = false
    public private(set) var error: AppError?

    private let authRepository: any AuthRepository

    public init(
        themeStore: ThemeStore,
        authRepository: any AuthRepository,
        appInfo: AppInfo,
    ) {
        self.themeStore = themeStore
        self.authRepository = authRepository
        self.appInfo = appInfo
    }

    public func signOut() async {
        guard !isSigningOut else { return }
        isSigningOut = true
        error = nil
        await authRepository.logout()
        isSigningOut = false
    }
}
