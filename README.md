# Music Assistant Desktop App

## WARNING: This is still in very early alpha. Bugs *will* occur

Right now the setup thing will always show upon opening. But it shuold save your choice from previous session.
![image](https://github.com/Un10ck3d/massapp/assets/74015378/cb97aa3e-12d8-4992-bfc6-0b58cedb81da)

[Squeezelite](https://en.wikipedia.org/wiki/Squeezelite) comes embedded into the application. This allows you playing music to your computer. The player name will be the same as your computer name. You can change the name in Music Assistant settings. You can also toggle if you wish to enable squeezelite at all.

The app can also do Discord Rich Presence. Meaning it will show on discord what music you are playing. Example:
![image](https://github.com/Un10ck3d/massapp/assets/74015378/8de18bac-b963-4aba-bb61-5730b41759a9)

Notice:
Untill [this PR](https://github.com/tauri-apps/wry/pull/994) gets merged and released the app runs the frontend openly on your computer on port 22863. The reason behind that is that webkit2 in windows dosnt allow connections to unsecured websockets if the app itself is secured with TLS..

## Installation

### Linux

#### Arch Linux

You can use the PKGBUILD file to install the app

Clone the repo:
`$ git clone https://github.com/Un10ck3d/massapp`

Install the app:
`$ makepkg -si`

#### Debian based distros

You can download the .deb from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

#### All the other distros

You can download the AppImage from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

### MacOS

You can download the .dmg from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/). This build is universal for both Intel and Apple Silicon computers

### Windows

You can download the .msi installer from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).
