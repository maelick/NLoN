use std::path::Path;
use polars_core::prelude::*;
use polars_io::prelude::*;

const data_dirname: &str = "../data-raw";

const kubernetes_raw_data_filename: &str = "lines.10k.cfo.sample.2000 - Kubernetes (Slackarchive.io).csv";
const kubernetes_data_filename: &str = "kubernetes.csv";

const lucene_raw_data_filename: &str = "lines.10k.cfo.sample.2000 - Lucene-dev mailing list.csv";
const lucene_data_filename: &str = "lucene.csv";

const mozilla_raw_data_filename: &str = "lines.10k.cfo.sample.2000 - Mozilla (Firefox, Core, OS).csv";
const mozilla_data_filename: &str = "mozilla.csv";

const extdata_dirname: &str = "../inst/extdata";
const stopwords_filename: &str = "mysql_sw_wo_code_words.txt";

pub fn read_data() -> PolarsResult<(DataFrame, DataFrame, DataFrame)> {
    let kubernetes_data = read_data_file(kubernetes_data_filename)?;
    let lucene_data = read_data_file(lucene_data_filename)?;
    let mozilla_data = read_data_file(mozilla_data_filename)?;
    Ok((kubernetes_raw_data, lucene_raw_data, mozilla_raw_data))
}

pub fn read_stopwords(filename: &str) -> PolarsResult<HashSet<String>> {
    let filename = Path::new(extdata_dirname).join(filename);
    let stopwords = std::fs::read_to_string(filename)?;
    Ok(stopwords.lines().map(|s| s.to_string()).collect())
}

fn read_data_file(filename: &str) -> PolarsResult<DataFrame> {
    let filename = Path::new(data_dirname).join(filename);
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(filename.to_str())?
            .finish()
}