plugins {
    alias(libs.plugins.kotlin.multiplatform)
}

group = "org.example"
version = "unspecified"

repositories {
    mavenCentral()
}

kotlin {
    macosX64() {
        binaries {
            executable()
        }
    }
    macosArm64() {
        binaries {
            executable()
        }
    }

    sourceSets {
        val commonMain by sourceSets.getting {
            dependencies {
                implementation(project(":lib"))
            }
        }
        val appleMain by creating {
        }
    }
}
