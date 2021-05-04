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

```bash
$ git clone https://github.com/abba23/spotify-adblock.git
$ cd spotify-adblock
$ make
```

## Install
```bash
$ sudo make install
```

#### Flatpak
```bash
$ mkdir -p ~/.spotify-adblock && cp target/release/libspotifyadblock.so ~/.spotify-adblock/spotify-adblock.so
$ mkdir -p ~/.config/spotify-adblock && cp config.toml ~/.config/spotify-adblock
$ flatpak override --user --filesystem="~/.spotify-adblock/spotify-adblock.so" --filesystem="~/.config/spotify-adblock/config.toml" com.spotify.Client
```

## Usage
### Command-line
```bash
$ LD_PRELOAD=/usr/local/lib/spotify-adblock.so spotify
```

#### Flatpak
```bash
$ flatpak run --command=sh com.spotify.Client -c 'eval "$(sed s#LD_PRELOAD=#LD_PRELOAD=$HOME/.spotify-adblock/spotify-adblock.so:#g /app/bin/spotify)"'
```

### Desktop file
You can integrate it with your desktop environment by creating a `.desktop` file (e.g. `spotify-adblock.desktop`) in `~/.local/share/applications`. This lets you easily run it from an application launcher without opening a terminal.

Examples:

<details> 
  <summary>Debian Package</summary>
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

<details>
  <summary>Flatpak</summary>
  <p>

```
[Desktop Entry]
Type=Application
Name=Spotify (adblock)
GenericName=Music Player
Icon=com.spotify.Client
Exec=flatpak run --file-forwarding --command=sh com.spotify.Client -c 'eval "$(sed s#LD_PRELOAD=#LD_PRELOAD=$HOME/.spotify-adblock/spotify-adblock.so:#g /app/bin/spotify)"' @@u %U @@
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

#### Flatpak
```bash
$ rm -r ~/.spotify-adblock ~/.config/spotify-adblock
$ flatpak override --user --reset com.spotify.Client
```

## Configuration
The allowlist and denylist can be configured in a config file located at (in ascending order of precedence):
* `/etc/spotify-adblock/config.toml` *(default)*
* `~/.config/spotify-adblock/config.toml` *(default for Flatpak)*
* `config.toml` in the working directory
