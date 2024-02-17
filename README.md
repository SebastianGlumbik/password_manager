# Password Manager
This project was created as my **bachelor thesis** at VŠB - Technical University of Ostrava.
It is a simple password manager application for macOS and Linux which allows you to store your passwords and other sensitive data.
I gave my best effort to make this application secure. Data are stored in an encrypted database and can be synchronized with a custom SFTP server.

<p align="center">
  <img alt="Password Manager" src="Password Manager.png">
</p>

## Features
- **Focus on security**
    - Storing data securely using [sqlcipher](https://github.com/sqlcipher/sqlcipher).
    - Best effort to clear all sensitive data from memory after usage.
    - Data are sent to the front-end at the very last moment when they are requested.
- **Native macOS look**
  - Design was inspired by macOS.
- **Light and dark mode**
    - Supports both light and dark mode<sup>1</sup>.
- **Synchronization**
    - Data can be synchronized with custom SFTP server<sup>2</sup>.
- **Password generator**
    - Generate strong passwords.
- **Checks if your password was exposed in a data breach**
    - Once per 24 hours, passwords are checked with [haveibeenpwned](https://haveibeenpwned.com/Passwords) API.
- **TOTP codes**
    - Generate TOTP codes for 2FA.

## Build guide
### Prerequisites
To build this application you need to have installed:
- **Rust (cargo)**
  - https://www.rust-lang.org/tools/install
- **Node.js (npm)**
  - https://nodejs.org/en/download/

#### macOS
On macOS, you need to have installed Xcode Command Line Tools. 
You can install them by running `xcode-select --install` in terminal.

#### Linux
On Linux you need to install a couple of system dependencies.
For **Ubuntu**, you can install them by running:
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
   1. ```.deb``` - Debian-based Linux
   2. ```.AppImage``` - Linux
   3. ```.dmg``` and ```.app``` - macOS

#### Notes
- This project was configured for macOS and Linux. **Windows is not supported.**
  - Tested on macOS 14 and Ubuntu 22.04.
- (macOS) You may need to give Terminal.app the permissions to control Finder.app in ```System-Settings``` -> ```Privacy & Security``` -> ```Automation```, otherwise the build will fail.
- If you want to run the application in development mode, run `npm run tauri dev` instead of `npm run tauri build`.
- Be sure to have the latest version of Rust and Node.js installed.
- If you have any problems with building the application, please make sure that you have installed all the prerequisites. For more information refer to https://tauri.app/v1/guides/getting-started/prerequisites and https://tauri.app/v1/guides/building/

## Sources
- List of Rust crates you can find in [Cargo.toml](src-tauri/Cargo.toml) file.
- List of JavaScript packages you can find in [package.json](package.json) file.
- Front-end built with [Tauri](https://tauri.app/), [Solid](https://www.solidjs.com) and [Tailwind CSS](https://tailwindcss.com).
- Encryption of SQLite database - [sqlcipher](https://github.com/sqlcipher/sqlcipher).
- Passwords are checked with [haveibeenpwned](https://haveibeenpwned.com/Passwords) API.
- Main icon (lock) - [Flaticon](https://www.flaticon.com/free-icon/lock_526812?term=password&page=1&position=28&origin=search&related_id=526812)
- Other icons (svg) - [Font Awesome](https://fontawesome.com/)
- Roboto font - [Google Fonts](https://fonts.google.com/specimen/Roboto/)

---
[1] Light/dark mode depends on system settings.<br>
[2] Does not support multiple application instances at the same time.