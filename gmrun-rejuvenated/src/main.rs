mod desktop;

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

    let display = Display::default().expect("no display available");
    let icon_theme = IconTheme::for_display(&display);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("gmrun-rejuvenated")
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
        entry.connect_changed(move |entry| {
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
    {
        let filtered = filtered.clone();
        let key_controller = EventControllerKey::new();
        let entry_for_tab = entry.clone();
        key_controller.connect_key_pressed(move |_, keyval, _keycode, _state| {
            if keyval == gtk4::gdk::Key::Tab {
                if let Some(top) = filtered.borrow().first() {
                    entry_for_tab.set_text(&top.name);
                    entry_for_tab.set_position(-1);
                }
                return Propagation::Stop;
            }

            if keyval == gtk4::gdk::Key::Escape {
                exit(0);
            }

            Propagation::Proceed
        });
        entry.add_controller(key_controller);
    }

    window.present();
    entry.grab_focus();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}
