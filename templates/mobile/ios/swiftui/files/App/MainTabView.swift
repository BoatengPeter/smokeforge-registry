import Core
import Foundation
import HomeFeature
import Observation
import SettingsFeature
import SwiftUI

@MainActor
struct MainTabView: View {
    let container: AppContainer
    let user: User
    @Bindable var coordinator: AppCoordinator

    var body: some View {
        TabView(selection: $coordinator.selectedTab) {
            HomeView(repository: container.systemRepository, user: user)
                .tabItem {
                    Label(String(localized: "tab.home", bundle: .main), systemImage: "house")
                }
                .tag(AppTab.home)

            SettingsView(
                themeStore: container.themeStore,
                repository: container.authRepository,
                appInfo: container.config.info,
            )
            .tabItem {
                Label(String(localized: "tab.settings", bundle: .main), systemImage: "gearshape")
            }
            .tag(AppTab.settings)
        }
    }
}
