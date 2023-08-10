# Music Assistant Desktop App

## WARNING: This is still in very early alpha. Bugs *will* be present. Please help finding them. You can report it on the Discord server

## The app requires that the webserver is exposed. You can set that in the settings:
![image](https://github.com/Un10ck3d/massapp/assets/74015378/8ea0b53a-e2a5-42c2-a98b-d04fcbe591bc)

Right now the setup thing will always show upon opening. But it shuold save your choice from previous session.
![image](https://github.com/Un10ck3d/massapp/assets/74015378/cb97aa3e-12d8-4992-bfc6-0b58cedb81da)

[Squeezelite](https://en.wikipedia.org/wiki/Squeezelite) comes embedded into the application. This allows you playing music to your computer. The player name will be the same as your computer name. You can change the name in Music Assistant settings. You can also toggle if you wish to enable squeezelite at all.

The app can also do Discord Rich Presence. Meaning it will show on discord what music you are playing. It only shows the music playing on the app's squeezelite player. Example:
![image](https://github.com/Un10ck3d/massapp/assets/74015378/8de18bac-b963-4aba-bb61-5730b41759a9)

## Notice: (WINDOWS ONLY!)
- There seems to be a layout issue with the sidebar on windows
- There seems to be a discord rpc issue on windows
- Untill [this PR](https://github.com/tauri-apps/wry/pull/994) gets merged and released the app runs the frontend openly on your computer on port 22863. The reason behind that is that webkit2 in windows dosnt allow connections to unsecured websockets if the app itself is secured with TLS..

## Installation

### Linux

#### Arch Linux

This app is on the arch aur with the name `music-assistant-desktop`

You can install it with yay: `yay music-assistant-desktop`

#### Debian based distros

You can download the .deb from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

#### All the other distros

You can download the AppImage from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

### MacOS

You can download the .dmg from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/). This build is universal for both Intel and Apple Silicon computers

### Windows

You can download the .msi installer from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).
