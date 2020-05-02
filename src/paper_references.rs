use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PaperReference {
    #[serde(rename = ":START_ID(Paper-ID)")]
    paper_id: i64,

    #[serde(rename = ":END_ID(Paper-ID)")]
    paper_reference_id: i64,

    #[serde(skip_deserializing, rename = ":TYPE")]
    relationship_type: Option<String>,
}

pub fn convert_paper_references() -> Result<(), Box<Error>> {
    println!("Converting PaperReferences.txt");
    let output_f =
        File::create("PaperReferences.csv").expect("Unable to create file PaperReferences.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f =
        File::open("PaperReferences.txt").expect("Unable to open file PaperReferences.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let paper_reference: PaperReference = result?;
        wtr.serialize(PaperReference {
            paper_id: paper_reference.paper_id,
            relationship_type: Some("REFERENCES".to_string()),
            paper_reference_id: paper_reference.paper_reference_id,
        })?;
    }
    wtr.flush()?;
    Ok(())
}
