# Music Assistant Desktop App 🎶

The Desktop app for Music Assistant! This app, among other things, enables you to play music from Music Assistant directly to your computer! Enjoy 🎵

> [!IMPORTANT]
> This is still in very early alpha. Bugs *will* be present. Please help finding them, you can report any bugs on the [Discord server](https://discord.gg/kaVm8hGpne) or in the [repo issues](https://github.com/music-assistant/music-assistant-desktop/issues)

## Setup

When starting the app for the first time you are asked about some information about the Music Assistant Server. 
![image](https://github.com/Un10ck3d/massapp/assets/74015378/cb97aa3e-12d8-4992-bfc6-0b58cedb81da)

> [!IMPORTANT]
> The app requires that the webserver is exposed. You can set that in the settings:
> ![How to fix](https://raw.githubusercontent.com/music-assistant/music-assistant-desktop/main/cant_connect_error.gif)

## Features

### [Squeezelite](https://en.wikipedia.org/wiki/Squeezelite)

Squeezelite comes embedded into the application. This allows you playing music to your computer. The player name will be the same as your computer name. You can change the name in Music Assistant settings. You can also toggle if you wish to enable squeezelite at all.

### [Discord Rich Presence](https://discord.com/developers/docs/rich-presence/how-to#so-what-is-it)

Like the Spotify app, the Music Assistant app can do Discord Rich Presence.

Example of Discord Rich Presence:

![Example of Discord Rich Presence](https://github.com/Un10ck3d/massapp/assets/74015378/8de18bac-b963-4aba-bb61-5730b41759a9)

> [!INFO]
> Only on Windows:
>
> - There seems to be a layout issue with the sidebar on windows
> - Untill [this PR](https://github.com/tauri-apps/wry/pull/994) gets merged and released the app runs the frontend openly on your computer on port 22863. The reason behind that is that webkit2 in windows dosnt allow connections to unsecured websockets if the app itself is secured with TLS..

## Installation

### Arch Linux

This app is on the arch aur with the name `music-assistant-desktop` or `music-assistant-desktop-bin` for just the binary

You can install it with yay: `yay music-assistant-desktop-bin`

### Debian (And debian based distrobutions)

You can download the .deb from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

### All the other linux distros

You can download the AppImage from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

### MacOS

You can download the .dmg from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

> [!IMPORTANT]
>The app is unsigned since we currently dont want to spend 90 USD/yr for an Apple Developer account. Therefore it will warn you about an untrusted developer/app downloaded from internet. To open it follow [these instructions](https://support.apple.com/guide/mac-help/open-a-mac-app-from-an-unidentified-developer-mh40616/mac) 

### Windows

You can download the .msi installer from the [releases](https://github.com/Un10ck3d/massapp/releases/latest/).

### From source

If you wish to build the app yourself you should first follow [the offical tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

Then clone the repository and install the node dependencies

```bash
$ git clone https://github.com/music-assistant/music-assistant-desktop --recursive
$ cd music-assistant-desktop
$ yarn install
$ cd frontend-source
$ yarn install
$ cd ..
```

And then build the app

`$ npx tauri build`
