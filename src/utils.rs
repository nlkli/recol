use std::io;
use strsim::{jaro_winkler, normalized_levenshtein};

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn fuzzy_search<'a, 'v>(items: &'v [&'a str], query: &str) -> Option<&'a str> {
    let query_norm = normalize(query);

    let mut best_score = 0.0;
    let mut best_item = None;

    for &item in items {
        let item_norm = normalize(item);

        let jw = jaro_winkler(&query_norm, &item_norm);
        let lev = normalized_levenshtein(&query_norm, &item_norm);

        // основной скор
        let mut score = 0.7 * jw + 0.3 * lev;

        // небольшой бонус, если все слова запроса присутствуют
        let query_words: Vec<&str> = query_norm.split(' ').collect();
        let mut word_matches = 0;

        for w in &query_words {
            if item_norm.contains(w) {
                word_matches += 1;
            }
        }

        score += 0.05 * word_matches as f64;

        if score > best_score {
            best_score = score;
            best_item = Some(item);
        }
    }

    best_item
}

#[inline(always)]
pub fn as_array_ref<T, const N: usize>(s: &[T]) -> &[T; N] {
    assert_eq!(s.len(), N);
    unsafe { &*(s.as_ptr() as *const [T; N]) }
}


pub fn io_other_error<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}

pub fn missing_field(path: &'static str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("required field `{}` is missing", path),
    )
}

#[macro_export]
macro_rules! require_field {
    ($root:expr, $path:literal, $field:ident) => {
        $root.$field
            .as_ref()
            .ok_or_else(|| missing_field($path))
    };
}
