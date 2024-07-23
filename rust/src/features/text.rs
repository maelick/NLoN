use std::collections::HashSet;
use polars::prelude::*;
use super::regex;
use super::tokenizers;

pub struct LegacyTextFeatureGenerator {
    stopwords: HashSet<String>,
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

impl LegacyTextFeatureGenerator {
    pub fn new(stopwords: HashSet<String>) -> Self {
        Self { stopwords }
    }

    fn caps_count(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::caps_count)?.collect();
        Ok(s.with_name("caps_count"))
    }

    pub fn caps_ratio(&self, df: &DataFrame) -> PolarsResult<Series> {
        let text_length = df.column("text_length")?.cast(&DataType::Float64)?;
        let s = df.column("caps_count")?.cast(&DataType::Float64)?.divide(&text_length)?;
        Ok(s.with_name("caps_ratio"))
    }

    fn special_chars_count(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::special_chars_count)?.collect();
        Ok(s.with_name("special_chars_count"))
    }

    pub fn special_chars_ratio(&self, df: &DataFrame) -> PolarsResult<Series> {
        let text_length = df.column("text_length")?.cast(&DataType::Float64)?;
        let s = df.column("special_chars_count")?.cast(&DataType::Float64)?.divide(&text_length)?;
        Ok(s.with_name("special_chars_ratio"))
    }

    fn numbers_count(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::numbers_count)?.collect();
        Ok(s.with_name("numbers_count"))
    }

    pub fn numbers_ratio(&self, df: &DataFrame) -> PolarsResult<Series> {
        let text_length = df.column("text_length")?.cast(&DataType::Float64)?;
        let s = df.column("numbers_count")?.cast(&DataType::Float64)?.divide(&text_length)?;
        Ok(s.with_name("numbers_ratio"))
    }

    fn stopwords_count(&self, s: &Series, tokenize: tokenizers::TokenizeFunc) -> PolarsResult<Series> {
        Ok(map_series(s, |s| {
            let tokens = tokenize(s);
            regex::stopwords_count(tokens, &self.stopwords)
        })?.collect())
    }

    pub fn stopwords_ratio1(&self, df: &DataFrame) -> PolarsResult<Series> {
        let word_count = df.column("words_count")?.cast(&DataType::Float64)?;
        let s = df.column("stopwords_count1")?.cast(&DataType::Float64)?.divide(&word_count)?;
        Ok(s.with_name("stopwords_ratio"))
    }

    pub fn stopwords_ratio2(&self, df: &DataFrame) -> PolarsResult<Series> {
        let word_count = df.column("words_count")?.cast(&DataType::Float64)?;
        let s = df.column("stopwords_count2")?.cast(&DataType::Float64)?.divide(&word_count)?;
        Ok(s.with_name("stopwords_ratio2"))
    }

    fn words_count(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, regex::words_count)?.collect();
        Ok(s.with_name("words_count"))
    }

    pub fn average_word_length(&self, df: &DataFrame) -> PolarsResult<Series> {
        let word_count = df.column("words_count")?.cast(&DataType::Float64)?;
        let s = df.column("text_length")?.cast(&DataType::Float64)?.divide(&word_count)?;
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

    fn char_count(&self, s: &Series) -> PolarsResult<Series> {
        let s: Series = map_series(s, |s| s.len() as i64)?.collect();
        // let s = s.str()?.str_len_chars(); // TODO FIXME
        Ok(s.with_name("text_length"))
    }

    pub fn generate(&self, s: &Series) -> PolarsResult<DataFrame> {
        let mut df = DataFrame::new(vec![
            s.clone(),
            self.char_count(s)?,
            self.caps_count(s)?,
            self.special_chars_count(s)?,
            self.numbers_count(s)?,
            self.words_count(s)?,
            self.stopwords_count(s, tokenizers::tokenize1)?.with_name("stopwords_count1"),
            self.stopwords_count(s, tokenizers::tokenize2)?.with_name("stopwords_count2"),
            self.ends_with_code_char(s)?,
            self.ends_with_punctuation(s)?,
            self.starts_with_three_letters(s)?,
            self.emoticons_count(s)?,
            self.starts_with_at(s)?,
        ])?;
        df.with_column(self.caps_ratio(&df)?)?;
        df.with_column(self.special_chars_ratio(&df)?)?;
        df.with_column(self.numbers_ratio(&df)?)?;
        df.with_column(self.stopwords_ratio1(&df)?)?;
        df.with_column(self.stopwords_ratio2(&df)?)?;
        df.with_column(self.average_word_length(&df)?)?;
        Ok(df)
    }
}