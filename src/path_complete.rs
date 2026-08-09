use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

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

/// The `$PATH` executable names in `bins` (see `scan_path_executables`)
/// whose name starts with `prefix`, in `bins`' existing (sorted) order.
/// This is the raw candidate set behind both [`complete_prefix`]'s
/// auto-fill decision and the Tab-shown candidate list in `main` when
/// there's more than one match.
pub fn matches_for_prefix<'a>(bins: &'a [String], prefix: &str) -> Vec<&'a str> {
    bins.iter()
        .map(String::as_str)
        .filter(|b| b.starts_with(prefix))
        .collect()
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

    match matches_for_prefix(bins, prefix).as_slice() {
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

/// Resolves a leading `~` in a directory prefix (e.g. `"~/"`, `"~"`) to the
/// user's actual home directory, leaving anything else untouched. `None`
/// only if `dir` starts with `~` and there's no resolvable home directory.
fn expand_tilde_dir(dir: &str) -> Option<PathBuf> {
    match dir.strip_prefix('~') {
        Some(rest) => {
            let home = dirs::home_dir()?;
            match rest.strip_prefix('/') {
                Some(rest) if !rest.is_empty() => Some(home.join(rest)),
                _ => Some(home),
            }
        }
        None => Some(PathBuf::from(dir)),
    }
}

/// The filesystem entries matching the directory+prefix encoded in `word`
/// (see [`complete_path`] for the accepted forms), as `(dir_as_typed,
/// sorted matches)` — `matches` pairs each entry's bare name with whether
/// it's a directory. `None` if `word` isn't a resolvable absolute/`~` path,
/// or its directory can't be read.
fn path_dir_matches(word: &str) -> Option<(&str, Vec<(String, bool)>)> {
    let slash_at = word.rfind('/')?;
    let dir_as_typed = &word[..=slash_at];
    let file_prefix = &word[slash_at + 1..];

    if !(dir_as_typed.starts_with('/') || dir_as_typed.starts_with('~')) {
        return None;
    }

    let dir = expand_tilde_dir(dir_as_typed)?;
    let entries = fs::read_dir(&dir).ok()?;

    let mut matches: Vec<(String, bool)> = entries
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_str()?.to_string();
            if !name.starts_with(file_prefix) {
                return None;
            }
            // Hide dotfiles unless the user's actually typing a dot-prefix,
            // same convention as shell globbing/completion.
            if name.starts_with('.') && !file_prefix.starts_with('.') {
                return None;
            }
            let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
            Some((name, is_dir))
        })
        .collect();
    matches.sort();

    Some((dir_as_typed, matches))
}

/// Tab-completes a filesystem path the user is typing (gmrun originally
/// completed file paths too, not just `$PATH` binary names). `word` is
/// everything typed so far for this argument, e.g. `"/usr/bi"` or
/// `"~/Doc"`. Only absolute (`/...`) and home-relative (`~/...`) paths are
/// handled — there's no meaningful "current directory" to resolve a bare
/// relative path against from inside the launcher.
///
/// Returns the full replacement text for `word`: the single match if
/// there's exactly one (with a trailing `/` for directories, so completion
/// can continue into them), or the longest common prefix among matches if
/// that's longer than what's already typed. `None` means there's nothing
/// useful to complete to.
pub fn complete_path(word: &str) -> Option<String> {
    if word == "~" {
        return Some("~/".to_string());
    }

    let (dir_as_typed, matches) = path_dir_matches(word)?;
    let file_prefix = &word[dir_as_typed.len()..];

    match matches.as_slice() {
        [] => None,
        [(name, is_dir)] => {
            let suffix = if *is_dir { "/" } else { "" };
            Some(format!("{dir_as_typed}{name}{suffix}"))
        }
        many => {
            let names: Vec<&str> = many.iter().map(|(n, _)| n.as_str()).collect();
            let common = longest_common_prefix(&names);
            if common.chars().count() > file_prefix.chars().count() {
                Some(format!("{dir_as_typed}{common}"))
            } else {
                None
            }
        }
    }
}

