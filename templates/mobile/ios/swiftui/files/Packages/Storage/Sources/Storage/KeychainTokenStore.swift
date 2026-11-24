import Foundation
import Security

public enum KeychainError: Error, Equatable {
    case unexpectedStatus(OSStatus)
    case invalidData
}

public final class KeychainTokenStore: TokenStore {
    private let service: String
    private let account = "session.tokens"

    public init(service: String = "com.smokeforge.session") {
        self.service = service
    }

    public func read() throws -> StoredTokens? {
        var query = baseQuery()
        query[kSecReturnData as String] = true
        query[kSecMatchLimit as String] = kSecMatchLimitOne

        var item: CFTypeRef?
        let status = SecItemCopyMatching(query as CFDictionary, &item)

        if status == errSecItemNotFound {
            return nil
        }

        guard status == errSecSuccess else {
            throw KeychainError.unexpectedStatus(status)
        }

        guard let data = item as? Data,
              let text = String(data: data, encoding: .utf8)
        else {
            throw KeychainError.invalidData
        }

        let parts = text.split(separator: "\n", omittingEmptySubsequences: false)
        guard parts.count == 2 else {
            throw KeychainError.invalidData
        }

        return StoredTokens(accessToken: String(parts[0]), refreshToken: String(parts[1]))
    }

    public func save(_ tokens: StoredTokens) throws {
        try clear()

        var query = baseQuery()
        query[kSecValueData as String] = Data("\(tokens.accessToken)\n\(tokens.refreshToken)".utf8)
        query[kSecAttrAccessible as String] = kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly

        let status = SecItemAdd(query as CFDictionary, nil)
        guard status == errSecSuccess else {
            throw KeychainError.unexpectedStatus(status)
        }
    }

    public func clear() throws {
        let status = SecItemDelete(baseQuery() as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw KeychainError.unexpectedStatus(status)
        }
    }

    private func baseQuery() -> [String: Any] {
        [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account,
        ]
    }
}
