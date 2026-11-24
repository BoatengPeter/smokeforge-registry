import Core
import Foundation

public actor APIClient {
    private let baseURL: URL
    private let session: URLSession
    private let encoder = JSONEncoder()
    private let decoder = JSONDecoder()

    public init(baseURL: URL, session: URLSession = .shared) {
        var normalized = baseURL
        if !normalized.absoluteString.hasSuffix("/") {
            normalized = URL(string: normalized.absoluteString + "/")!
        }
        self.baseURL = normalized
        self.session = session
    }

    public func login(email: String, password: String) async throws -> AuthTokens {
        let data = try await send(
            path: "api/v1/auth/login",
            method: "POST",
            body: try encoded(LoginRequest(email: email, password: password)),
            extraHeaders: ["X-Refresh-Token-Transport": "body"],
        )
        return try tokens(from: data)
    }

    public func register(email: String, password: String) async throws -> User {
        let data = try await send(
            path: "api/v1/auth/register",
            method: "POST",
            body: try encoded(RegisterRequest(email: email, password: password)),
        )
        guard let response = try? decoder.decode(UserResponse.self, from: data) else {
            throw APIError.decoding
        }
        return response.domain()
    }

    public func refresh(refreshToken: String) async throws -> AuthTokens {
        let data = try await send(
            path: "api/v1/auth/refresh",
            method: "POST",
            body: try encoded(RefreshRequest(refreshToken: refreshToken)),
            extraHeaders: ["X-Refresh-Token-Transport": "body"],
        )
        return try tokens(from: data)
    }

    public func me(accessToken: String) async throws -> User {
        let data = try await send(
            path: "api/v1/auth/me",
            method: "GET",
            accessToken: accessToken,
        )
        guard let response = try? decoder.decode(UserResponse.self, from: data) else {
            throw APIError.decoding
        }
        return response.domain()
    }

    public func logout(refreshToken: String) async throws {
        _ = try await send(
            path: "api/v1/auth/logout",
            method: "POST",
            body: try encoded(RefreshRequest(refreshToken: refreshToken)),
        )
    }

    public func requestPasswordReset(email: String) async throws {
        _ = try await send(
            path: "api/v1/auth/forgot-password",
            method: "POST",
            body: try encoded(ForgotPasswordRequest(email: email)),
        )
    }

    public func ping() async throws -> String {
        let data = try await send(path: "api/v1/ping", method: "GET")
        guard let response = try? decoder.decode(PingResponse.self, from: data) else {
            throw APIError.decoding
        }
        return response.message
    }

    private func encoded<T: Encodable>(_ value: T) throws -> Data {
        try encoder.encode(value)
    }

    private func tokens(from data: Data) throws -> AuthTokens {
        guard let response = try? decoder.decode(TokenResponse.self, from: data),
              let refresh = response.refreshToken
        else {
            throw APIError.decoding
        }
        return AuthTokens(accessToken: response.accessToken, refreshToken: refresh)
    }

    private func send(
        path: String,
        method: String,
        body: Data? = nil,
        accessToken: String? = nil,
        extraHeaders: [String: String] = [:],
    ) async throws -> Data {
        guard let url = URL(string: path, relativeTo: baseURL)?.absoluteURL else {
            throw APIError.transport
        }

        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue("application/json", forHTTPHeaderField: "Accept")

        if let accessToken {
            request.setValue("Bearer \(accessToken)", forHTTPHeaderField: "Authorization")
        }

        for (key, value) in extraHeaders {
            request.setValue(value, forHTTPHeaderField: key)
        }

        if let body {
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
            request.httpBody = body
        }

        let data: Data
        let response: URLResponse

        do {
            (data, response) = try await session.data(for: request)
        } catch {
            throw APIError.transport
        }

        guard let http = response as? HTTPURLResponse else {
            throw APIError.transport
        }

        guard (200..<300).contains(http.statusCode) else {
            let problem = try? decoder.decode(ProblemDetails.self, from: data)
            throw APIError.http(status: http.statusCode, problem: problem)
        }

        return data
    }
}
