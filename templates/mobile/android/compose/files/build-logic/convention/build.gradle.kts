plugins {
    `kotlin-dsl`
}

group = "com.smoke.buildlogic"

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(17)
    }
}

dependencies {
    compileOnly("com.android.tools.build:gradle:9.5.2")
    compileOnly("org.jetbrains.kotlin:kotlin-gradle-plugin:2.2.10")
    compileOnly("org.jetbrains.kotlin:compose-compiler-gradle-plugin:2.2.10")
    compileOnly("org.jetbrains.kotlin:kotlin-serialization:2.2.10")
    compileOnly("com.google.devtools.ksp:symbol-processing-gradle-plugin:2.3.14")
    compileOnly("com.google.dagger:hilt-android-gradle-plugin:2.60.1")
}
