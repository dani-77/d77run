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

/// Reorders `entries` (oldest first, as stored on disk) into a frecency
/// ranking, à la Firefox's URL bar: each distinct entry's score is the sum
/// of `1 / (age + 1)` over every occurrence, where `age` is how many
/// launches ago that occurrence was (0 = the most recent line in the log).
/// A command used often keeps a high score even after newer one-offs run
/// after it, while an old single use fades out as more recent history
/// piles up. Ties (same score) break toward whichever was used more
/// recently.
fn rank_by_frecency(entries: &[String]) -> Vec<String> {
    let len = entries.len();

    // (entry, cumulative score, age of its most recent occurrence).
    let mut scored: Vec<(String, f64, usize)> = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let age = len - 1 - index;
        let weight = 1.0 / (age as f64 + 1.0);

        match scored.iter_mut().find(|(e, _, _)| e == entry) {
            Some(existing) => {
                existing.1 += weight;
                existing.2 = age; // later occurrences (smaller age) overwrite
            }
            None => scored.push((entry.clone(), weight, age)),
        }
    }

    scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.2.cmp(&b.2)));
    scored.into_iter().map(|(e, _, _)| e).collect()
}

/// Every distinct history entry, best (most "frecent") first — see
/// [`rank_by_frecency`].
pub fn frecent_order() -> Vec<String> {
    rank_by_frecency(&load())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strs(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn frequent_entry_outranks_a_single_more_recent_one() {
        // "vim" used 3 times, "htop" used once and more recently: "vim"'s
        // accumulated weight should still win.
        let entries = strs(&["vim", "ls", "vim", "vim", "htop"]);
        let ranked = rank_by_frecency(&entries);
        assert_eq!(ranked[0], "vim");
    }

    #[test]
    fn single_use_ranks_by_recency() {
        let entries = strs(&["firefox", "htop", "vim"]);
        assert_eq!(
            rank_by_frecency(&entries),
            strs(&["vim", "htop", "firefox"])
        );
    }

    #[test]
    fn every_distinct_entry_appears_exactly_once() {
        let entries = strs(&["a", "b", "a", "c", "b", "a"]);
        let mut ranked = rank_by_frecency(&entries);
        ranked.sort();
        assert_eq!(ranked, strs(&["a", "b", "c"]));
    }

    #[test]
    fn empty_history_ranks_to_an_empty_list() {
        assert_eq!(rank_by_frecency(&[]), Vec::<String>::new());
    }
}
