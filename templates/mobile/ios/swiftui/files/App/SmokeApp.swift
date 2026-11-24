import SwiftUI

@main
@MainActor
struct SmokeApp: App {
    @State private var container = AppContainer.live

    var body: some Scene {
        WindowGroup {
            RootView(container: container)
        }
    }
}
