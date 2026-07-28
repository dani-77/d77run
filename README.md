<p align="center">
  <img
    width="600"
    src="./docs/onagre.png"
    alt="d77-launcher logo"
  />
</p>

<p align="center">
  <a href="https://github.com/dani-77/d77-launcher/actions/workflows/CD.yml"><img
      src="https://github.com/dani-77/d77-launcher/actions/workflows/CD.yml/badge.svg"
      alt="GitHub Actions workflow status"
  /></a>
    <img
      src="https://github.com/dani-77/d77-launcher/actions/workflows/Release.yaml/badge.svg"
      alt="GitHub Actions workflow status"
  /></a>
  <br />
  <a href="https://conventionalcommits.org"
    ><img
      src="https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg"
      alt="Conventional commits"
  /></a>
  <a href="https://github.com/dani-77/d77-launcher/blob/main/LICENSE"
    ><img
      src="https://img.shields.io/github/license/dani-77/d77-launcher"
      alt="Repository license"
  /></a>
</p>

<p align="center">
A general purpose application launcher for X and wayland inspired <br>
by rofi/wofi and alfred,<br>
build with <a href ="https://github.com/hecrj/iced/">iced</a>
and <a href ="https://github.com/pop-os/launcher">pop-launcher</a>.
</p>

---

