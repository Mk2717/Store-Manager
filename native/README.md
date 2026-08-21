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

## Local desktop build

Install the official Tauri 2 system prerequisites, Rust and Tauri CLI, then run:

```sh
cd native
cargo tauri icon ../public/cflex-logo.png
cargo tauri build
```

The application contains no inventory seed data. Store records load only after
an authorized user signs into the live private store.

