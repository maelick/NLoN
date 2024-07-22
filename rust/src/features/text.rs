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

    fn caps_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let caps_ratio: Series = s.str()?
            .iter()
            .map(|s| regex::caps_ratio(s.unwrap_or("")))
            .collect();
        Ok(caps_ratio.with_name("caps_ratio"))
    }

    fn special_chars_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let special_chars_ratio: Series = s.str()?
            .iter()
            .map(|s| regex::special_chars_ratio(s.unwrap_or("")))
            .collect();
        Ok(special_chars_ratio.with_name("special_chars_ratio"))
    }

    fn numbers_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let numbers_ratio: Series = s.str()?
            .iter()
            .map(|s| regex::numbers_ratio(s.unwrap_or("")))
            .collect();
        Ok(numbers_ratio.with_name("numbers_ratio"))
    }

    fn stopwords_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let stopwords_ratio: Series = s.str()?
            .iter()
            .map(|s| regex::stopwords_ratio(s.unwrap_or(""), tokenizers::tokenize1, &self.stopwords))
            .collect();
        Ok(stopwords_ratio.with_name("stopwords_ratio"))
    }

    fn average_word_length(&self, s: &Series) -> PolarsResult<Series> {
        let avg_word_length: Series = s.str()?
            .iter()
            .map(|s| regex::average_word_length(s.unwrap_or("")))
            .collect();
        Ok(avg_word_length.with_name("average_word_length"))
    }

    fn ends_with_code_char(&self, s: &Series) -> PolarsResult<Series> {
        let ends_with_code_char: Series = s.str()?
            .iter()
            .map(|s| regex::ends_with_code_char(s.unwrap_or("")))
            .collect();
        Ok(ends_with_code_char.with_name("ends_with_code_char"))
    }

    fn ends_with_punctuation(&self, s: &Series) -> PolarsResult<Series> {
        let ends_with_punctuation: Series = s.str()?
            .iter()
            .map(|s| regex::ends_with_punctuation(s.unwrap_or("")))
            .collect();
        Ok(ends_with_punctuation.with_name("ends_with_punctuation"))
    }

    fn starts_with_three_letters(&self, s: &Series) -> PolarsResult<Series> {
        let starts_with_three_letters: Series = s.str()?
            .iter()
            .map(|s| regex::starts_with_three_letters(s.unwrap_or("")))
            .collect();
        Ok(starts_with_three_letters.with_name("starts_with_three_letters"))
    }

    fn emoticons_count(&self, s: &Series) -> PolarsResult<Series> {
        let emoticons_count: Series = s.str()?
            .iter()
            .map(|s| regex::emoticons_count(s.unwrap_or("")))
            .collect();
        Ok(emoticons_count.with_name("emoticons_count"))
    }

    fn starts_with_at(&self, s: &Series) -> PolarsResult<Series> {
        let starts_with_at: Series = s.str()?
            .iter()
            .map(|s| regex::starts_with_at(s.unwrap_or("")))
            .collect();
        Ok(starts_with_at.with_name("starts_with_at"))
    }

    pub fn generate(&self, s: &Series) -> PolarsResult<DataFrame> {
        DataFrame::new(vec![
            self.caps_ratio(s)?,
            self.special_chars_ratio(s)?,
            self.numbers_ratio(s)?,
            self.average_word_length(s)?,
            self.stopwords_ratio(s)?,
            self.ends_with_code_char(s)?,
            self.ends_with_punctuation(s)?,
            self.starts_with_three_letters(s)?,
            self.emoticons_count(s)?,
            self.starts_with_at(s)?,
        ])
    }
}