import Core
import Foundation

public struct AuthTokens: Equatable, Sendable {
    public let accessToken: String
    public let refreshToken: String

    public init(accessToken: String, refreshToken: String) {
        self.accessToken = accessToken
        self.refreshToken = refreshToken
    }
}

public struct ProblemDetails: Codable, Equatable, Sendable {
    public let title: String?
    public let status: Int?
    public let detail: String?
    public let code: String?
    public let errors: [FieldError]

    public init(title: String? = nil, status: Int? = nil, detail: String? = nil, code: String? = nil, errors: [FieldError] = []) {
        self.title = title
        self.status = status
        self.detail = detail
        self.code = code
        self.errors = errors
    }
}

public struct FieldError: Codable, Equatable, Sendable {
    public let field: String
    public let message: String

    public init(field: String, message: String) {
        self.field = field
        self.message = message
    }
}

public enum APIError: Error, Equatable, Sendable {
    case transport
    case http(status: Int, problem: ProblemDetails?)
    case decoding
}

struct TokenResponse: Decodable {
    let accessToken: String
    let refreshToken: String?

    enum CodingKeys: String, CodingKey {
        case accessToken = "access_token"
        case refreshToken = "refresh_token"
    }
}

struct UserResponse: Decodable {
    let id: String
    let email: String
    let role: String
    let emailVerified: Bool
    let createdAt: String

    enum CodingKeys: String, CodingKey {
        case id, email, role
        case emailVerified = "email_verified"
        case createdAt = "created_at"
    }

    func domain() -> User {
        User(id: id, email: email, role: role, emailVerified: emailVerified, createdAt: createdAt)
    }
}

struct PingResponse: Decodable {
    let message: String
}

struct LoginRequest: Encodable {
    let email: String
    let password: String
}

struct RegisterRequest: Encodable {
    let email: String
    let password: String
}

struct RefreshRequest: Encodable {
    let refreshToken: String

    enum CodingKeys: String, CodingKey {
        case refreshToken = "refresh_token"
    }
}

struct ForgotPasswordRequest: Encodable {
    let email: String
}
