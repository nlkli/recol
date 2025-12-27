use std::{
    fmt::Write,
    fs,
    io::{self, BufRead},
    path::Path,
};
use strsim::{jaro_winkler, normalized_levenshtein};

pub fn fuzzy_search_strings<'a>(items: &'a [String], query: &str) -> Option<&'a str> {
    let refs: Vec<&'a str> = items.iter().map(|s| s.as_str()).collect();
    fuzzy_search(&refs, query)
}

pub fn fuzzy_search<'a, 'v>(items: &'v [&'a str], query: &str) -> Option<&'a str> {
    if query.len() > 512 {
        return None;
    }
    let query_norm = query
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let mut best_score = 0.0;
    let mut best_item = None;

    for &item in items {
        let item_norm = item.to_lowercase();

        let jw = jaro_winkler(&query_norm, &item_norm);
        let lev = normalized_levenshtein(&query_norm, &item_norm);

        let mut score = 0.7 * jw + 0.3 * lev;

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

pub fn write_content_inside_text_block<P>(
    path: P,
    content: &str,
    blocks_mark: (&str, &str),
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut buf = String::new();
    let mut lines = reader.lines();
    let mut inserted = false;

    while let Some(line) = lines.next() {
        let line = line?;
        let _ = writeln!(&mut buf, "{}", &line);
        if line == blocks_mark.0 {
            let _ = writeln!(&mut buf, "{}", &content);
            inserted = true;
            break;
        }
    }
    if inserted {
        let mut replace_buf = String::new();
        let mut found_end = false;
        while let Some(line) = lines.next() {
            let line = line?;
            let _ = writeln!(&mut replace_buf, "{}", &line);
            if line == blocks_mark.1 {
                found_end = true;
                break;
            }
        }
        if found_end {
            let _ = writeln!(&mut buf, "{}", blocks_mark.1);
        } else {
            let _ = writeln!(&mut buf, "{}", &replace_buf);
        }
        while let Some(line) = lines.next() {
            let line = line?;
            let _ = writeln!(&mut buf, "{}", &line);
        }
    } else {
        let _ = writeln!(&mut buf, "\n{}{content}\n{}", blocks_mark.0, blocks_mark.1,);
    }

    fs::write(&path, &buf)?;
    Ok(())
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
        $root.$field.as_ref().ok_or_else(|| missing_field($path))
    };
}
