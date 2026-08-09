/// Ranks how well `name` matches `query` for the results list, replacing
/// plain substring matching with dmenu/gmrun-style priority scoring. Lower
/// is a better match; `None` means `name` doesn't match `query` at all.
///
/// Priority, best to worst:
/// 0. Exact match (case-insensitive).
/// 1. `name` starts with `query` (e.g. "fire" -> "Firefox").
/// 2. `query` starts a word inside `name` (e.g. "fire" -> "Mozilla Firefox").
/// 3. `query` appears anywhere else in `name` (e.g. "fox" -> "Firefox").
pub fn score(name: &str, query: &str) -> Option<u8> {
    if query.is_empty() {
        return None;
    }

    let name = name.to_lowercase();
    let query = query.to_lowercase();

    if name == query {
        return Some(0);
    }
    if name.starts_with(&query) {
        return Some(1);
    }
    if name
        .split(|c: char| !c.is_alphanumeric())
        .any(|word| word.starts_with(&query))
    {
        return Some(2);
    }
    if name.contains(&query) {
        return Some(3);
    }

    None
}

/// Filters `items` down to those whose name (via `name_of`) matches `query`
/// at all, and orders the survivors best-match-first. Items scoring equally
/// keep their relative order from `items` (a stable sort), so e.g. an
/// already-alphabetical input stays alphabetical within each score tier.
pub fn filter_and_rank<T: Clone>(items: &[T], query: &str, name_of: impl Fn(&T) -> &str) -> Vec<T> {
    let mut scored: Vec<(u8, &T)> = items
        .iter()
        .filter_map(|item| score(name_of(item), query).map(|s| (s, item)))
        .collect();

    scored.sort_by_key(|(s, _)| *s);
    scored.into_iter().map(|(_, item)| item.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match_scores_best() {
        assert_eq!(score("Firefox", "firefox"), Some(0));
    }

    #[test]
    fn prefix_match_beats_word_boundary_match() {
        assert_eq!(score("Firefox", "fire"), Some(1));
        assert_eq!(score("Mozilla Firefox", "fire"), Some(2));
    }

    #[test]
    fn word_boundary_match_beats_bare_substring_match() {
        assert_eq!(score("Mozilla Firefox", "fire"), Some(2));
        assert_eq!(score("Firefox", "fox"), Some(3));
    }

    #[test]
    fn no_match_scores_none() {
        assert_eq!(score("Firefox", "chrome"), None);
        assert_eq!(score("Firefox", ""), None);
    }

    #[test]
    fn is_case_insensitive() {
        assert_eq!(score("GIMP", "gimp"), Some(0));
        assert_eq!(score("gimp", "GIMP"), Some(0));
    }

    #[test]
    fn filter_and_rank_orders_best_matches_first() {
        let names = vec![
            "Mozilla Firefox".to_string(),
            "Firefox".to_string(),
            "Firefox ESR".to_string(),
            "Terminal".to_string(),
        ];

        let ranked = filter_and_rank(&names, "firefox", |n| n.as_str());
        assert_eq!(
            ranked,
            vec![
                "Firefox".to_string(),
                "Firefox ESR".to_string(),
                "Mozilla Firefox".to_string()
            ]
        );
    }

    #[test]
    fn filter_and_rank_keeps_input_order_within_a_score_tier() {
        // Both are pure substring (score 3) matches for "box"; alphabetical
        // input order should be preserved since neither outranks the other.
        let names = vec!["Sandbox".to_string(), "Dropbox".to_string()];
        let ranked = filter_and_rank(&names, "box", |n| n.as_str());
        assert_eq!(ranked, names);
    }
}
