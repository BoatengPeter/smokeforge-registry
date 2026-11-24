import Foundation

@MainActor
public protocol AuthRepository: AnyObject {
    func restore() async
    func login(email: String, password: String) async throws
    func register(email: String, password: String) async throws
    func requestPasswordReset(email: String) async throws
    func logout() async
}

@MainActor
public protocol SystemRepository: AnyObject {
    func ping() async throws -> String
    func currentUser() async throws -> User
}
