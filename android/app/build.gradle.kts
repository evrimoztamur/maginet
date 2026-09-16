plugins { id("com.android.application"); id("org.jetbrains.kotlin.android") }
val playPublicKey = providers.environmentVariable("MAGINET_PLAY_PUBLIC_KEY").getOrElse("").filterNot { it.isWhitespace() }
require(playPublicKey.matches(Regex("[A-Za-z0-9+/=]*"))) { "MAGINET_PLAY_PUBLIC_KEY must contain only Base64 public-key data" }
val releaseVersionCode = providers.environmentVariable("MAGINET_VERSION_CODE").getOrElse("1").toInt()
require(releaseVersionCode in 1..2_100_000_000) { "MAGINET_VERSION_CODE is outside the Play version-code range" }
android {
    namespace = "zone.evrim.maginet"
    compileSdk = 36
    defaultConfig {
        applicationId = "zone.evrim.maginet"
        minSdk = 26
        targetSdk = 36
        versionCode = releaseVersionCode
        versionName = providers.environmentVariable("MAGINET_VERSION_NAME").getOrElse("1.0")
        buildConfigField("String", "PLAY_PUBLIC_KEY", "\"$playPublicKey\"")
    }
    signingConfigs {
        if (System.getenv("MAGINET_KEYSTORE") != null) create("release") {
            storeFile = file(System.getenv("MAGINET_KEYSTORE"))
            storePassword = System.getenv("MAGINET_STORE_PASSWORD")
            keyAlias = System.getenv("MAGINET_KEY_ALIAS")
            keyPassword = System.getenv("MAGINET_KEY_PASSWORD")
        }
    }
    buildTypes {
        debug { applicationIdSuffix = ".debug" }
        release { signingConfig = signingConfigs.findByName("release") }
    }
    buildFeatures { buildConfig = true }
    compileOptions { sourceCompatibility = JavaVersion.VERSION_17; targetCompatibility = JavaVersion.VERSION_17 }
    kotlinOptions { jvmTarget = "17" }
}
dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
    implementation("androidx.activity:activity-ktx:1.10.1")
    implementation("com.android.billingclient:billing:8.3.0")
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.json:json:20250517")
}
