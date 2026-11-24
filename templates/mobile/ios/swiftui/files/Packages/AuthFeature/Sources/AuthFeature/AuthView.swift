import Core
import DesignSystem
import Foundation
import SwiftUI

@MainActor
public struct AuthView: View {
    @State private var model: AuthViewModel

    public init(repository: any AuthRepository) {
        _model = State(initialValue: AuthViewModel(repository: repository))
    }

    public var body: some View {
        @Bindable var model = model

        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 18) {
                    Text("SMOKE")
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(SmokeColor.signal)

                    SmokeScreenHeader(
                        titleKey.title,
                        subtitle: titleKey.subtitle,
                    )

                    TextField(String(localized: "auth.email", bundle: .module), text: $model.email)
                        .textFieldStyle(.roundedBorder)
                        .emailInput()
                        .autocorrectionDisabled()

                    if model.mode != .forgot {
                        SecureField(String(localized: "auth.password", bundle: .module), text: $model.password)
                            .textFieldStyle(.roundedBorder)
                            .passwordInput(isNew: model.mode == .register)
                    }

                    if model.mode == .register {
                        SecureField(String(localized: "auth.confirm_password", bundle: .module), text: $model.confirmPassword)
                            .textFieldStyle(.roundedBorder)
                            .passwordInput(isNew: true)
                    }

                    if let error = model.error {
                        SmokeErrorBanner(error.userMessage)
                    }

                    if let notice = model.notice {
                        Text(noticeKey(notice), bundle: .module)
                            .font(.callout)
                            .foregroundStyle(SmokeColor.signal)
                    }

                    Button {
                        Task { await model.submit() }
                    } label: {
                        if model.isLoading {
                            ProgressView()
                                .frame(maxWidth: .infinity)
                        } else {
                            Text(actionKey, bundle: .module)
                        }
                    }
                    .buttonStyle(SmokePrimaryButtonStyle())
                    .disabled(model.isLoading)

                    HStack {
                        if model.mode == .login {
                            Button {
                                model.setMode(.register)
                            } label: {
                                Text("auth.need_account", bundle: .module)
                            }
                            Spacer()
                            Button {
                                model.setMode(.forgot)
                            } label: {
                                Text("auth.forgot_password", bundle: .module)
                            }
                        } else {
                            Button {
                                model.setMode(.login)
                            } label: {
                                Text("auth.have_account", bundle: .module)
                            }
                        }
                    }
                    .font(.footnote)
                }
                .padding(24)
            }
        }
        .smokeTheme()
    }

    private var titleKey: (title: String, subtitle: String) {
        switch model.mode {
        case .login:
            (String(localized: "auth.login.title", bundle: .module), String(localized: "auth.login.subtitle", bundle: .module))
        case .register:
            (String(localized: "auth.register.title", bundle: .module), String(localized: "auth.register.subtitle", bundle: .module))
        case .forgot:
            (String(localized: "auth.forgot.title", bundle: .module), String(localized: "auth.forgot.subtitle", bundle: .module))
        }
    }

    private var actionKey: LocalizedStringKey {
        switch model.mode {
        case .login: "auth.sign_in"
        case .register: "auth.create_account"
        case .forgot: "auth.send_reset"
        }
    }

    private func noticeKey(_ notice: AuthNotice) -> LocalizedStringKey {
        switch notice {
        case .accountCreated: "auth.notice.account_created"
        case .resetSent: "auth.notice.reset_sent"
        }
    }
}


private extension View {
    @ViewBuilder
    func emailInput() -> some View {
        #if os(iOS)
        textContentType(.emailAddress)
            .keyboardType(.emailAddress)
            .textInputAutocapitalization(.never)
        #else
        self
        #endif
    }

    @ViewBuilder
    func passwordInput(isNew: Bool) -> some View {
        #if os(iOS)
        textContentType(isNew ? .newPassword : .password)
        #else
        self
        #endif
    }
}
