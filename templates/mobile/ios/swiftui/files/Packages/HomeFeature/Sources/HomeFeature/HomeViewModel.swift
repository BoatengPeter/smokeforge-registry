import Core
import Observation

@MainActor
@Observable
public final class HomeViewModel {
    public private(set) var user: User
    public private(set) var apiMessage: String?
    public private(set) var isLoading = true
    public private(set) var error: AppError?

    private let repository: any SystemRepository

    public init(repository: any SystemRepository, user: User) {
        self.repository = repository
        self.user = user
    }

    public func refresh() async {
        isLoading = true
        error = nil
        defer { isLoading = false }

        do {
            apiMessage = try await repository.ping()
            user = try await repository.currentUser()
        } catch let error as AppError {
            self.error = error
        } catch {
            error = .unknown
        }
    }
}
