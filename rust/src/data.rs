use std::path::Path;
use std::collections::HashSet;
use polars::prelude::*;

const DATA_DIRNAME: &str = "../data-raw";

const _KUBERNETES_RAW_DATA_FILENAME: &str = "lines.10k.cfo.sample.2000 - Kubernetes (Slackarchive.io).csv";
const KUBERNETES_DATA_FILENAME: &str = "kubernetes.csv";

const _LUCENE_RAW_DATA_FILENAME: &str = "lines.10k.cfo.sample.2000 - Lucene-dev mailing list.csv";
const LUCENE_DATA_FILENAME: &str = "lucene.csv";

const _MOZILLA_RAW_DATA_FILENAME: &str = "lines.10k.cfo.sample.2000 - Mozilla (Firefox, Core, OS).csv";
const MOZILLA_DATA_FILENAME: &str = "mozilla.csv";

const EXTDATA_DIRNAME: &str = "../inst/extdata";
const STOPWORDS_FILENAME: &str = "mysql_sw_wo_code_words.txt";

pub fn read_data() -> PolarsResult<DataFrame> {
    let kubernetes_data = read_data_file(KUBERNETES_DATA_FILENAME)?;
    let lucene_data = read_data_file(LUCENE_DATA_FILENAME)?;
    let mozilla_data = read_data_file(MOZILLA_DATA_FILENAME)?;
    mozilla_data.vstack(&kubernetes_data)?.vstack(&lucene_data)
}

pub fn read_stopwords() -> PolarsResult<HashSet<String>> {
    let filename = Path::new(EXTDATA_DIRNAME).join(STOPWORDS_FILENAME);
    let stopwords = std::fs::read_to_string(filename)?;
    Ok(stopwords.lines().map(|s| s.to_string()).collect())
}

fn read_data_file(filename: &str) -> PolarsResult<DataFrame> {
    let filename = Path::new(DATA_DIRNAME).join(filename);
    let filename = filename.to_str().map(Into::into);
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(filename)?
            .finish()
}