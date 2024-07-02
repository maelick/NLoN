use std::collections::HashSet;

fn main() {
    let s = "Hello, world!";
    println!("{}", s);
    let mut stopwords = HashSet::new();
    stopwords.insert("Hello");
    let tokens = nlon_rust::features::regex::tokenize1("Hello world!");
    println!("{}", tokens.join("\n"));
    println!("{}", nlon_rust::features::regex::count_stopwords(tokens, stopwords));
}
