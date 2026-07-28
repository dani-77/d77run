# Themes

## tokyo-night-gmrun.scss

A Tokyo Night theme aimed at making d77-launcher feel closer to `gmrun`:

- Small window (`52px` tall, `640px` wide) — mostly just the search bar
- Results list uses `--height: shrink` instead of the default
  `fill-portion 6`, so it doesn't reserve a big fixed area when there's
  nothing to show
- Thin scrollbar, no heavy borders, monospace font

### Install

```sh
mkdir -p ~/.config/d77-launcher
cp docs/themes/tokyo-night-gmrun.scss ~/.config/d77-launcher/theme.scss
```

Edit `--font-family` to match whatever monospace/Nerd Font you have
installed.

### Window resizing

The top-level window (`.d77-launcher { height }` / `width`) now grows and
shrinks live with the number of visible results (see
`D77Launcher::resize_to_content` in `src/app/mod.rs`), so with zero results it
collapses back down to just the search bar, gmrun-style.
