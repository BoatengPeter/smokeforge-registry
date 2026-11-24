import Core
import TestSupport
import Testing
@testable import AuthFeature

@MainActor
struct AuthViewModelTests {
    @Test
    func successfulLoginTrimsEmailAndClearsPassword() async {
        let repository = FakeAuthRepository()
        let model = AuthViewModel(repository: repository)
        model.email = " person@example.com "
        model.password = "correct horse battery staple"

        await model.submit()

        #expect(repository.lastLogin?.email == "person@example.com")
        #expect(model.password.isEmpty)
        #expect(model.error == nil)
    }

    @Test
    func invalidCredentialsExposeAStableError() async {
        let repository = FakeAuthRepository()
        repository.loginError = .invalidCredentials
        let model = AuthViewModel(repository: repository)
        model.email = "person@example.com"
        model.password = "wrong"

        await model.submit()

        #expect(model.error == .invalidCredentials)
        #expect(model.isLoading == false)
    }

    @Test
    func registrationMovesBackToLoginWithNotice() async {
        let repository = FakeAuthRepository()
        let model = AuthViewModel(repository: repository)
        model.setMode(.register)
        model.email = "person@example.com"
        model.password = "a long enough password"
        model.confirmPassword = "a long enough password"

        await model.submit()

        #expect(model.mode == .login)
        #expect(model.notice == .accountCreated)
    }

    @Test
    func registrationRejectsMismatchedPasswords() async {
        let repository = FakeAuthRepository()
        let model = AuthViewModel(repository: repository)
        model.setMode(.register)
        model.email = "person@example.com"
        model.password = "a long enough password"
        model.confirmPassword = "different password"

        await model.submit()

        #expect(model.error == .validation("Passwords do not match."))
        #expect(repository.lastRegister == nil)
    }
}
