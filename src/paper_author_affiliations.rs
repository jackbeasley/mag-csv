use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PaperAuthorAffiliation {
    #[serde(rename = ":START_ID(Paper-ID)")]
    paper_id: i64,

    #[serde(rename = ":END_ID(Author-ID)")]
    author_id: i64,

    #[serde(rename = "affiliationId:long")]
    // Note: this is tricky to model, so just include as attribute
    affiliation_id: Option<i64>,

    #[serde(rename = "authorSequenceNumber:int")]
    author_sequence_number: i32, // 1 is first author

    #[serde(skip_serializing)]
    original_affiliation: Option<String>,

    #[serde(skip_deserializing, rename = ":TYPE")]
    relationship_type: Option<String>,
}

pub fn convert_paper_author_affilations() -> Result<(), Box<Error>> {
    println!("Converting PaperAuthorAffiliations.txt");
    let output_f = File::create("PaperAuthorAffiliations.csv")
        .expect("Unable to create file PaperAuthorAffiliations.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f = File::open("PaperAuthorAffiliations.txt")
        .expect("Unable to open file PaperAuthorAffiliations.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let paper_author_affiliation: PaperAuthorAffiliation = result?;
        wtr.serialize(PaperAuthorAffiliation {
            relationship_type: Some("AUTHORED_BY".to_string()),
            ..paper_author_affiliation
        })?;
    }
    wtr.flush()?;
    Ok(())
}
