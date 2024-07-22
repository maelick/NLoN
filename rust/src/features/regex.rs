use std::collections::HashSet;

use regex::Regex;
use once_cell::sync::Lazy;

use super::tokenizers::TokenizeFunc;

// https://github.com/M3SOulu/NLoN/blob/master/R/features.R

static CAPS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[A-Z]").expect("regex didn't compile"));
static SPECIAL_CHARS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[^a-zA-Z\\d\\s]").expect("regex didn't compile"));
static NUMBERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("\\d").expect("regex didn't compile"));
static WORDS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("\\w+").expect("regex didn't compile"));
const EMOTICONS: &str = ":-\\)|;-\\)|:\\)|;\\)|:-\\(|:\\(";
static EMOTICONS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(EMOTICONS).expect("regex didn't compile"));
static TRAILING_EMOTICONS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!("{}$", EMOTICONS).as_str()).expect("regex didn't compile"));
static TRAILING_CODE_CHAR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[){;]$").expect("regex didn't compile"));
static TRAILING_PUNCTUATION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[.!?:,]$").expect("regex didn't compile"));
static STARTS_WITH_THREE_LETTERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^\\s*[a-zA-Z]{3}").expect("regex didn't compile"));

fn count_stopwords(tokens: Vec<&str>, stopwords: &HashSet<&str>) -> usize {
    tokens.iter().filter(|s| stopwords.contains(s as &str)).count()
}

fn caps_count(s: &str) -> usize {
    CAPS_REGEX.find_iter(s).count()
}

fn special_chars_count(s: &str) -> usize {
    SPECIAL_CHARS_REGEX.find_iter(s).count()
}

fn numbers_count(s: &str) -> usize {
    NUMBERS_REGEX.find_iter(s).count()
}

pub fn caps_ratio(s: &str) -> f64 {
    caps_count(s) as f64 / s.len() as f64
}

pub fn special_chars_ratio(s: &str) -> f64 {
    special_chars_count(s) as f64 / s.len() as f64
}

pub fn numbers_ratio(s: &str) -> f64 {
    numbers_count(s) as f64 / s.len() as f64
}

pub fn stopwords_ratio(s: &str, tokenize: TokenizeFunc, stopwords: &HashSet<&str>) -> f64 {
    let tokens = tokenize(s);
    count_stopwords(tokens, stopwords) as f64 / words_count(s) as f64
}

fn words_count(s: &str) -> usize {
    WORDS_REGEX.find_iter(s).count()
}

pub fn average_word_length(s: &str) -> f64 {
    s.len() as f64 / words_count(s) as f64
}

pub fn ends_with_code_char(s: &str) -> bool {
    !TRAILING_EMOTICONS_REGEX.is_match(s) && TRAILING_CODE_CHAR_REGEX.is_match(s)
}

pub fn ends_with_punctuation(s: &str) -> bool {
    TRAILING_PUNCTUATION_REGEX.is_match(s)
}

pub fn starts_with_three_letters(s: &str) -> bool {
    STARTS_WITH_THREE_LETTERS_REGEX.is_match(s)
}

pub fn emoticons_count(s: &str) -> usize {
    EMOTICONS_REGEX.find_iter(s).count()
}

pub fn starts_with_at(s: &str) -> bool {
    s.starts_with('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_stopwords() {
        let tokens = vec!["Hello", "World"];
        let stopwords = vec!["World"].into_iter().collect();
        assert_eq!(count_stopwords(tokens, &stopwords), 1);
    }

    #[test]
    fn test_caps() {
        assert_eq!(caps_count("Hello World!"), 2);
    }

    #[test]
    fn test_specialchars() {
        assert_eq!(special_chars_count("Hello World!"), 1);
    }

    #[test]
    fn test_numbers() {
        assert_eq!(numbers_count("Hello World! 123"), 3);
    }

    #[test]
    fn test_words() {
        assert_eq!(words_count("Hello World!"), 2);
    }
}