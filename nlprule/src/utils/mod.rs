use lazy_static::lazy_static;
use onig::{Captures, Regex};

pub mod parallelism;
pub mod regex;

// see https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
pub fn apply_to_first<F>(string: &str, func: F) -> String
where
    F: Fn(char) -> String,
{
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(first) => func(first) + c.as_str(),
    }
}

pub fn is_title_case(string: &str) -> bool {
    let mut char_case = string.chars().map(|x| x.is_uppercase());

    char_case.next().unwrap_or(false) && !char_case.any(|x| x)
}

pub fn is_uppercase(string: &str) -> bool {
    !string.chars().any(|x| x.is_lowercase())
}

// see https://github.com/rust-onig/rust-onig/issues/59#issuecomment-340160520
pub fn dollar_replace(mut replacement: String, caps: &Captures) -> String {
    for i in 1..caps.len() {
        replacement = replacement.replace(&format!("${}", i), caps.at(i).unwrap_or(""));
    }
    replacement
}

// remove duplicate whitespaces
pub fn normalize_whitespace(string: &str) -> String {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\s)\s+").unwrap();
    }

    REGEX.replace_all(string, |caps: &Captures| caps.at(1).unwrap().to_string())
}

#[inline]
pub fn splitting_chars() -> &'static str {
    r##"«»'’`´‘],.:;!?/\()<=>„“”"+#…*"##
}

#[inline]
pub fn no_space_chars() -> &'static str {
    r##","##
}

pub fn fix_nospace_chars(text: &str) -> String {
    text.char_indices()
        .filter(|(i, c)| {
            if c.is_whitespace() {
                !no_space_chars()
                    .chars()
                    .any(|nospace_c| text[(i + c.len_utf8())..].starts_with(nospace_c))
            } else {
                true
            }
        })
        .map(|x| x.1)
        .collect()
}
