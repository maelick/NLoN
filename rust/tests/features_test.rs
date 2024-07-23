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
    let expected = vec![0., 0., 0., 0., 0.75, 0.75, 0.5];
    let count = generator.stopwords_ratio(&input).expect("Failed to calculate stopwords ratio");
    assert_eq!(count, Series::new("stopwords_ratio", expected));
}

#[test]
fn test_stopwords_count2() {
    let stopwords = data::read_stopwords().expect("Failed to read stopwords");
    let generator = LegacyTextFeatureGenerator::new(stopwords);

    let input = Series::new("text", vec!["", "text", "123", "!@#$", "This is some text.",
        "This isn't some text.", "This is."]);
    let expected = vec![0., 0., 0., 0., 0.75, 0.75, 1.];
    let count = generator.stopwords_ratio2(&input).expect("Failed to calculate stopwords ratio");
    assert_eq!(count, Series::new("stopwords_ratio", expected));
}