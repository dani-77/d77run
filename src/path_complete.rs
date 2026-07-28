use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;

/// Scans every directory in `$PATH` for executable file names, for
/// Tab-completing the raw command line the way gmrun originally did.
/// Only bare binary names are collected (no file-path completion): a
/// name is included if it's a regular file or symlink with at least one
/// executable bit set.
pub fn scan_path_executables() -> Vec<String> {
    let Some(path_var) = std::env::var_os("PATH") else {
        return Vec::new();
    };

    let mut names = BTreeSet::new();
    for dir in std::env::split_paths(&path_var) {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if !(file_type.is_file() || file_type.is_symlink()) {
                continue;
            }
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            if metadata.permissions().mode() & 0o111 == 0 {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                names.insert(name.to_string());
            }
        }
    }

    names.into_iter().collect()
}

/// The prefix shared by every string in `items`. Empty if `items` is empty.
fn longest_common_prefix(items: &[&str]) -> String {
    let Some(first) = items.first() else {
        return String::new();
    };

    let mut prefix: Vec<char> = first.chars().collect();
    for item in &items[1..] {
        let common = prefix
            .iter()
            .zip(item.chars())
            .take_while(|(a, b)| **a == *b)
            .count();
        prefix.truncate(common);
        if prefix.is_empty() {
            break;
        }
    }

    prefix.into_iter().collect()
}

/// Given the sorted executable names from `scan_path_executables` and the
/// word currently being typed, returns what Tab-completion should replace
/// it with: the single match if there's exactly one, or the longest
/// common prefix shared by every match if that's longer than what's
/// already typed. `None` means there's nothing useful to complete to
/// (no matches, or the match set is already fully disambiguated).
pub fn complete_prefix(bins: &[String], prefix: &str) -> Option<String> {
    if prefix.is_empty() {
        return None;
    }

    let matches: Vec<&str> = bins
        .iter()
        .filter(|b| b.starts_with(prefix))
        .map(String::as_str)
        .collect();

    match matches.as_slice() {
        [] => None,
        [single] => Some(single.to_string()),
        many => {
            let common = longest_common_prefix(many);
            if common.chars().count() > prefix.chars().count() {
                Some(common)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completes_unambiguous_prefix_to_the_single_match() {
        let bins = vec!["udiskie".to_string(), "firefox".to_string()];
        assert_eq!(complete_prefix(&bins, "udi"), Some("udiskie".to_string()));
    }

    #[test]
    fn completes_ambiguous_prefix_to_the_common_part() {
        let bins = vec![
            "systemctl".to_string(),
            "systemd-analyze".to_string(),
            "systemd-run".to_string(),
        ];
        assert_eq!(complete_prefix(&bins, "sys"), Some("system".to_string()));
    }

    #[test]
    fn returns_none_when_the_match_set_is_already_fully_disambiguated() {
        // "ls" is itself one of the matches and also the longest common
        // prefix of the whole set, so there's nothing left to add.
        let bins = vec!["ls".to_string(), "lsblk".to_string(), "lsof".to_string()];
        assert_eq!(complete_prefix(&bins, "ls"), None);
    }

    #[test]
    fn returns_none_when_nothing_matches() {
        let bins = vec!["firefox".to_string()];
        assert_eq!(complete_prefix(&bins, "zzz"), None);
    }

    #[test]
    fn returns_none_for_an_empty_prefix() {
        let bins = vec!["firefox".to_string()];
        assert_eq!(complete_prefix(&bins, ""), None);
    }

    #[test]
    fn scan_path_executables_finds_something_on_a_real_system() {
        // Sanity check against the real $PATH of whatever machine runs the
        // tests — every POSIX system has at least `sh` on it somewhere.
        let bins = scan_path_executables();
        assert!(!bins.is_empty());
        assert!(bins.iter().any(|b| b == "sh"));
    }
}
