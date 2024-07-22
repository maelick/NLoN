use nlon_rust::features;

#[test]
fn test_tokenize1() {
    let tokens = features::regex::tokenize1("Hello world!");
    assert_eq!(tokens, vec!["Hello", "world!"]);
}

#[test]
fn test_tokenize2() {
    let tokens = features::regex::tokenize2("Hello world!");
    assert_eq!(tokens, vec!["Hello", "world!"]);
}

#[test]
fn test_count_stopwords() {
    let tokens = vec!["Hello", "World"];
    let stopwords = vec!["World"].into_iter().collect();
    assert_eq!(features::regex::count_stopwords(tokens, stopwords), 1);
}

#[test]
fn test_caps() {
    assert_eq!(features::regex::caps("Hello World!"), 2);
}

#[test]
fn test_specialchars() {
    assert_eq!(features::regex::specialchars("Hello World!"), 1);
}

#[test]
fn test_numbers() {
    assert_eq!(features::regex::numbers("Hello World! 123"), 3);
}

#[test]
fn test_capsratio() {
    assert_eq!(features::regex::capsratio("Hello World!"), 2 / "Hello World!".len());
}

#[test]
fn test_specialcharsratio() {
    assert_eq!(features::regex::specialcharsratio("Hello World!"), 1 / "Hello World!".len());
}

#[test]
fn test_numbersratio() {
    assert_eq!(features::regex::numbersratio("Hello World! 123"), 3 / "Hello World! 123".len());
}

#[test]
fn test_stopwordsratio1() {
    let stopwords = vec!["World"].into_iter().collect();
    assert_eq!(features::regex::stopwordsratio1("Hello World!", stopwords), 1 / 2);
}

#[test]
fn test_stopwordsratio2() {
    let stopwords = vec!["World"].into_iter().collect();
    assert_eq!(features::regex::stopwordsratio2("Hello World!", stopwords), 1 / 2);
}

#[test]
fn test_words() {
    assert_eq!(features::regex::words("Hello World!"), 2);
}

#[test]
fn test_averagewordlength() {
    assert_eq!(features::regex::averagewordlength("Hello World!"), "Hello World!".len() / 2);
}