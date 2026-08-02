mod desktop;
mod history;
mod path_complete;

use std::cell::RefCell;
use std::process::{exit, Command, Stdio};
use std::rc::Rc;

use gtk4::gdk::Display;
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Entry, EventControllerKey, IconLookupFlags,
    IconTheme, Image, Label, ListBox, ListBoxRow, Orientation, SelectionMode, TextDirection,
};

use desktop::DesktopApp;

const APP_ID: &str = "dev.d77.gmrun-rejuvenated";
const ICON_SIZE: i32 = 24;
const MAX_RESULTS: usize = 8;

/// Runs whatever the user typed as a raw shell command, detached from us.
/// This is gmrun's original, core behaviour: if nothing else claims the
/// input, just `sh -c` it.
fn run_raw_command(raw: &str) {
    let raw = raw.trim();
    if raw.is_empty() {
        return;
    }

    let _ = Command::new("sh")
        .arg("-c")
        .arg(raw)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

/// Launches a matched desktop entry's `Exec=` command directly (no shell),
/// same spirit as gmrun but resolving through the actual application list
/// instead of only `$PATH` lookups.
fn run_desktop_app(app: &DesktopApp) {
    let Ok(tokens) = shell_words::split(&app.exec) else {
        return;
    };
    let Some((bin, args)) = tokens.split_first() else {
        return;
    };

    let _ = Command::new(bin)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

/// Tab-completes the last whitespace-separated word in `entry` against
/// `$PATH` executables (gmrun's raw-command completion). No-ops if the
/// word being typed looks like a path (contains `/`) or there's nothing
/// useful to complete it to.
fn complete_path_word(entry: &Entry, path_bins: &[String]) {
    let full_text = entry.text();
    let text: &str = &full_text;

    let word_start = text
        .char_indices()
        .rev()
        .find(|&(_, c)| c.is_whitespace())
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(0);

    let (head, word) = text.split_at(word_start);
    if word.is_empty() || word.contains('/') {
        return;
    }

    if let Some(completion) = path_complete::complete_prefix(path_bins, word) {
        entry.set_text(&format!("{head}{completion}"));
        entry.set_position(-1);
    }
}

fn lookup_icon(icon_theme: &IconTheme, icon_name: Option<&str>) -> Image {
    let name = icon_name
        .filter(|n| !n.is_empty())
        .unwrap_or("application-x-executable");

    if name.starts_with('/') {
        return Image::from_file(name);
    }

    let paintable = icon_theme.lookup_icon(
        name,
        &["application-x-executable"],
        ICON_SIZE,
        1,
        TextDirection::None,
        IconLookupFlags::empty(),
    );

    let image = Image::from_paintable(Some(&paintable));
    image.set_pixel_size(ICON_SIZE);
    image
}

fn build_row(icon_theme: &IconTheme, app: &DesktopApp) -> ListBoxRow {
    let row_box = GtkBox::new(Orientation::Horizontal, 8);
    row_box.set_margin_top(4);
    row_box.set_margin_bottom(4);
    row_box.set_margin_start(8);
    row_box.set_margin_end(8);

    let icon = lookup_icon(icon_theme, app.icon_name.as_deref());
    let label = Label::new(Some(&app.name));
    label.set_xalign(0.0);
    label.set_hexpand(true);

    row_box.append(&icon);
    row_box.append(&label);

    let row = ListBoxRow::new();
    row.set_child(Some(&row_box));
    row
}

fn build_ui(app: &Application) {
    let all_apps = Rc::new(desktop::scan_applications());
    let filtered: Rc<RefCell<Vec<DesktopApp>>> = Rc::new(RefCell::new(Vec::new()));
    let path_bins = Rc::new(path_complete::scan_path_executables());

    // Persistent command history (gmrun-style): oldest first on disk, walked
    // most-recent-first via `history_pos`. `suppress_history_reset` lets our
    // own history-driven `set_text` calls skip the "user is typing, cancel
    // history browsing" reset that `connect_changed` otherwise does.
    let history_entries: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(history::load()));
    let history_pos: Rc<RefCell<Option<usize>>> = Rc::new(RefCell::new(None));

    let display = Display::default().expect("no display available");
    let icon_theme = IconTheme::for_display(&display);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("d77run")
        .default_width(640)
        .default_height(52)
        .resizable(false)
        .decorated(false)
        .build();

    let container = GtkBox::new(Orientation::Vertical, 0);

    let entry = Entry::builder()
        .placeholder_text("Run a command or search applications...")
        .build();
    entry.set_margin_top(6);
    entry.set_margin_bottom(6);
    entry.set_margin_start(8);
    entry.set_margin_end(8);

    let results = ListBox::new();
    results.set_selection_mode(SelectionMode::Browse);
    results.set_visible(false);

    container.append(&entry);
    container.append(&results);
    window.set_child(Some(&container));

    // Filter the application list as the user types.
    {
        let filtered = filtered.clone();
        let all_apps = all_apps.clone();
        let results = results.clone();
        let icon_theme = icon_theme.clone();
        let history_pos = history_pos.clone();
        entry.connect_changed(move |entry| {
            // Any text change cancels history browsing by default. The
            // history-nav code path below restores the position it wants
            // right after calling set_text, overriding this — which also
            // covers the case where set_text's new value is identical to
            // what's already there and GTK skips emitting `changed`
            // entirely (e.g. cycling between two history entries with the
            // same text): nothing here runs, but nothing needed to.
            *history_pos.borrow_mut() = None;

            let query = entry.text().to_lowercase();

            while let Some(child) = results.first_child() {
                results.remove(&child);
            }

            if query.is_empty() {
                filtered.borrow_mut().clear();
                results.set_visible(false);
                return;
            }

            let matches: Vec<DesktopApp> = all_apps
                .iter()
                .filter(|a| a.name.to_lowercase().contains(&query))
                .take(MAX_RESULTS)
                .cloned()
                .collect();

            for app in &matches {
                results.append(&build_row(&icon_theme, app));
            }

            results.set_visible(!matches.is_empty());
            *filtered.borrow_mut() = matches;
        });
    }

    // Enter: launch the selected result if there is one, otherwise fall
    // back to running the raw text as a shell command (gmrun's behaviour).
    {
        let filtered = filtered.clone();
        let results = results.clone();
        let window = window.clone();
        entry.connect_activate(move |entry| {
            history::append(&entry.text());

            let selected_index = results.selected_row().map(|row| row.index());

            if let Some(index) = selected_index {
                if let Some(app) = filtered.borrow().get(index as usize) {
                    run_desktop_app(app);
                    window.close();
                    return;
                }
            }

            run_raw_command(&entry.text());
            window.close();
        });
    }

    // Tab: complete the entry text to the top match's name, gmrun-style.
    // If there's no matching application, fall back to completing the
    // word being typed against `$PATH` executables instead (gmrun
    // originally tab-completed binary names too, not just app names).
    // This has to run in the Capture phase on the *window*, otherwise GTK's
    // own focus-navigation swallows Tab before our handler ever sees it.
    {
        let filtered = filtered.clone();
        let path_bins = path_bins.clone();
        let key_controller = EventControllerKey::new();
        key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
        let entry_for_tab = entry.clone();
        let results_for_nav = results.clone();
        let history_entries = history_entries.clone();
        let history_pos = history_pos.clone();
        key_controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
            if keyval == gtk4::gdk::Key::Tab {
                // Clone the name and drop the borrow *before* calling
                // set_text: it re-enters via the `changed` signal, which
                // needs its own borrow_mut() on `filtered`.
                let top_name = filtered.borrow().first().map(|app| app.name.clone());
                if let Some(name) = top_name {
                    entry_for_tab.set_text(&name);
                    entry_for_tab.set_position(-1);
                    // Tab picks the top match, so select it in the results
                    // list too — Enter should fire immediately, no need to
                    // press Down first.
                    if let Some(row) = results_for_nav.row_at_index(0) {
                        results_for_nav.select_row(Some(&row));
                    }
                } else {
                    complete_path_word(&entry_for_tab, &path_bins);
                }
                return Propagation::Stop;
            }

            // Keep keyboard focus in the entry (so typing keeps working)
            // but move the result list's selection with the arrow keys,
            // the way most launchers behave. When there's no result list
            // showing, Up/Down instead walk persistent command history
            // (gmrun-style). Once history browsing has started, it keeps
            // going even if the history text we land on happens to also
            // match an app name (e.g. a Tab-completed app name saved from a
            // previous run) and repopulates the results list — otherwise
            // Up/Down would silently switch to list nav mid-cycle and the
            // entry text would stop changing.
            if keyval == gtk4::gdk::Key::Down || keyval == gtk4::gdk::Key::Up {
                let browsing_history = history_pos.borrow().is_some();
                let len = filtered.borrow().len() as i32;
                if len > 0 && !browsing_history {
                    let current = results_for_nav.selected_row().map(|r| r.index());
                    let next = match (keyval == gtk4::gdk::Key::Down, current) {
                        (true, Some(i)) => (i + 1).min(len - 1),
                        (true, None) => 0,
                        (false, Some(i)) => (i - 1).max(0),
                        (false, None) => len - 1,
                    };
                    if let Some(row) = results_for_nav.row_at_index(next) {
                        results_for_nav.select_row(Some(&row));
                    }
                } else {
                    let hist = history_entries.borrow();
                    if !hist.is_empty() {
                        let current_pos = *history_pos.borrow();
                        let next = if keyval == gtk4::gdk::Key::Up {
                            // Walk toward older entries, wrapping back around
                            // to the most recent one instead of sticking at
                            // the oldest — getting stuck there defeats the
                            // point of cycling through history.
                            Some(match current_pos {
                                None => 0,
                                Some(i) if i + 1 >= hist.len() => 0,
                                Some(i) => i + 1,
                            })
                        } else {
                            match current_pos {
                                None | Some(0) => None,
                                Some(i) => Some(i - 1),
                            }
                        };

                        match next {
                            Some(i) => {
                                entry_for_tab.set_text(&hist[hist.len() - 1 - i]);
                                entry_for_tab.set_position(-1);
                            }
                            None => entry_for_tab.set_text(""),
                        }
                        // Set *after* set_text: `connect_changed` unconditionally
                        // resets this to `None` on every text change it sees, so
                        // writing here overrides that back to what we actually
                        // want — and still lands correctly even when set_text is
                        // a no-op because the new text equals the old (GTK then
                        // skips `changed` entirely, e.g. cycling between two
                        // history entries with identical text).
                        *history_pos.borrow_mut() = next;
                    }
                }
                return Propagation::Stop;
            }

            if keyval == gtk4::gdk::Key::Escape {
                exit(0);
            }

            Propagation::Proceed
        });
        window.add_controller(key_controller);
    }

    window.present();
    entry.grab_focus();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}
