# Void Linux packaging

An `xbps-src` template for installing `d77run` as a real system package on
Void Linux. Void has no AUR-style overlay, so using this means dropping the
template into a local checkout of `void-packages`.

## One-time setup

```
git clone --depth 1 https://github.com/void-linux/void-packages.git
cd void-packages
./xbps-src binary-bootstrap
```

## Add this template

From this repo:

```
cp -r void/srcpkgs/d77run /path/to/void-packages/srcpkgs/
```

(symlink instead of `cp` if you want `git pull` here to keep it in sync.)

## Build & install

```
cd /path/to/void-packages
./xbps-src pkg d77run
sudo xbps-install --repository=hostdir/binpkgs -R d77run
```

Run it with `d77run`, or bind it directly in your compositor/window-manager
config.

## Notes

- The template fetches the tagged release tarball (`refs/tags/v$version`).
  Bump `version` and recompute the checksum whenever you want to package a
  newer release, e.g.:
  `curl -sL https://github.com/dani-77/d77run/archive/refs/tags/v<version>.tar.gz | sha256sum`.
- `gtk4-devel`/`gtk4` and `pkg-config` are already packaged in
  void-packages, so `xbps-src`/`xbps-install` pull them in automatically.