/// Every full-path candidate `word` could complete to, one per matching
/// filesystem entry (directories get a trailing `/`), sorted. This is the
/// list shown below the entry on Tab when there's more than one match:
/// [`complete_path`] alone only reports the single-match/common-prefix
/// outcome, not the individual options behind it.
pub fn path_candidates(word: &str) -> Vec<String> {
    let Some((dir_as_typed, matches)) = path_dir_matches(word) else {
        return Vec::new();
    };

    matches
        .into_iter()
        .map(|(name, is_dir)| {
            let suffix = if is_dir { "/" } else { "" };
            format!("{dir_as_typed}{name}{suffix}")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_for_prefix_lists_every_match() {
        let bins = vec![
            "systemctl".to_string(),
            "systemd-analyze".to_string(),
            "systemd-run".to_string(),
            "firefox".to_string(),
        ];
        assert_eq!(
            matches_for_prefix(&bins, "system"),
            vec!["systemctl", "systemd-analyze", "systemd-run"]
        );
    }

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

    /// Builds `<tmp>/gmrun-rejuvenated-test-path-complete-<suffix>/` with the
    /// given entry names (directories get a trailing `/`) and returns its path.
    fn make_test_dir(suffix: &str, entries: &[&str]) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("gmrun-rejuvenated-test-path-complete-{suffix}"));
        std::fs::create_dir_all(&dir).unwrap();
        for entry in entries {
            if let Some(sub) = entry.strip_suffix('/') {
                std::fs::create_dir_all(dir.join(sub)).unwrap();
            } else {
                std::fs::write(dir.join(entry), "").unwrap();
            }
        }
        dir
    }

    #[test]
    fn completes_path_to_the_single_matching_entry() {
        let dir = make_test_dir("single", &["firefox.desktop"]);
        let word = format!("{}/firef", dir.display());
        assert_eq!(
            complete_path(&word),
            Some(format!("{}/firefox.desktop", dir.display()))
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn completes_path_to_a_directory_with_a_trailing_slash() {
        let dir = make_test_dir("dir", &["Documents/"]);
        let word = format!("{}/Doc", dir.display());
        assert_eq!(
            complete_path(&word),
            Some(format!("{}/Documents/", dir.display()))
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn completes_ambiguous_path_to_the_common_prefix() {
        let dir = make_test_dir("ambiguous", &["report-jan.txt", "report-feb.txt"]);
        let word = format!("{}/rep", dir.display());
        assert_eq!(
            complete_path(&word),
            Some(format!("{}/report-", dir.display()))
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn path_candidates_lists_every_ambiguous_match() {
        let dir = make_test_dir(
            "candidates",
            &["report-jan.txt", "report-feb.txt", "notes.txt"],
        );
        let word = format!("{}/rep", dir.display());
        assert_eq!(
            path_candidates(&word),
            vec![
                format!("{}/report-feb.txt", dir.display()),
                format!("{}/report-jan.txt", dir.display()),
            ]
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn path_candidates_still_lists_a_single_match() {
        // Callers decide whether a one-entry list is worth displaying
        // (it usually isn't, since `complete_path` alone already handles
        // that case) — this function just reports what matched.
        let dir = make_test_dir("candidates-single", &["firefox.desktop"]);
        let word = format!("{}/firef", dir.display());
        assert_eq!(
            path_candidates(&word),
            vec![format!("{}/firefox.desktop", dir.display())]
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn path_completion_hides_dotfiles_unless_asked_for() {
        let dir = make_test_dir("dotfiles", &[".hidden", "visible.txt"]);
        let word = format!("{}/", dir.display());
        assert_eq!(
            complete_path(&word),
            Some(format!("{}/visible.txt", dir.display()))
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn returns_none_for_a_bare_relative_word() {
        // No slash and no `~`: nothing meaningful to resolve against from
        // inside the launcher.
        assert_eq!(complete_path("read"), None);
    }

    #[test]
    fn returns_none_when_no_path_entry_matches() {
        let dir = make_test_dir("nomatch", &["firefox.desktop"]);
        let word = format!("{}/zzz", dir.display());
        assert_eq!(complete_path(&word), None);
        std::fs::remove_dir_all(&dir).ok();
    }
}
