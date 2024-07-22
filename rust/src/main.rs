use nlon_rust::features::text::LegacyTextFeatureGenerator;

fn main() {
    let stopwords = vec!["World"].into_iter().collect();
    let generator = LegacyTextFeatureGenerator::new(stopwords);
    let data = nlon_rust::data::read_data().expect("Failed to read data");
    println!("{:?}", data);
    let text = data.column("text").unwrap();
    let features = generator.generate(text).unwrap();
    println!("{:?}", features);
}
