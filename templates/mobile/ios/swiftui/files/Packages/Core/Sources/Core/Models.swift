import Foundation
import Observation

public struct User: Identifiable, Equatable, Sendable {
    public let id: String
    public let email: String
    public let role: String
    public let emailVerified: Bool
    public let createdAt: String

    public init(id: String, email: String, role: String, emailVerified: Bool, createdAt: String) {
        self.id = id
        self.email = email
        self.role = role
        self.emailVerified = emailVerified
        self.createdAt = createdAt
    }
}

public enum SessionState: Equatable, Sendable {
    case loading
    case signedOut
    case signedIn(User)
}

public enum ThemeMode: String, CaseIterable, Codable, Hashable, Sendable {
    case system
    case light
    case dark

    public var title: String {
        switch self {
        case .system: "System"
        case .light: "Light"
        case .dark: "Dark"
        }
    }
}

public enum AppError: Error, Equatable, Sendable {
    case network
    case unauthorized
    case invalidCredentials
    case emailNotVerified
    case emailAlreadyRegistered
    case validation(String?)
    case server(String?)
    case unknown

    public var userMessage: String {
        switch self {
        case .network:
            "The API cannot be reached. Check your connection and try again."
        case .unauthorized, .invalidCredentials:
            "The email or password is incorrect."
        case .emailNotVerified:
            "Verify your email before signing in."
        case .emailAlreadyRegistered:
            "An account with that email already exists."
        case .validation(let message):
            message ?? "Check the information and try again."
        case .server(let message):
            message ?? "The request could not be completed."
        case .unknown:
            "Something went wrong. Try again."
        }
    }
}

public struct AppInfo: Equatable, Sendable {
    public let environment: String
    public let apiBaseURL: String
    public let versionName: String

    public init(environment: String, apiBaseURL: String, versionName: String) {
        self.environment = environment
        self.apiBaseURL = apiBaseURL
        self.versionName = versionName
    }
}
