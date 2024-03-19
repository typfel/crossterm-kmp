plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.uniffi)
    alias(libs.plugins.maven.publish)
}

repositories {
    mavenCentral()
}

uniffi {
    // The directory containing the Rust crate.
    crateDirectory = layout.projectDirectory.dir("../crossterm-ffi")

    // The name of the crate as in Cargo.toml's package.name.
    crateName = "crossterm"

    // The name of the library as in Cargo.toml's library.name. Defaults to "${crateName}".
    libraryName = "crossterm"

    // The UDL file. Defaults to "${crateDirectory}/src/${crateName}.udl".
    // udlFile = layout.projectDirectory.file("rust/src/my_crate.udl")

    // The UDL namespace as in the UDL file. Defaults to "${crateName}".
    namespace = "crossterm"

    // The profile to build the crate. Defaults to "debug".
    profile = "debug"
}


kotlin {
    jvm {
        compilations.all {
            kotlinOptions.jvmTarget = "1.8"
        }
        testRuns["test"].executionTask.configure {
            useJUnitPlatform()
        }
    }
    linuxX64()
    macosArm64()
    macosX64()

    sourceSets {
        val commonTest by getting {
            dependencies {
                implementation(kotlin("test"))
            }
        }
    }
}

project.afterEvaluate {
    val buildBindings = tasks.getByName("buildBindings")
    tasks.withType(org.gradle.jvm.tasks.Jar::class.java) {
        dependsOn(buildBindings)
    }
}