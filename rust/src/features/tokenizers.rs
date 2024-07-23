use super::regex::WORD_SEP_REGEX;
use super::regex::WORDS_REGEX;

pub type TokenizeFunc = fn(&str) -> Vec<String>;

pub fn tokenize1<'t>(s: &'t str) -> Vec<String> {
    let s = s.to_lowercase();
    let split =  WORD_SEP_REGEX.split(s.as_str());
    return split.map(|s| s.to_string()).collect();
}

pub fn tokenize2<'t>(s: &'t str) -> Vec<String> {
    let s = s.to_lowercase();
    let split =  WORDS_REGEX.find_iter(s.as_str());
    return split.map(|m| m.as_str().to_string()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize1() {
        let tokens = tokenize1("Hello world!");
        assert_eq!(tokens, vec!["hello", "world!"]);
    }

    #[test]
    fn test_tokenize2() {
        let tokens = tokenize2("Hello world!");
        assert_eq!(tokens, vec!["hello", "world"]);
    }
}