use std::collections::HashSet;
use polars::prelude::*;
use super::regex;
use super::tokenizers;

pub struct LegacyTextFeatureGenerator<'a> {
    stopwords: HashSet<&'a str>,
}

fn map_series<'b, F, T>(s: &'b Series, f: F) -> PolarsResult<impl Iterator<Item = T> + 'b>
where
    F: Fn(&str) -> T + 'b,
{
    let iter = s.str()?
        .iter()
        .map(move |s| f(s.unwrap_or("")));
    Ok(iter)
}

impl<'a> LegacyTextFeatureGenerator<'a> {
    pub fn new(stopwords: HashSet<&'a str>) -> Self {
        Self { stopwords }
    }

    fn caps_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::caps_ratio)?.collect();
        Ok(s.with_name("caps_ratio"))
    }

    fn special_chars_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::special_chars_ratio)?.collect();
        Ok(s.with_name("special_chars_ratio"))
    }

    fn numbers_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::numbers_ratio)?.collect();
        Ok(s.with_name("numbers_ratio"))
    }

    fn stopwords_ratio(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, |s| regex::stopwords_ratio(s, tokenizers::tokenize1, &self.stopwords))?.collect();
        Ok(s.with_name("stopwords_ratio"))
    }

    fn stopwords_ratio2(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, |s| regex::stopwords_ratio(s, tokenizers::tokenize2, &self.stopwords))?.collect();
        Ok(s.with_name("stopwords_ratio2"))
    }

    fn average_word_length(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::average_word_length)?.collect();
        Ok(s.with_name("average_word_length"))
    }

    fn ends_with_code_char(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::ends_with_code_char)?.collect();
        Ok(s.with_name("ends_with_code_char"))
    }

    fn ends_with_punctuation(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::ends_with_punctuation)?.collect();
        Ok(s.with_name("ends_with_punctuation"))
    }

    fn starts_with_three_letters(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::starts_with_three_letters)?.collect();
        Ok(s.with_name("starts_with_three_letters"))
    }

    fn emoticons_count(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::emoticons_count)?.collect();
        Ok(s.with_name("emoticons_count"))
    }

    fn starts_with_at(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::starts_with_at)?.collect();
        Ok(s.with_name("starts_with_at"))
    }

    pub fn generate(&self, s: &Series) -> PolarsResult<DataFrame> {
        DataFrame::new(vec![
            self.caps_ratio(s)?,
            self.special_chars_ratio(s)?,
            self.numbers_ratio(s)?,
            self.average_word_length(s)?,
            self.stopwords_ratio(s)?,
            self.stopwords_ratio2(s)?,
            self.ends_with_code_char(s)?,
            self.ends_with_punctuation(s)?,
            self.starts_with_three_letters(s)?,
            self.emoticons_count(s)?,
            self.starts_with_at(s)?,
        ])
    }
}