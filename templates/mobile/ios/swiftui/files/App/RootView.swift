import AuthFeature
import Core
import SwiftUI

@MainActor
struct RootView: View {
    let container: AppContainer
    @State private var coordinator = AppCoordinator()

    var body: some View {
        Group {
            switch container.session.state {
            case .loading:
                ProgressView()
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
            case .signedOut:
                AuthView(repository: container.authRepository)
            case .signedIn(let user):
                MainTabView(container: container, user: user, coordinator: coordinator)
            }
        }
        .preferredColorScheme(colorScheme)
        .task {
            await container.authRepository.restore()
        }
    }

    private var colorScheme: ColorScheme? {
        switch container.themeStore.mode {
        case .system:
            nil
        case .light:
            .light
        case .dark:
            .dark
        }
    }
}
