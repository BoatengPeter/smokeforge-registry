import Core

@MainActor
public final class FakeAuthRepository: AuthRepository {
    public var loginError: AppError?
    public var registerError: AppError?
    public var forgotError: AppError?
    public var lastLogin: (email: String, password: String)?
    public var lastRegister: (email: String, password: String)?

    public init() {}

    public func restore() async {}

    public func login(email: String, password: String) async throws {
        lastLogin = (email, password)
        if let loginError { throw loginError }
    }

    public func register(email: String, password: String) async throws {
        lastRegister = (email, password)
        if let registerError { throw registerError }
    }

    public func requestPasswordReset(email: String) async throws {
        if let forgotError { throw forgotError }
    }

    public func logout() async {}
}

@MainActor
public final class FakeSystemRepository: SystemRepository {
    public var pingValue = "pong"
    public var pingError: AppError?
    public var user = TestData.user

    public init() {}

    public func ping() async throws -> String {
        if let pingError { throw pingError }
        return pingValue
    }

    public func currentUser() async throws -> User {
        user
    }
}

public enum TestData {
    public static let user = User(
        id: "01934f5a-7d0f-7a61-9e0b-7b2f4b4b6a1e",
        email: "person@example.com",
        role: "user",
        emailVerified: true,
        createdAt: "2026-11-20T12:00:00Z",
    )
}
