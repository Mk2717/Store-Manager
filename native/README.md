# C.Flex native application release kit

This Tauri 2 shell opens the production C.Flex Store Manager in a dedicated,
branded application window. It uses the same server-side authentication,
two-factor verification and cloud records as the web/PWA installation.

## Desktop packages

The `native-desktop.yml` workflow builds these independently on the matching
operating system:

- Windows: `.msi` and `.exe`
- Linux: `.deb` and `.AppImage`
- macOS: `.dmg` and `.app`

The workflow's artifacts are unsigned test builds until the Windows and Apple
signing certificates are added. Linux packages do not require a store account.

## Mobile packages

Tauri can initialize Android and iOS projects from this same shell:

```sh
cd native
cargo tauri android init
cargo tauri ios init
```

Android release APK/AAB files require an Android keystore. iPhone/iPad builds
require an Apple Developer account, provisioning profile and signing
certificate. Unsigned or simulator-only builds must not be presented to store
staff as production installers.

### Signed iPhone / iPad IPA

The `ios-signed` job runs only from **Actions → Build native mobile packages →
Run workflow**. It never uses an Apple ID password. Before running it, create an
`ios-production` GitHub environment and add these encrypted secrets:

- `APPLE_TEAM_ID`: Apple Developer Team ID.
- `APPLE_API_KEY_ID`: App Store Connect API key ID.
- `APPLE_API_ISSUER`: App Store Connect issuer ID.
- `APPLE_API_PRIVATE_KEY_BASE64`: base64 content of the downloaded `.p8` key.
- `IOS_CERTIFICATE_P12_BASE64`: base64 content of the Apple Distribution `.p12` certificate.
- `IOS_CERTIFICATE_PASSWORD`: password used when exporting the `.p12` certificate.
- `IOS_PROVISIONING_PROFILE_BASE64`: base64 content of the App Store profile for `com.cflex.storemanager`.

Encode each file without adding line breaks:

```sh
base64 < AuthKey_KEYID.p8 | tr -d '\n'
base64 < CFlex_Distribution.p12 | tr -d '\n'
base64 < CFlex_AppStore.mobileprovision | tr -d '\n'
```

The job creates a temporary keychain, builds the signed IPA, retains the private
GitHub artifact for 14 days, and removes the certificate, profile, API key and
keychain even if the build fails.

## Local desktop build

Install the official Tauri 2 system prerequisites, Rust and Tauri CLI, then run:

```sh
cd native
cargo tauri icon ../public/cflex-logo.png
cargo tauri build
```

The application contains no inventory seed data. Store records load only after
an authorized user signs into the live private store.
