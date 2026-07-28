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

- The template pins a specific commit of *this* repo (`_commit`, since
  there are no release tags yet). Bump it and recompute the checksum the
  same way whenever you want to package a newer revision.
- Its `checksum` is a placeholder — this template was authored without
  access to `xbps-src` to compute the real sha256. Run `./xbps-src pkg
  d77run` once; it fetches the tarball, fails on the checksum mismatch,
  and prints the real sha256 to paste in.
- `gtk4-devel`/`gtk4` and `pkg-config` are already packaged in
  void-packages, so `xbps-src`/`xbps-install` pull them in automatically.
