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