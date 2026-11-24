import Core
import DesignSystem
import Foundation
import SwiftUI

@MainActor
public struct HomeView: View {
    @State private var model: HomeViewModel

    public init(repository: any SystemRepository, user: User) {
        _model = State(initialValue: HomeViewModel(repository: repository, user: user))
    }

    public var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 18) {
                    SmokeScreenHeader(
                        String(localized: "home.title", bundle: .module),
                        subtitle: String(localized: "home.subtitle", bundle: .module),
                    )

                    Text(String(format: String(localized: "home.greeting", bundle: .module), model.user.email))
                        .font(.body)

                    if let error = model.error {
                        SmokeErrorBanner(error.userMessage) {
                            Task { await model.refresh() }
                        }
                    }

                    SmokeCard(String(localized: "home.api_panel", bundle: .module)) {
                        if model.isLoading {
                            ProgressView()
                        } else {
                            Text(model.apiMessage ?? String(localized: "home.api_pending", bundle: .module))
                                .foregroundStyle(SmokeColor.signal)
                        }
                    }

                    SmokeCard(String(localized: "home.account_panel", bundle: .module)) {
                        Text(model.user.email)
                        Text(String(format: String(localized: "home.account_role", bundle: .module), model.user.role))
                            .foregroundStyle(.secondary)
                        Text(
                            String(
                                localized: model.user.emailVerified
                                    ? "home.account_verified"
                                    : "home.account_unverified",
                                bundle: .module,
                            ),
                        )
                        .foregroundStyle(.secondary)
                    }

                    Button {
                        Task { await model.refresh() }
                    } label: {
                        Text("home.refresh", bundle: .module)
                    }
                    .buttonStyle(.bordered)
                    .frame(maxWidth: .infinity)
                }
                .padding(24)
            }
            .navigationTitle(String(localized: "home.nav_title", bundle: .module))
        }
        .smokeTheme()
        .task {
            await model.refresh()
        }
    }
}