**d77-launcher** is a personal fork of [onagre](https://github.com/oknozor/onagre)
by [oknozor](https://github.com/oknozor) (all credit for the original design
and codebase goes to them). This fork adds a few `gmrun`-style ergonomics on
top: a raw shell-command fallback, a window that grows/shrinks with the live
result count, and a Tokyo Night theme tuned for that compact footprint.

d77-launcher is build on top of [pop-launcher](https://github.com/pop-os/launcher) which makes it very versatile.
The pop-launcher plugin system allows you to extend d77-launcher with plugins from the community or even write your own
using any programming language.

## Features

- Works on x11 and wayland.
- Fully customizable theme.
- Default plugins: calc, files, pop_shell, recent, terminal, desktop entries, find, pulse, scripts, web.
- Can be extended with [pop-launcher](https://github.com/pop-os/launcher) plugins.
- `gmrun`-style raw shell command fallback: if nothing matches your input, it's run as `sh -c "<input>"`.
- Dynamic window resizing: the window grows/shrinks with the number of visible results instead of a fixed size.
- Bundled Tokyo Night theme (`docs/themes/tokyo-night-gmrun.scss`) tuned for a small, gmrun-like footprint.

## Install

**Dependencies:**
- ⚠️ [pop-launcher](https://github.com/pop-os/launcher) > 1.2.4
    **Rust 1.8 introduced a breaking change in the way sorting is handled, d77-launcher will unexpectedly crash with older version of pop launcher.**
    **Currently, for arch users, the only way to get the latest version of pop-launcher is to build it from source.**
- [Qalculate](http://qalculate.github.io/) (optional)

There is no distro package for d77-launcher (it's a personal fork), so you'll need
Rust and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build it.

**From source:**

```bash
git clone https://github.com/dani-77/d77-launcher.git
cd d77-launcher
cargo build --release --locked
sudo mv target/release/d77-launcher /usr/bin/d77-launcher
```

**With cargo:**

```bash
cargo install --git https://github.com/dani-77/d77-launcher
```

## Usage

**1. Key bindings:**


| Key             | Action                       |
|:----------------|:-----------------------------|
| `Arrow up/down` | Change selection             |
| `Tab`           | Autocomplete (in files mode) |
| `Esc`           | Quit without launching       |
| `Enter`         | Launch selection             |

**2. Plugins:**

To use a plugin simply match its regex when typing your query.

For instance the `file` plugin will match `^(/|~).*`, typing `~/` would enable the plugin and start the file navigation.

Plugin with no prefix are enabled by default, there entry will be mixed in the search results.

If nothing matches your input at all (no desktop entry, no plugin), d77-launcher falls back to
running it as a raw shell command (`sh -c "<input>"`), gmrun-style.

**Default plugins:**

| Mode        | Description                                                   | Prefix           | Configuration                                            |
|:------------|:--------------------------------------------------------------|:-----------------|:---------------------------------------------------------|
| History     | Display the most used desktop entries on start                |                  |                                                          |
| PopLauncher | Search for desktop entries                                    |                  |                                                          |
| Pulse       | Control PulseAudio devices and volume                         |                  |                                                          |
| Script      | Shell scripts as launcher options                             |                  | `$HOME/.local/share/pop-launcher/scripts`                |
| Terminal    | Terminal or background commands                               | 'run '           |                                                          |
| Web         | Web search                                                    | 'ddg ', 'g', ... | `$HOME/.local/share/pop-launcher/plugins/web/config.ron` |
| Files       | Find files using fd/find                                      | 'find '          |                                                          |
| Recent      | Recently-opened document search                               | 'recent '        |                                                          |
| Calc        | Calculator with unit conversion (uses Qalculate! expressions) | '= '             |                                                          |
| Help        | List available pop-launcher modes                             | '?'              |                                                          |


## Theming

d77-launcher will look for a theme file in `$XDG_CONFIG_HOME/d77-launcher/theme.scss` (typically
`~/.config/d77-launcher/theme.scss`) and will fall back to the default theme if none is found or
if your theme contains syntax errors. To ensure your theme is correctly formatted, run
`d77-launcher` from the terminal and check the logs.

> **Migrating from onagre:** if you used to run upstream onagre, copy your old
> `~/.config/onagre/theme.scss` to `~/.config/d77-launcher/theme.scss` — the config
> directory changed along with the app name.

For a detailed theming reference, see [docs/website/src/theming-reference.md](docs/website/src/theming-reference.md),
or take a look at the [theme examples directory](docs/website/src/.vuepress/public/theme_examples)
and the bundled [Tokyo Night gmrun theme](docs/themes/tokyo-night-gmrun.scss) (see [docs/themes/README.md](docs/themes/README.md)).

## Gallery

---
<img src="docs/website/src/.vuepress/public/screenshots/default-theme.png" alt="default-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto; width: 65%;"/>

*Default theme*

---
<img src="docs/website/src/.vuepress/public/screenshots/murz.png" alt="murz-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto; width: 65%;"/>

[*Murz*](docs/website/src/.vuepress/public/theme_examples/murz.scss) (credit to [murz](https://github.com/Murzchnvok/rofi-collection))

---
<img src="docs/website/src/.vuepress/public/screenshots/nord-rounded.png" alt="simple-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto; width: 65%;"/>

[*Nord*](docs/website/src/.vuepress/public/theme_examples/nord-rounded.scss)

---
<img src="docs/website/src/.vuepress/public/screenshots/not-adwaita.png" alt="not-adwaita-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto; width: 65%;"/>

[*Not-Adwaita*](docs/website/src/.vuepress/public/theme_examples/not-adwaita.scss)

---
<img src="docs/website/src/.vuepress/public/screenshots/solarized.png" alt="solarized-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto; width: 65%;"/>

[*Solarized*](docs/website/src/.vuepress/public/theme_examples/solarized.scss)

---
<img src="docs/website/src/.vuepress/public/screenshots/darcula.png" alt="darcula-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto; width: 65%;"/>

*Darcula*

---
<img src="docs/website/src/.vuepress/public/screenshots/hollow.png" alt="darcula-theme-screenshot" style="display: block; margin-left: auto; margin-right: auto; width: 65%;"/>

*Hollow*

---

## Roadmap / What's left to do

This fork is functional and builds clean, but a few things are still open:

- [ ] **Docs website not migrated** — `docs/website` (the VuePress site) still reads as
      the upstream onagre project (titles, logo, GitHub links, `docs.onagre.dev` domain).
      The CI no longer publishes to that domain (removed, since this fork doesn't own it),
      but the site content itself still needs a rebrand pass if it's going to be published.
- [ ] **Tune dynamic-resize constants** — `D77Launcher::resize_to_content` in `src/app/mod.rs`
      uses hardcoded `BASE_HEIGHT`/`ROW_HEIGHT`/`MAX_VISIBLE_ROWS` constants tuned for the
      default theme; if rows look clipped or leave a gap with a custom theme, retune those.
- [ ] **Clippy/lint cleanup** — three `mismatched_lifetime_syntaxes` warnings in
      `src/app/cache.rs` (cosmetic, `cargo fix --bin d77-launcher` fixes them automatically).
- [ ] **Pick a default theme** — decide whether the bundled Tokyo Night gmrun theme
      (`docs/themes/tokyo-night-gmrun.scss`) becomes the shipped default or stays opt-in.
- [ ] **Release/publish pipeline** — `cog.toml` and the GitHub Actions workflows now point at
      `dani-77/d77-launcher`, but nothing has been tagged/released from this fork yet.

## Related projects

- [pop-launcher](https://github.com/pop-os/launcher)
- [pop-shell](https://github.com/pop-os/shell/)
- [cosmic-launcher](https://github.com/pop-os/cosmic-launcher)
- [onagre](https://github.com/oknozor/onagre) — the upstream project this fork is based on.

## Code of conduct

This project is bound by a [code of conduct](CODE_OF_CONDUCT.md) based on the [contributor covenant](https://www.contributor-covenant.org/) if you are not familiar with it, and want to contribute please, read it before going further.

## Contributing

Having a question or suggestion for a new feature? Feel free to open an issue or submit a PR.

## License

All the code in this repository is released under the MIT License, for more information take a look at the [LICENSE](LICENSE) file.

## Thanks

Credit to [oknozor](https://github.com/oknozor) for onagre, the project this fork is built on,
and to [@themou3ad](https://github.com/themou3ad) for the original logo.
