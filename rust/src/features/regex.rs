use std::collections::HashSet;

use regex::Regex;

use super::tokenizers::TokenizeFunc;

// https://github.com/M3SOulu/NLoN/blob/master/R/features.R

pub fn count_stopwords(tokens: Vec<&str>, stopwords: HashSet<&str>) -> usize {
    tokens.iter().filter(|s| stopwords.contains(s as &str)).count()
}

pub fn caps(s: &str) -> usize {
    let re = Regex::new("[A-Z]").expect("regex didn't compile");
    re.find_iter(s).count()
}

pub fn specialchars(s: &str) -> usize {
    let re = Regex::new("[^a-zA-Z\\d\\s]").expect("regex didn't compile");
    re.find_iter(s).count()
}

pub fn numbers(s: &str) -> usize {
    let re = Regex::new("[\\d]").expect("regex didn't compile");
    re.find_iter(s).count()
}

pub fn capsratio(s: &str) -> usize {
    caps(s) / s.len()
}

pub fn specialcharsratio(s: &str) -> usize {
    specialchars(s) / s.len()
}

pub fn numbersratio(s: &str) -> usize {
    numbers(s) / s.len()
}

pub fn stopwords_ratio(s: &str, tokenize: TokenizeFunc, stopwords: HashSet<&str>) -> usize {
    let tokens = tokenize(s);
    count_stopwords(tokens, stopwords) / words(s)
}

pub fn words(s: &str) -> usize {
    let re = Regex::new("[\\s+]").expect("regex didn't compile");
    re.find_iter(s).count() + 1
}

pub fn averagewordlength(s: &str) -> usize {
    s.len() / words(s)
}

pub fn lastcharcode(s: &str) {
    let re1 = Regex::new("(:-\\)|;-\\)|:\\)|;\\)|:-\\(|:\\()$").expect("regex didn't compile");
    let re2 = Regex::new("[){;]$").expect("regex didn't compile");
    println!("LastCharCode")
}

pub fn lastcharnl(s: &str) {
    let re = Regex::new("\\.$|\\!$|\\?$|:$|,$").expect("regex didn't compile");
    println!("LastCharNL")
}

pub fn first3chars(s: &str) -> &str {
    let re = Regex::new("\\s").expect("regex didn't compile");
    return s
}

pub fn first3charsletters(s: &str) -> usize {
    let re = Regex::new("[a-zA-Z]").expect("regex didn't compile");
    re.find_iter(first3chars(s)).count()
}

pub fn emoticons(s: &str) -> usize {
    let re = Regex::new(":-\\)|;-\\)|:\\)|;\\)|:-\\(|:\\(").expect("regex didn't compile");
    re.find_iter(s).count()
}

pub fn startwithat(s: &str) -> bool {
    let re = Regex::new("^@").expect("regex didn't compile");
    re.is_match(s)
}