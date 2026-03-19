# spotify-adblock
Spotify adblocker for Linux (macOS untested) that works by wrapping `getaddrinfo` and `cef_urlrequest_create`. It blocks requests to domains that are not on the allowlist, as well as URLs that are on the denylist.

### Notes
* This **does not** work with the snap Spotify package.
* This **might not** work with the Flatpak Spotify package, depending on your system's shared libraries' versions.
* On Debian-based distributions (e.g. Ubuntu), the Debian Spotify package can be installed by following the instructions at the bottom of [this page](https://www.spotify.com/us/download/linux/). *(recommended)*

## Build
Prerequisites:
* Git
* Make
* Rust
* [Cargo](https://doc.rust-lang.org/cargo/)

```bash
$ git clone https://github.com/abba23/spotify-adblock.git
$ cd spotify-adblock
$ make
```

## Install
```bash
$ sudo make install
```

#### Install (Flatpak)
```bash
$ mkdir -p ~/.var/app/com.spotify.Client/config/spotify-adblock
$ cp target/release/libspotifyadblock.so ~/.var/app/com.spotify.Client
$ cp config.toml ~/.var/app/com.spotify.Client/config/spotify-adblock
$ flatpak --user override "--env=LD_PRELOAD=$HOME/.var/app/com.spotify.Client/libspotifyadblock.so" com.spotify.Client
```

## Usage

### Flatpak

No extra configuration needed, just launch the app.

### Command-line
```bash
$ LD_PRELOAD=/usr/local/lib/spotify-adblock.so spotify
```

### Desktop file
You can integrate it with your desktop environment by creating a `.desktop` file (e.g. `spotify-adblock.desktop`) in `~/.local/share/applications`. This lets you easily run it from an application launcher without opening a terminal.

Examples:

<details> 
  <summary>Debian Package</summary>
  <p>

```ini
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
```bash
$ sudo make uninstall
```

#### Uninstall (Flatpak)
```bash
$ flatpak --user override --unset-env=LD_PRELOAD com.spotify.Client
$ rm -rv ~/.var/app/com.spotify.Client/{config/spotify-adblock,libspotifyadblock.so}
```

## Configuration
The allowlist and denylist can be configured in a config file located at (in descending order of precedence):
* `config.toml` in the working directory
* `$XDG_CONFIG_HOME/spotify-adblock/config.toml`
* `~/.config/spotify-adblock/config.toml`
* `~/.var/app/com.spotify.Client/config/spotify-adblock/config.toml` *(Flatpak)*
* `/etc/spotify-adblock/config.toml` *(default)*
