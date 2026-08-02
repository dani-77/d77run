use std::fs;
use std::io::Write;
use std::path::PathBuf;

const MAX_ENTRIES: usize = 500;

/// `$XDG_DATA_HOME/d77run/history` (or the `dirs` crate's platform default),
/// one entry per line, oldest first — same idea as gmrun's persistent
/// command history.
fn history_path() -> Option<PathBuf> {
    let mut dir = dirs::data_dir()?;
    dir.push("d77run");
    Some(dir.join("history"))
}

/// Loads history entries, oldest first. Returns an empty list if there's no
/// history file yet or it can't be read.
pub fn load() -> Vec<String> {
    let Some(path) = history_path() else {
        return Vec::new();
    };
    let Ok(content) = fs::read_to_string(path) else {
        return Vec::new();
    };
    content.lines().map(|l| l.to_string()).collect()
}

/// Appends `entry` to the history file, skipping empty input and immediate
/// repeats of the last entry. Keeps at most `MAX_ENTRIES` lines.
pub fn append(entry: &str) {
    let entry = entry.trim();
    if entry.is_empty() {
        return;
    }

    let Some(path) = history_path() else {
        return;
    };
    let Some(parent) = path.parent() else {
        return;
    };
    if fs::create_dir_all(parent).is_err() {
        return;
    }

    let mut lines = load();
    if lines.last().map(|s| s.as_str()) == Some(entry) {
        return;
    }

    lines.push(entry.to_string());
    if lines.len() > MAX_ENTRIES {
        let excess = lines.len() - MAX_ENTRIES;
        lines.drain(0..excess);
    }

    if let Ok(mut file) = fs::File::create(&path) {
        let _ = file.write_all(lines.join("\n").as_bytes());
        let _ = file.write_all(b"\n");
    }
}
