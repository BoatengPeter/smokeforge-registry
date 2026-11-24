import Core
import Foundation
import Observation

public enum AuthMode: Equatable {
    case login
    case register
    case forgot
}

public enum AuthNotice: Equatable {
    case accountCreated
    case resetSent
}

@MainActor
@Observable
public final class AuthViewModel {
    public var mode: AuthMode = .login
    public var email = ""
    public var password = ""
    public var confirmPassword = ""
    public private(set) var isLoading = false
    public private(set) var error: AppError?
    public private(set) var notice: AuthNotice?

    private let repository: any AuthRepository

    public init(repository: any AuthRepository) {
        self.repository = repository
    }

    public func setMode(_ mode: AuthMode) {
        self.mode = mode
        error = nil
        notice = nil
        password = ""
        confirmPassword = ""
    }

    public func submit() async {
        guard !isLoading else { return }
        if let validation = validate() {
            error = validation
            return
        }

        isLoading = true
        error = nil
        notice = nil
        defer { isLoading = false }

        do {
            switch mode {
            case .login:
                try await repository.login(
                    email: email.trimmingCharacters(in: .whitespacesAndNewlines),
                    password: password,
                )
                password = ""
                confirmPassword = ""
            case .register:
                try await repository.register(
                    email: email.trimmingCharacters(in: .whitespacesAndNewlines),
                    password: password,
                )
                mode = .login
                password = ""
                confirmPassword = ""
                notice = .accountCreated
            case .forgot:
                try await repository.requestPasswordReset(
                    email: email.trimmingCharacters(in: .whitespacesAndNewlines),
                )
                notice = .resetSent
            }
        } catch let error as AppError {
            self.error = error
        } catch {
            self.error = .unknown
        }
    }

    private func validate() -> AppError? {
        let trimmed = email.trimmingCharacters(in: .whitespacesAndNewlines)
        if trimmed.isEmpty {
            return .validation("Enter your email address.")
        }

        if !trimmed.contains("@") || !trimmed.contains(".") {
            return .validation("Enter a valid email address.")
        }

        if mode == .forgot {
            return nil
        }

        if password.isEmpty {
            return .validation("Enter your password.")
        }

        if mode == .register && password.count < 12 {
            return .validation("Use at least 12 characters for the password.")
        }

        if mode == .register && password != confirmPassword {
            return .validation("Passwords do not match.")
        }

        return nil
    }
}
