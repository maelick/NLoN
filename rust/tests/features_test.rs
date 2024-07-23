use nlon_rust::features::text::LegacyTextFeatureGenerator;
use nlon_rust::data;
use polars::prelude::*;

// TODO generate csv with R features and use them for integration tests

#[test]
fn test_stopwords_count1() {
    let stopwords = data::read_stopwords().expect("Failed to read stopwords");
    let generator = LegacyTextFeatureGenerator::new(stopwords);

    let input = Series::new("text", vec!["", "text", "123", "!@#$", "This is some text.",
        "This isn't some text.", "This is."]);
    let expected = vec![0, 0, 0, 0, 3, 3, 1];

    let df = generator.generate(&input).expect("Failed to generate features");
    let col = df.column("stopwords_count1").expect("Failed to get stopwords ratio");
    assert_eq!(col, &Series::new("stopwords_count1", expected));
}

#[test]
fn test_stopwords_count2() {
    let stopwords = data::read_stopwords().expect("Failed to read stopwords");
    let generator = LegacyTextFeatureGenerator::new(stopwords);

    let input = Series::new("text", vec!["", "text", "123", "!@#$", "This is some text.",
        "This isn't some text.", "This is."]);
    let expected = vec![0, 0, 0, 0, 3, 3, 2];

    let df = generator.generate(&input).expect("Failed to generate features");
    let col = df.column("stopwords_count2").expect("Failed to get stopwords ratio");
    assert_eq!(col, &Series::new("stopwords_count2", expected));
}

#[test]
fn test_average_word_length() {
    let stopwords = data::read_stopwords().expect("Failed to read stopwords");
    let generator = LegacyTextFeatureGenerator::new(stopwords);

    let input = Series::new("text", vec!["", "123", "123 123", "1", "!2c$", "abc def!", "1 234"]);
    let expected = vec![0., 3., 3.5, 1., 4., 4., 2.5];

    let df = generator.generate(&input).expect("Failed to generate features");
    let col = df.column("average_word_length").expect("Failed to get stopwords ratio");
    assert_eq!(col, &Series::new("average_word_length", expected));
}