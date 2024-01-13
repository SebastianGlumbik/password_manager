# Password Manager
**Bachelor thesis**

## Build guide
### Prerequisites
To build this application you need to have installed:
- **Rust (cargo)**
  - https://www.rust-lang.org/tools/install
- **Node.js (npm)**
  - https://nodejs.org/en/download/

#### macOS
On macOS you need to have installed Xcode Command Line Tools. 
You can install them by running `xcode-select --install` in terminal.

#### Linux
On Linux you need to install a couple of system dependencies.
For **Ubuntu** you can install them by running:
```
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```
If you are using other Linux distribution please refer to https://tauri.app/v1/guides/getting-started/prerequisites/#setting-up-linux.

### Building
1. Clone this repository.
2. Move to repository directory.
3. Run `npm install` to install dependencies.
4. Run `npm run tauri build` to build the application.
5. Built application will be located in ```src-tauri/target/release/bundle```, where you can choose between:
   1. Your distribution package (```.deb``` for Ubuntu and ```.dmg``` for macOS) to install the application.
   2. ```.AppImage``` (Linux) or ```.app``` (macOS) to run the application.

#### Notes
- This project was configured for macOS and Linux. **Windows is not supported.**
  - Tested on macOS 14 and Ubuntu 22.04.
- (macOS) You may need to give Terminal.app the permissions to control Finder.app in ```System-Settings``` -> ```Privacy & Security``` -> ```Automation```, otherwise the build will fail.
- If you want to run the application in development mode, run `npm run tauri dev` instead of `npm run tauri build`.
- Be sure to have the latest version of Rust and Node.js installed.
- If you have any problems with building the application, please make sure that you have installed all the prerequisites. For more information refer to https://tauri.app/v1/guides/getting-started/prerequisites and https://tauri.app/v1/guides/building/

## Crates
Crates and their usage in this project:
- [rusqlite](https://crates.io/crates/rusqlite) - Storing data in SQLite database with build in [sqlcipher](https://github.com/sqlcipher/sqlcipher) for encryption.
- [zeroize](https://crates.io/crates/zeroize) - Securely zeros memory 
- [totp-rs](https://crates.io/crates/totp-rs) - Generating TOTP codes
- [passwords](https://crates.io/crates/passwords) - Generating passwords
- [chrono](https://crates.io/crates/chrono) - Storing time
- [secrecy](https://crates.io/crates/secrecy) - Storing secrets
- [regex](https://crates.io/crates/regex) - Validating input
- [sha1](https://crates.io/crates/sha1) - Hashing password for haveibeenpwned API
- [reqwest](https://crates.io/crates/reqwest) - Sending requests to haveibeenpwned API
- [tokio](https://crates.io/crates/tokio) - Async runtime for reqwest
- [card-validate](https://crates.io/crates/card-validate) - Validating credit card numbers

## Sources
- Main icon (lock) - [Flaticon](https://www.flaticon.com/free-icon/lock_526812?term=password&page=1&position=28&origin=search&related_id=526812)
- Icons for buttons - [Font Awesome](https://fontawesome.com/)
- Roboto font - [Google Fonts](https://fonts.google.com/specimen/Roboto/)

