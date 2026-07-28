# gmrun-rejuvenated

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

MVP, functional and manually verified on a real X11 display (Xvfb + xdotool): typing filters the
list with correctly-resolved icons, Up/Down moves a visible selection, Enter on a selected row
spawns that app's `Exec=`, Enter with nothing selected runs the raw command, Escape quits without
running anything.

- [x] Scans `$XDG_DATA_HOME/applications`, `$XDG_DATA_DIRS/applications` (or the
      `/usr/local/share` + `/usr/share` default), recursively, parsing `.desktop` files
      (skips `NoDisplay`/`Hidden`, ignores `[Desktop Action ...]` sub-sections, strips
      field codes like `%f`/`%U` from `Exec=`).
- [x] Filters the list live as you type, showing icon + name rows.
- [x] Up/Down moves the selection in the results list (focus stays in the entry).
- [x] Enter launches the selected row's `Exec=`, or falls back to raw shell exec.
- [x] Tab completes the entry to the top match's name.
- [x] Escape quits.
- [ ] Not yet done: match ranking (currently plain substring match, no fuzzy/priority scoring),
      history/frecency, `$PATH` executable completion for the raw-command path (gmrun originally
      tab-completed binary names and file paths, not just app names), real Wayland session
      testing (only X11/Xvfb has been exercised so far), packaging.

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

## License

MIT, see [LICENSE](LICENSE). This is original code, not derived from gmrun's GPL-licensed sources
or from onagre.
