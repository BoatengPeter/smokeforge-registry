import Foundation

public struct StoredTokens: Equatable, Sendable {
    public let accessToken: String
    public let refreshToken: String

    public init(accessToken: String, refreshToken: String) {
        self.accessToken = accessToken
        self.refreshToken = refreshToken
    }
}

public protocol TokenStore: AnyObject {
    func read() throws -> StoredTokens?
    func save(_ tokens: StoredTokens) throws
    func clear() throws
}
