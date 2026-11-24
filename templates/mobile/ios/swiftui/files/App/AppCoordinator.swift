import Observation

enum AppTab: Hashable {
    case home
    case settings
}

@MainActor
@Observable
final class AppCoordinator {
    var selectedTab: AppTab = .home
}
