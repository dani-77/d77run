# gmrun-rejuvenated (exploration)

A from-scratch, minimal GTK4 launcher inspired by [gmrun](http://sourceforge.net/projects/gmrun),
built as an isolated experiment on the `explore/gmrun-rejuvenation` branch — it does not touch
the main `d77-launcher` (iced-based) app and is not part of its workspace.

Goals, compared to the original gmrun:

- **Modern GTK.** Written against GTK4 (`gtk4-rs`) instead of gmrun's GTK2, so it builds and
  themes correctly on current systems.
- **Icons for `/usr/share/applications` entries.** Resolves `Icon=` from `.desktop` files through
  GTK4's own icon-theme lookup (`IconTheme::lookup_icon`), which follows the icon theme spec's
  fallback chain properly — this is the part that didn't work in the original gmrun.
- **Direct command execution kept.** Pressing Enter with nothing selected from the results list
  still runs whatever you typed as a raw shell command (`sh -c "<input>"`), unchanged from gmrun's
  core behaviour.

## Status

MVP, functional but not battle-tested (no display server in the dev sandbox this was built in,
so UI interaction hasn't been visually verified — only `cargo build`/`cargo test`/`cargo clippy`
were run clean). What's there:

- [x] Scans `$XDG_DATA_HOME/applications`, `$XDG_DATA_DIRS/applications` (or the
      `/usr/local/share` + `/usr/share` default), recursively, parsing `.desktop` files
      (skips `NoDisplay`/`Hidden`, ignores `[Desktop Action ...]` sub-sections, strips
      field codes like `%f`/`%U` from `Exec=`).
- [x] Filters the list live as you type, showing icon + name rows.
- [x] Enter launches the selected row's `Exec=`, or falls back to raw shell exec.
- [x] Tab completes the entry to the top match's name.
- [x] Escape quits.
- [ ] Not yet done: match ranking (currently plain substring match, no fuzzy/priority scoring),
      history/frecency, `$PATH` executable completion for the raw-command path (gmrun originally
      tab-completed binary names and file paths, not just app names), visual testing on a real
      Wayland/X11 session, packaging.

## Build & test

```bash
cd gmrun-rejuvenated
cargo build
cargo test
cargo run
```

Requires GTK4 dev headers (`libgtk-4-dev` on Debian/Ubuntu) to build.
