# spotify-adblock
Spotify adblocker for Linux (macOS untested) that works by wrapping `getaddrinfo` and `cef_urlrequest_create`. It blocks requests to domains in the allowlist and URLs in the denylist.

### Notes
* This does not work with the snap and Flatpak Spotify packages.

## Build
Prerequisites:
* Git
* Make
* Rust


---

    $ git clone https://github.com/abba23/spotify-adblock.git
    $ cd spotify-adblock
    $ make

## Install
    $ sudo make install

## Usage

### Command-line
    $ LD_PRELOAD=/usr/local/lib/spotify-adblock.so spotify

### Desktop file
You can integrate it with your desktop environment by creating a `.desktop` file (e.g. `spotify-adblock.desktop`) in `~/.local/share/applications`. This lets you easily run it from an application launcher without opening a terminal.

<details> 
  <summary>Example</summary>
  <p>

```
[Desktop Entry]
Type=Application
Name=Spotify (adblock)
GenericName=Music Player
Icon=spotify-client
TryExec=spotify
Exec=env LD_PRELOAD=/usr/local/lib/spotify-adblock.so spotify %U
Terminal=false
MimeType=x-scheme-handler/spotify;
Categories=Audio;Music;Player;AudioVideo;
StartupWMClass=spotify
```
  </p>
</details>

## Uninstall
    $ sudo make uninstall


## Configuration
The allowlist and denylist can be configured in a config file located at `/etc/spotify-adblock/config.toml`, which can be overriden by (in ascending order of precedence):
* `~/.config/spotify-adblock/config.toml`
* `config.toml` in the working directory
