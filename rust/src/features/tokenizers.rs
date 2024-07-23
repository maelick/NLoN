use regex::Regex;

pub type TokenizeFunc = fn(&str) -> Vec<&str>;

pub fn tokenize1<'t>(s: &'t str) -> Vec<&'t str> {
    let re = Regex::new("\\s+").expect("regex didn't compile");
    return re.split(&s).collect();
}

pub fn tokenize2<'t>(s: &'t str) -> Vec<&'t str> {
    let re = Regex::new("\\s+").expect("regex didn't compile");
    return re.split(&s).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize1() {
        let tokens = tokenize1("Hello world!");
        assert_eq!(tokens, vec!["Hello", "world!"]);
    }

    #[test]
    fn test_tokenize2() {
        let tokens = tokenize2("Hello world!");
        assert_eq!(tokens, vec!["Hello", "world!"]);
    }
}