use std::collections::HashSet;

use regex::Regex;
use once_cell::sync::Lazy;

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

pub fn stopwords_count(tokens: Vec<String>, stopwords: &HashSet<String>) -> i64 {
    tokens.iter().filter(|s| stopwords.contains(&s.to_string())).count() as i64
}

pub fn caps_count(s: &str) -> i64 {
    CAPS_REGEX.find_iter(s).count() as i64
}

pub fn special_chars_count(s: &str) -> i64 {
    SPECIAL_CHARS_REGEX.find_iter(s).count() as i64
}

pub fn numbers_count(s: &str) -> i64 {
    NUMBERS_REGEX.find_iter(s).count() as i64
}

pub fn words_count(s: &str) -> i64 {
    // FIXME
    // WORDS_REGEX.find_iter(s).count()
    WORD_SEP_REGEX.find_iter(s).count() as i64 + 1
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

    fn run_tests<I, O>(inputs: &Vec<I>, expected: &Vec<O>, func: impl Fn(&I) -> O)
    where
        I: std::fmt::Display,
        O: PartialEq + std::fmt::Debug
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
        run_tests(&inputs, &expected, |s| caps_count(s));
    }

    #[test]
    fn test_special_chars_count() {
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "x-y", "test;", "just some text"];
        let expected = vec![0, 0, 0, 4, 1, 1, 1, 0];
        run_tests(&inputs, &expected, |s| special_chars_count(s));
    }

    #[test]
    fn test_numbers_count() {
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "There is 1 number."];
        let expected = vec![0, 0, 3, 0, 0, 1];
        run_tests(&inputs, &expected, |s| numbers_count(s));
    }

    #[test]
    fn test_words_count() {
        let inputs = vec!["", "text", "123", "!@#$", "This is some text.", "one-word.", "abc!def"];
        let expected = vec![1, 1, 1, 1, 4, 1, 1];
        // FIXME: first one should be 0 -> need to use proper word count (or tokenizer?)
        run_tests(&inputs, &expected, |s| words_count(s));
    }



    #[test]
    fn test_ends_with_code_char() {
        let inputs = vec!["", "This is text.", "func(x, y);", "if (true) {",
            "func()", ":-)", "Hello ;-)", ":)", ":-(", ":(", ":-))"];
        let expected = vec![false, false, true, true, true, false, false, false, false, false, true];
        run_tests(&inputs, &expected, |s| ends_with_code_char(s));
    }

    #[test]
    fn test_ends_with_punctuation() {
        let inputs = vec!["", "abc", "1", ".", "!", "?", "? ", ":", ",", "Hello!!"];
        let expected = vec![false, false, false, true, true, true, false, true, true, true];
        run_tests(&inputs, &expected, |s| ends_with_punctuation(s));
    }

    #[test]
    fn test_ends_either_with_punctuation_or_code_char() {
        let inputs = vec!["", "This is text.", "func(x, y);", "if (true) {",
            "func()", ":-)", "Hello ;-)", ":)", ":-(", ":(", ":-))",
            "", "abc", "1", ".", "!", "?", "? ", ":", ",", "Hello!!"];
        for input in inputs {
            let ends_with_punctuation = ends_with_punctuation(input);
            let ends_with_code_char = ends_with_code_char(input);
            assert!(!ends_with_punctuation || !ends_with_code_char, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_starts_with_three_letters() {
        let inputs = vec!["", "a", "a  bcde", " abcde", "      abcde"];
        let expected = vec![false, false, true, true, true];
        run_tests(&inputs, &expected, |s| starts_with_three_letters(s));
    }

    #[test]
    fn test_emoticons_count() {
        let inputs = vec!["", "123", "abc", ":--)", ":-)", "Hello ;-)", "\":-)\";",
            ":)", ";)", ":-(", ":(", ":(:)", ":) :) :)", ":):(:"];
        let expected = vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 3, 2];
        run_tests(&inputs, &expected, |s| emoticons_count(s));
    }

    #[test]
    fn test_starts_with_at() {
        let inputs = vec!["", "abc", "123", "!@#", "@abc", "@", " @"];
        let expected = vec![false, false, false, false, true, true, false];
        run_tests(&inputs, &expected, |s| starts_with_at(s));
    }
}