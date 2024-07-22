use std::collections::HashSet;
use polars::prelude::*;
use super::regex;
use super::tokenizers;

pub struct LegacyTextFeatureGenerator<'a> {
    stopwords: HashSet<&'a str>,
}

impl<'a> LegacyTextFeatureGenerator<'a> {
    pub fn new(stopwords: HashSet<&'a str>) -> Self {
        Self { stopwords }
    }

    fn caps_ratio(&self, s: &Series) -> Series {
        let caps_ratio: Vec<f64> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::caps_ratio(s.unwrap_or("")))
            .collect();
        Series::new("caps_ratio", caps_ratio)
    }

    fn special_chars_ratio(&self, s: &Series) -> Series {
        let special_chars_ratio: Vec<f64> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::special_chars_ratio(s.unwrap_or("")))
            .collect();
        Series::new("special_chars_ratio", special_chars_ratio)
    }

    fn numbers_ratio(&self, s: &Series) -> Series {
        let numbers_ratio: Vec<f64> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::numbers_ratio(s.unwrap_or("")))
            .collect();
        Series::new("numbers_ratio", numbers_ratio)
    }

    fn stopwords_ratio(&self, s: &Series) -> Series {
        let stopwords_ratio: Vec<f64> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::stopwords_ratio(s.unwrap_or(""), tokenizers::tokenize1, &self.stopwords))
            .collect();
        Series::new("stopwords_ratio", stopwords_ratio)
    }

    fn average_word_length(&self, s: &Series) -> Series {
        let average_word_length: Vec<f64> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::average_word_length(s.unwrap_or("")))
            .collect();
        Series::new("average_word_length", average_word_length)
    }

    fn ends_with_code_char(&self, s: &Series) -> Series {
        let ends_with_code_char: Vec<bool> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::ends_with_code_char(s.unwrap_or("")))
            .collect();
        Series::new("ends_with_code_char", ends_with_code_char)
    }

    fn ends_with_punctuation(&self, s: &Series) -> Series {
        let ends_with_punctuation: Vec<bool> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::ends_with_punctuation(s.unwrap_or("")))
            .collect();
        Series::new("ends_with_punctuation", ends_with_punctuation)
    }

    fn starts_with_three_letters(&self, s: &Series) -> Series {
        let starts_with_three_letters: Vec<bool> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::starts_with_three_letters(s.unwrap_or("")))
            .collect();
        Series::new("starts_with_three_letters", starts_with_three_letters)
    }

    fn emoticons_count(&self, s: &Series) -> Series {
        let emoticons_count: Vec<i64> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::emoticons_count(s.unwrap_or("")))
            .collect();
        Series::new("emoticons_count", emoticons_count)
    }

    fn starts_with_at(&self, s: &Series) -> Series {
        let starts_with_at: Vec<bool> = s.str()
            .unwrap()
            .iter()
            .map(|s| regex::starts_with_at(s.unwrap_or("")))
            .collect();
        Series::new("starts_with_at", starts_with_at)
    }

    pub fn generate(&self, s: &Series) -> DataFrame {
        DataFrame::new(vec![
            self.caps_ratio(s),
            self.special_chars_ratio(s),
            self.numbers_ratio(s),
            self.average_word_length(s),
            self.stopwords_ratio(s),
            self.ends_with_code_char(s),
            self.ends_with_punctuation(s),
            self.starts_with_three_letters(s),
            self.emoticons_count(s),
            self.starts_with_at(s),
        ]).unwrap()
    }
}