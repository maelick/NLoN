use std::collections::HashSet;

use regex::Regex;
use once_cell::sync::Lazy;

use super::tokenizers::TokenizeFunc;

// https://github.com/M3SOulu/NLoN/blob/master/R/features.R

static CAPS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[A-Z]").expect("regex didn't compile"));
static SPECIAL_CHARS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[^a-zA-Z\\d\\s]").expect("regex didn't compile"));
static NUMBERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("\\d").expect("regex didn't compile"));
pub static WORDS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[\\w']+").expect("regex didn't compile"));
pub static WORD_SEP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("\\s+").expect("regex didn't compile"));
const EMOTICONS: &str = ":-\\)|;-\\)|:\\)|;\\)|:-\\(|:\\(";
static EMOTICONS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(EMOTICONS).expect("regex didn't compile"));
static TRAILING_EMOTICONS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!("({})$", EMOTICONS).as_str()).expect("regex didn't compile"));
static TRAILING_CODE_CHAR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[){;]$").expect("regex didn't compile"));
static TRAILING_PUNCTUATION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("[.!?:,]$").expect("regex didn't compile"));
static STARTS_WITH_THREE_LETTERS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^(\\s*[a-zA-Z]){3}").expect("regex didn't compile"));

fn stopwords_count(tokens: Vec<String>, stopwords: &HashSet<String>) -> usize {
    tokens.iter().filter(|s| stopwords.contains(&s.to_string())).count()
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

pub fn stopwords_ratio(s: &str, tokenize: TokenizeFunc, stopwords: &HashSet<String>) -> f64 {
    let tokens = tokenize(s);
    stopwords_count(tokens, stopwords) as f64 / words_count(s) as f64
}

fn words_count(s: &str) -> usize {
    // FIXME
    // WORDS_REGEX.find_iter(s).count()
    WORD_SEP_REGEX.find_iter(s).count() + 1
}

pub fn average_word_length(s: &str) -> f64 {
    // FIXME use average of token len instead
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

pub fn emoticons_count(s: &str) -> i64 {
    EMOTICONS_REGEX.find_iter(s).count() as i64
}

pub fn starts_with_at(s: &str) -> bool {
    s.starts_with('@')
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tokenizers;

    fn to_string_col<T>(words: Vec<&str>) -> T
    where
        T: FromIterator<String>
    {
        words.into_iter().map(|s| s.to_string()).collect()
    }

    fn run_tests<T>(inputs: &Vec<&str>, expected: &Vec<T>, func: impl Fn(&str) -> T)
    where
        T: PartialEq + std::fmt::Debug
    {
        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let value = func(input);
            assert_eq!(value, *expected, "Failed for input: {} (got {:?}, expected {:?})", input, value, expected);
        }
    }

    #[test]
    fn test_stopwords_count() {
        let stopwords = to_string_col(vec!["this", "is", "isn't", "some", "peekaboo"]);
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "This isn't some text.", "This is."];

        // With tokenize1
        let expected = vec![0, 0, 0, 0, 3, 3, 1];
        run_tests(&inputs, &expected, |s| {
            stopwords_count(tokenizers::tokenize1(s), &stopwords)
        });

        // With tokenize2
        let expected = vec![0, 0, 0, 0, 3, 3, 2];
        run_tests(&inputs, &expected, |s| {
            stopwords_count(tokenizers::tokenize2(s), &stopwords)
        });
    }

    #[test]
    fn test_caps_count() {
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "A", "ABC", "aaABCaa"];
        let expected = vec![0, 0, 0, 0, 1, 1, 3, 3];
        run_tests(&inputs, &expected, caps_count);
    }

    #[test]
    fn test_special_chars_count() {
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "x-y", "test;", "just some text"];
        let expected = vec![0, 0, 0, 4, 1, 1, 1, 0];
        run_tests(&inputs, &expected, special_chars_count);
    }

    #[test]
    fn test_numbers_count() {
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "There is 1 number."];
        let expected = vec![0, 0, 3, 0, 0, 1];
        run_tests(&inputs, &expected, numbers_count);
    }

    #[test]
    fn test_words_count() {
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "one-word.", "abc!def"];
        let expected = vec![1, 1, 1, 1, 4, 1, 1];
        // FIXME: first one should be 0 -> need to use proper word count (or tokenizer?)
        run_tests(&inputs, &expected, words_count);
    }

    #[test]
    fn test_caps_ratio() {
        assert_eq!(caps_ratio("Hello World!"), 2. / "Hello World!".len() as f64);
    }

    #[test]
    fn test_special_chars_ratio() {
        assert_eq!(special_chars_ratio("Hello World!"), 1. / 12. as f64);
    }

    #[test]
    fn test_numbers_ratio() {
        assert_eq!(numbers_ratio("Hello World! 123"), 3. / 16. as f64);
    }

    #[test]
    fn test_stopwords_ratio1() {
        let stopwords = to_string_col(vec!["world", "peekaboo"]);
        assert_eq!(stopwords_ratio("Hello World", tokenizers::tokenize1, &stopwords), 0.5);
    }

    #[test]
    fn test_stopwords_ratio2() {
        let stopwords = to_string_col(vec!["world", "peekaboo"]);
        assert_eq!(stopwords_ratio("Hello World", tokenizers::tokenize2, &stopwords), 0.5);
    }

    #[test]
    fn test_average_word_length() {
        let inputs = vec!["", "123", "123 123", "1", "!2c$", "abc def!", "1 234"];
        let expected = vec![0., 3., 3.5, 1., 4., 4., 2.5];
        run_tests(&inputs, &expected, average_word_length);
    }
}