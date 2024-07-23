use nlon_rust::features::text::LegacyTextFeatureGenerator;
use nlon_rust::data;

fn main() {
    let stopwords = data::read_stopwords().expect("Failed to read stopwords");
    let generator = LegacyTextFeatureGenerator::new(stopwords);
    let data = nlon_rust::data::read_data().expect("Failed to read data");
    println!("{:?}", data);
    let text = data.column("text").unwrap();
    let features = generator.generate(text).unwrap();
    println!("{:?}", features);
}
