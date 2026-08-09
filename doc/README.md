# d77run (developer notes)

This is the technical/developer-facing README — status checklist, build details, packaging
internals. If you just want to *use* d77run, see the [root README](../README.md) instead; this
one assumes you're building or changing the code.

A from-scratch, minimal GTK4 launcher inspired by [gmrun](http://sourceforge.net/projects/gmrun),
built as the `d77run` binary.

The original gmrun is a small, fast "run dialog": type a command, hit Enter, it runs. It hasn't
seen much upkeep in years, still leans on GTK2, and its `.desktop`-file icon handling never
really worked. This project keeps gmrun's core idea — small window, direct command execution,
minimal ceremony — while rebuilding it clean:

- **Modern GTK.** Written against GTK4 (`gtk4-rs`) instead of gmrun's GTK2, so it builds and
  themes correctly on current systems.
- **Icons for `/usr/share/applications` entries.** Resolves `Icon=` from `.desktop` files through
  GTK4's own icon-theme lookup (`IconTheme::lookup_icon`), which follows the icon theme spec's
  fallback chain properly — this is the part that didn't work in the original gmrun.
- **Direct command execution kept.** Pressing Enter with nothing selected from the results list
  still runs whatever you typed as a raw shell command (`sh -c "<input>"`), unchanged from gmrun's
  core behaviour.

> The previous project that used to live in this repository — **d77-launcher**, a Rust/iced fork
> of [onagre](https://github.com/oknozor/onagre) — has moved to its own repository,
> [dani-77/d77-launcher](https://github.com/dani-77/d77-launcher).

## Status

**v0.3.0.** Functional and manually verified on both a real X11 display (Xvfb + xdotool) and a
real Wayland session.

- [x] Scans `$XDG_DATA_HOME/applications`, `$XDG_DATA_DIRS/applications` (or the
      `/usr/local/share` + `/usr/share` default), recursively, parsing `.desktop` files
      (skips `NoDisplay`/`Hidden`, ignores `[Desktop Action ...]` sub-sections, strips
      field codes like `%f`/`%U` from `Exec=`).
- [x] Filters the list live as you type, showing icon + name rows, ranked best match first: exact
      match, then name-prefix match, then word-boundary match, then any other substring match —
      ties keep their existing alphabetical order.
- [x] Up/Down moves the selection in the results list (focus stays in the entry), or — once the
      list isn't showing any matches — walks persistent command history instead, ordered by
      frecency (a frequency+recency blend, à la Firefox's URL bar) rather than strict
      most-recent-first, so a frequently-used command doesn't get buried by a single more recent
      one-off. Every launched command/app is appended to `$XDG_DATA_HOME/d77run/history`.
- [x] Enter launches the selected row's `Exec=`, or falls back to raw shell exec (`sh -c
      "<input>"`) when nothing's selected.
- [x] Tab completes the entry: to the top application match's name (also selecting it in the
      results list, so Enter fires immediately); or, if nothing matches an application, against
      `$PATH` executables and filesystem paths (absolute `/...` or home-relative `~/...` — there's
      no meaningful "current directory" to resolve a bare relative path against from inside the
      launcher) instead, directories completing with a trailing `/` so completion can continue
      into them. A single match completes outright; an ambiguous one fills in the longest common
      prefix *and* lists every actual candidate below the entry (e.g. `xdg-us` → Tab lists
      `xdg-user-dir` and `xdg-user-dirs-update`), navigable with Up/Down — picking one only
      completes the text, it doesn't run anything by itself.
- [x] Escape quits.
- [x] Packaging: `PKGBUILD` for Arch, `xbps-src` template for Void (see [Packaging](#packaging)).
- [ ] Not yet done: match ranking is still plain scoring tiers, not a proper fuzzy matcher (e.g.
      no credit for scattered-but-ordered characters like `ffx` → "Firefox").

## Build & run

```bash
cargo build --release
cargo test
cargo run
# or, once built:
sudo mv target/release/d77run /usr/bin/d77run
```

Requires GTK4 dev headers to build (`libgtk-4-dev` on Debian/Ubuntu, `gtk4-devel` on Fedora,
`gtk4` on Arch).

## Packaging

### Arch

A `PKGBUILD` is included, building straight from the working tree (no source tarball fetch):

```bash
makepkg -si
```

### Void Linux

An `xbps-src` template lives under `void/srcpkgs/d77run/` — see
[`void/README.md`](../void/README.md) for how to drop it into a `void-packages` checkout and build
with `xbps-src`.

## License

MIT, see [LICENSE](../LICENSE). This is original code, not derived from gmrun's GPL-licensed
sources or from onagre.
