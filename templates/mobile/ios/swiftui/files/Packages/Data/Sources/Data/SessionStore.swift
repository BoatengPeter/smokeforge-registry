import Core
import Observation

@MainActor
@Observable
public final class SessionStore {
    public private(set) var state: SessionState

    public init(state: SessionState = .loading) {
        self.state = state
    }

    func set(_ state: SessionState) {
        self.state = state
    }
}
