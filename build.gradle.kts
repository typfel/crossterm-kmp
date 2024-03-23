buildscript {
    repositories {
        google()
        mavenCentral()
    }

    dependencies {
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:${libs.versions.kotlin.get()}")
        classpath("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.22.0")
        classpath("com.jakewharton.mosaic:mosaic-gradle-plugin:${libs.versions.mosaic.get()}")
    }
}

plugins {
    alias(libs.plugins.kotlin.multiplatform) apply false
}

allprojects {
    repositories {
        google()
        mavenCentral()
    }
}