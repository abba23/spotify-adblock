# spotify-adblock
Spotify adblocker for Linux that works by wrapping `cef_urlrequest_create` and blocking requests to URLs on a denylist.

### Notes
* This **does not** work with the snap Spotify package.
* This **might not** work with the Flatpak Spotify package, depending on your system's shared libraries' versions.
* On Debian-based distributions (e.g. Ubuntu), the Debian Spotify package can be installed by following the instructions at the bottom of [this page](https://www.spotify.com/us/download/linux/). *(recommended)*

## Build
Prerequisites:
* Git
* Make
* [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

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
$ mkdir -p ~/.var/app/com.spotify.Client/config/spotify-adblock && cp config.toml ~/.var/app/com.spotify.Client/config/spotify-adblock
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

```bash
$ cp spotify-adblock-deb.desktop ~/.local/share/applications
$ cp spotify-adblock-flatpack.desktop ~/.local/share/applications
```

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
The allowlist and denylist are configured in a config file located at `/etc/spotify-adblock/config.toml`.
It can be overriden with a user-specific version at `$XDG_CONFIG_HOME/spotify-adblock/config.toml` or `$HOME/.config/spotify-adblock/config.toml`.