import Core
import Foundation

struct AppConfig {
    let baseURL: URL
    let info: AppInfo

    static func from(bundle: Bundle = .main) -> AppConfig {
        let environment = bundle.object(forInfoDictionaryKey: "AppEnvironment") as? String ?? "release"
        let version = bundle.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String ?? "1.0"
        let rawBaseURL = bundle.object(forInfoDictionaryKey: "APIBaseURL") as? String ?? "https://api.example.com"
        let base = URL(string: rawBaseURL) ?? URL(string: "https://api.example.com")!

        return AppConfig(
            baseURL: base,
            info: AppInfo(
                environment: environment,
                apiBaseURL: base.absoluteString,
                versionName: version,
            ),
        )
    }
}
