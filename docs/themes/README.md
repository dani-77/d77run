# Themes

## tokyo-night-gmrun.scss

A Tokyo Night theme aimed at making onagre feel closer to `gmrun`:

- Small window (`52px` tall, `640px` wide) — mostly just the search bar
- Results list uses `--height: shrink` instead of the default
  `fill-portion 6`, so it doesn't reserve a big fixed area when there's
  nothing to show
- Thin scrollbar, no heavy borders, monospace font

### Install

```sh
mkdir -p ~/.config/onagre
cp docs/themes/tokyo-night-gmrun.scss ~/.config/onagre/theme.scss
```

Edit `--font-family` to match whatever monospace/Nerd Font you have
installed.

### Known limitation

Onagre's top-level window size is still fixed at startup (from
`.onagre { height }` / `width`), it does not currently grow/shrink
dynamically as results appear or disappear — `--height: shrink` only
affects the internal `.rows` container's share of that fixed window,
not the window itself. A true "no window at all until there's a match"
behaviour like gmrun's would need a follow-up code change (e.g. issuing
an `iced::window::resize` command from `on_input_changed` based on
`current_entries_len()`).
