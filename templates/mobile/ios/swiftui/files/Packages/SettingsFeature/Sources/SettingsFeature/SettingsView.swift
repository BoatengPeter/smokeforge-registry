import Core
import DesignSystem
import Foundation
import SwiftUI

@MainActor
public struct SettingsView: View {
    @State private var model: SettingsViewModel

    public init(
        themeStore: ThemeStore,
        repository: any AuthRepository,
        appInfo: AppInfo,
    ) {
        _model = State(
            initialValue: SettingsViewModel(
                themeStore: themeStore,
                authRepository: repository,
                appInfo: appInfo,
            ),
        )
    }

    public var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 18) {
                    SmokeScreenHeader(
                        String(localized: "settings.title", bundle: .module),
                        subtitle: String(localized: "settings.subtitle", bundle: .module),
                    )

                    if let error = model.error {
                        SmokeErrorBanner(error.userMessage)
                    }

                    SmokeCard(String(localized: "settings.appearance", bundle: .module)) {
                        HStack(spacing: 8) {
                            ForEach(ThemeMode.allCases, id: \.self) { mode in
                                Button {
                                    model.themeStore.set(mode)
                                } label: {
                                    Text(themeTitle(mode), bundle: .module)
                                }
                                .buttonStyle(.bordered)
                                .tint(model.themeStore.mode == mode ? SmokeColor.signal : nil)
                            }
                        }
                    }

                    SmokeCard(String(localized: "settings.environment", bundle: .module)) {
                        Text(String(format: String(localized: "settings.environment_value", bundle: .module), model.appInfo.environment))
                        Text(model.appInfo.apiBaseURL)
                            .foregroundStyle(.secondary)
                        Text(String(format: String(localized: "settings.version", bundle: .module), model.appInfo.versionName))
                            .font(.footnote)
                            .foregroundStyle(.secondary)
                    }

                    SmokeCard(String(localized: "settings.account", bundle: .module)) {
                        Button {
                            Task { await model.signOut() }
                        } label: {
                            if model.isSigningOut {
                                ProgressView()
                                    .frame(maxWidth: .infinity)
                            } else {
                                Text("settings.sign_out", bundle: .module)
                            }
                        }
                        .buttonStyle(.bordered)
                        .disabled(model.isSigningOut)
                    }
                }
                .padding(24)
            }
            .navigationTitle(String(localized: "settings.nav_title", bundle: .module))
        }
        .smokeTheme()
    }

    private func themeTitle(_ mode: ThemeMode) -> LocalizedStringKey {
        switch mode {
        case .system: "settings.theme.system"
        case .light: "settings.theme.light"
        case .dark: "settings.theme.dark"
        }
    }
}
