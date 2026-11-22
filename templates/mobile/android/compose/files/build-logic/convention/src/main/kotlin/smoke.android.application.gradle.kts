import com.android.build.api.dsl.ApplicationExtension
import org.gradle.api.JavaVersion

plugins {
    id("com.android.application")
}

extensions.configure<ApplicationExtension> {
    compileSdk = 37
    compileSdkMinor = 1

    defaultConfig {
        minSdk = 26
        targetSdk = 37
        versionCode = 1
        versionName = "1.0"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    testOptions {
        unitTests {
            isIncludeAndroidResources = true
        }
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}
