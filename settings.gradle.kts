rootProject.name = "crossterm-kmp"

pluginManagement {
    repositories {
        mavenCentral()
        gradlePluginPortal()
    }
}

include(":lib")
include("samples:input")
include("samples:mosaic")
