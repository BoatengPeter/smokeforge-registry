import Core
import Foundation
import Networking
import Storage

@MainActor
public final class AuthRepositoryImpl: AuthRepository {
    private let api: APIClient
    private let tokens: TokenStore
    private let session: SessionStore

    public init(api: APIClient, tokens: TokenStore, session: SessionStore) {
        self.api = api
        self.tokens = tokens
        self.session = session
    }

    public func restore() async {
        let stored: StoredTokens?
        do {
            stored = try tokens.read()
        } catch {
            session.set(.signedOut)
            return
        }

        guard let stored else {
            session.set(.signedOut)
            return
        }

        do {
            let user = try await api.me(accessToken: stored.accessToken)
            session.set(.signedIn(user))
        } catch {
            guard let refreshed = try? await refresh(stored) else {
                try? tokens.clear()
                session.set(.signedOut)
                return
            }

            do {
                let user = try await api.me(accessToken: refreshed.accessToken)
                session.set(.signedIn(user))
            } catch {
                try? tokens.clear()
                session.set(.signedOut)
            }
        }
    }

    public func login(email: String, password: String) async throws {
        do {
            let issued = try await api.login(email: email.trimmingCharacters(in: .whitespacesAndNewlines), password: password)
            let stored = StoredTokens(accessToken: issued.accessToken, refreshToken: issued.refreshToken)
            try tokens.save(stored)
            let user = try await api.me(accessToken: stored.accessToken)
            session.set(.signedIn(user))
        } catch let error as APIError {
            throw map(error, login: true)
        } catch {
            throw AppError.unknown
        }
    }

    public func register(email: String, password: String) async throws {
        do {
            _ = try await api.register(
                email: email.trimmingCharacters(in: .whitespacesAndNewlines),
                password: password,
            )
        } catch let error as APIError {
            throw map(error, login: false)
        } catch {
            throw AppError.unknown
        }
    }

    public func requestPasswordReset(email: String) async throws {
        do {
            try await api.requestPasswordReset(email: email.trimmingCharacters(in: .whitespacesAndNewlines))
        } catch let error as APIError {
            throw map(error, login: false)
        } catch {
            throw AppError.unknown
        }
    }

    public func logout() async {
        if let stored = try? tokens.read() {
            try? await api.logout(refreshToken: stored.refreshToken)
        }

        try? tokens.clear()
        session.set(.signedOut)
    }

    private func refresh(_ stored: StoredTokens) async throws -> StoredTokens {
        let issued = try await api.refresh(refreshToken: stored.refreshToken)
        let refreshed = StoredTokens(accessToken: issued.accessToken, refreshToken: issued.refreshToken)
        try tokens.save(refreshed)
        return refreshed
    }

    private func map(_ error: APIError, login: Bool) -> AppError {
        switch error {
        case .transport:
            return .network
        case .decoding:
            return .server(nil)
        case .http(let status, let problem):
            switch status {
            case 401:
                return login ? .invalidCredentials : .unauthorized
            case 403:
                return .emailNotVerified
            case 404:
                return .server(nil)
            case 409:
                return .emailAlreadyRegistered
            case 422:
                return .validation(problem?.errors.first?.message ?? problem?.detail)
            case 500...599:
                return .server(problem?.detail)
            default:
                return .validation(problem?.detail)
            }
        }
    }
}
