use strsim::{jaro_winkler, normalized_damerau_levenshtein, sorensen_dice};

/// Finds the top-N best matching strings from `candidates` for the given `query`.
///
/// Returns candidates sorted from most to least relevant, capped at `limit`.
/// Results below the minimum confidence threshold are excluded entirely.
/// Returns an empty `Vec` if no candidates clear the threshold.
pub fn search_top_n<'a>(
    query: &str,
    candidates: &[&'a str],
    limit: usize,
    min_score: Option<f64>,
) -> Vec<&'a str> {
    if limit == 0 {
        return vec![];
    }

    const DEFAULT_MIN_SCORE: f64 = 0.333333;

    let min_score = min_score.unwrap_or(DEFAULT_MIN_SCORE);

    let query_lower = query.to_lowercase();

    let mut scored: Vec<(&str, f64)> = candidates
        .iter()
        .filter_map(|&candidate| {
            let score = combined_score(&query_lower, &candidate.to_lowercase());
            if score >= min_score {
                Some((candidate, score))
            } else {
                None
            }
        })
        .collect();

    // Sort descending by score; break ties alphabetically for determinism.
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then_with(|| a.0.cmp(b.0)));

    scored.into_iter().take(limit).map(|(c, _)| c).collect()
}

/// Finds the best matching string from `candidates` for the given `query`.
///
/// Combines multiple similarity metrics to handle typos, case differences,
/// partial input, and minor misspellings robustly.
///
/// Returns `None` if `candidates` is empty or no match clears the minimum
/// confidence threshold.
pub fn search<'a>(query: &str, candidates: &[&'a str], min_score: Option<f64>) -> Option<&'a str> {
    const DEFAULT_MIN_SCORE: f64 = 0.333333;

    let min_score = min_score.unwrap_or(DEFAULT_MIN_SCORE);

    let query_lower = query.to_lowercase();

    candidates
        .iter()
        .filter_map(|&candidate| {
            let score = combined_score(&query_lower, &candidate.to_lowercase());
            if score >= min_score {
                Some((candidate, score))
            } else {
                None
            }
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(candidate, _)| candidate)
}

/// Computes a weighted composite similarity score in [0.0, 1.0].
fn combined_score(query: &str, candidate: &str) -> f64 {
    // Exact match short-circuit.
    if query == candidate {
        return 1.0;
    }

    // Prefix / substring bonuses reward partial input (e.g. "rust" → "rustfmt").
    let prefix_bonus = prefix_score(query, candidate);
    let substring_bonus = if candidate.contains(query) { 0.15 } else { 0.0 };

    // Core metrics covering different error types:
    //   jaro_winkler  – transpositions, common-prefix boost
    //   ndl           – insertions / deletions / substitutions + transpositions
    //   sorensen_dice – bigram overlap, good for partial matches
    let jw = jaro_winkler(query, candidate);
    let ndl = normalized_damerau_levenshtein(query, candidate);
    let sd = sorensen_dice(query, candidate);

    let base = 0.40 * jw + 0.35 * ndl + 0.25 * sd;

    (base + prefix_bonus + substring_bonus).min(1.0)
}

/// Returns a small bonus when the query is a leading prefix of the candidate,
/// scaled by how much of the candidate the query covers.
fn prefix_score(query: &str, candidate: &str) -> f64 {
    if candidate.starts_with(query) && !candidate.is_empty() {
        0.20 * (query.len() as f64 / candidate.len() as f64)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRUITS: &[&str] = &[
        "apple",
        "apricot",
        "banana",
        "blueberry",
        "cherry",
        "grape",
        "grapefruit",
        "kiwi",
        "lemon",
        "lime",
        "mango",
        "orange",
        "papaya",
        "peach",
        "pear",
        "pineapple",
        "plum",
        "raspberry",
        "strawberry",
        "watermelon",
    ];

    fn find(query: &str) -> Option<&'static str> {
        search(query, FRUITS, None)
    }

    #[test]
    fn exact_match() {
        assert_eq!(find("mango"), Some("mango"));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(find("APPLE"), Some("apple"));
        assert_eq!(find("Strawberry"), Some("strawberry"));
    }

    #[test]
    fn single_typo() {
        assert_eq!(find("appel"), Some("apple")); // transposition
        assert_eq!(find("grppe"), Some("grape")); // missing char
        assert_eq!(find("peacj"), Some("peach")); // substitution
    }

    #[test]
    fn partial_input() {
        assert_eq!(find("pine"), Some("pineapple"));
        assert_eq!(find("water"), Some("watermelon"));
        assert_eq!(find("rasp"), Some("raspberry"));
    }

    #[test]
    fn missing_characters() {
        assert_eq!(find("banan"), Some("banana"));
        assert_eq!(find("lmon"), Some("lemon"));
    }

    #[test]
    fn no_match_returns_none() {
        assert_eq!(find("zzzzzzz"), None);
        assert_eq!(find("xyz123"), None);
    }

    #[test]
    fn empty_candidates_returns_none() {
        assert_eq!(search("apple", &[], None), None);
    }

    #[test]
    fn single_candidate() {
        assert_eq!(search("aple", &["apple"], None), Some("apple"));
    }
}
