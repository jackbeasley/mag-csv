use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Journal {
    #[serde(rename = "journalId:ID(Journal-ID)")]
    journal_id: i64,

    #[serde(rename = "rank:long")]
    rank: Option<i64>,

    #[serde(rename = "normalizedName:string")]
    normalized_name: Option<String>,

    #[serde(rename = "displayName:string")]
    display_name: Option<String>,

    #[serde(rename = "issn:string")]
    issn: Option<String>,

    #[serde(rename = "publisher:string")]
    publisher: Option<String>,

    #[serde(rename = "website:string")]
    webpage: Option<String>,

    #[serde(rename = "paperCount:long")]
    paper_count: Option<i64>,

    #[serde(rename = "citationCount:long")]
    citation_count: Option<i64>,

    #[serde(rename = "createdDate:date")]
    created_date: Option<String>,

    #[serde(skip_deserializing, rename = ":LABEL")]
    label: Option<String>,
}

pub fn convert_journals() -> Result<(), Box<Error>> {
    println!("Converting Journals.txt");
    let output_f = File::create("Journals.csv").expect("Unable to open file");
    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f = File::open("Journals.txt").expect("Unable to open file");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let journal: Journal = result?;
        wtr.serialize(Journal {
            label: Some("Journal".to_string()),
            ..journal
        })?;
    }
    wtr.flush()?;
    Ok(())
}
