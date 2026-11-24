import Core
import Networking
import Storage

@MainActor
public final class SystemRepositoryImpl: SystemRepository {
    private let api: APIClient
    private let tokens: TokenStore

    public init(api: APIClient, tokens: TokenStore) {
        self.api = api
        self.tokens = tokens
    }

    public func ping() async throws -> String {
        do {
            return try await api.ping()
        } catch let error as APIError {
            throw map(error)
        } catch {
            throw AppError.unknown
        }
    }

    public func currentUser() async throws -> User {
        guard let stored = try? tokens.read() else {
            throw AppError.unauthorized
        }

        do {
            return try await api.me(accessToken: stored.accessToken)
        } catch APIError.http(let status, _) where status == 401 {
            do {
                let issued = try await api.refresh(refreshToken: stored.refreshToken)
                let refreshed = StoredTokens(accessToken: issued.accessToken, refreshToken: issued.refreshToken)
                try tokens.save(refreshed)
                return try await api.me(accessToken: refreshed.accessToken)
            } catch {
                throw AppError.unauthorized
            }
        } catch let error as APIError {
            throw map(error)
        } catch {
            throw AppError.unknown
        }
    }

    private func map(_ error: APIError) -> AppError {
        switch error {
        case .transport:
            return .network
        case .decoding:
            return .server(nil)
        case .http(let status, let problem):
            switch status {
            case 401:
                return .unauthorized
            case 403:
                return .emailNotVerified
            case 500...599:
                return .server(problem?.detail)
            default:
                return .validation(problem?.detail)
            }
        }
    }
}
