use nlon_rust::features;

#[test]
fn test_tokenize1() {
    let tokens = features::tokenizers::tokenize1("Hello world!");
    assert_eq!(tokens, vec!["Hello", "world!"]);
}

#[test]
fn test_tokenize2() {
    let tokens = features::tokenizers::tokenize2("Hello world!");
    assert_eq!(tokens, vec!["Hello", "world!"]);
}

#[test]
fn test_capsratio() {
    assert_eq!(features::regex::caps_ratio("Hello World!"), 2. / "Hello World!".len() as f64);
}

#[test]
fn test_specialcharsratio() {
    assert_eq!(features::regex::special_chars_ratio("Hello World!"), 1. / "Hello World!".len() as f64);
}

#[test]
fn test_numbersratio() {
    assert_eq!(features::regex::numbers_ratio("Hello World! 123"), 3. / "Hello World! 123".len() as f64);
}

#[test]
fn test_stopwordsratio1() {
    let stopwords = vec!["World"].into_iter().collect();
    assert_eq!(features::regex::stopwords_ratio("Hello World!", features::tokenizers::tokenize1, &stopwords), 1 / 2);
}

#[test]
fn test_stopwordsratio2() {
    let stopwords = vec!["World"].into_iter().collect();
    assert_eq!(features::regex::stopwords_ratio("Hello World!", features::tokenizers::tokenize2, &stopwords), 1 / 2);
}

#[test]
fn test_averagewordlength() {
    assert_eq!(features::regex::average_word_length("Hello World!"), "Hello World!".len() as f64 / 2.);
}