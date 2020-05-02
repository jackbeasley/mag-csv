use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PaperFieldOfStudy {
    #[serde(rename = ":START_ID(Paper-ID)")]
    paper_id: i64,

    #[serde(rename = ":END_ID(FieldOfStudy-ID)")]
    field_of_study_id: i64,

    #[serde(rename = "score:float")]
    score: f64, // Confidence between 0 and 1

    #[serde(skip_deserializing, rename = ":TYPE")]
    relationship_type: Option<String>,
}

pub fn convert_paper_fields_of_study() -> Result<(), Box<Error>> {
    println!("Converting PaperFieldsOfStudy.txt");
    let output_f = File::create("PaperFieldsOfStudy.csv")
        .expect("Unable to create file PaperFieldsOfStudy.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f =
        File::open("PaperFieldsOfStudy.txt").expect("Unable to open file PaperFieldsOfStudy.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let paper_field_of_study: PaperFieldOfStudy = result?;
        wtr.serialize(PaperFieldOfStudy {
            paper_id: paper_field_of_study.paper_id,
            relationship_type: Some("IN_FIELD".to_string()),
            field_of_study_id: paper_field_of_study.field_of_study_id,
            score: paper_field_of_study.score,
        })?;
    }
    wtr.flush()?;
    Ok(())
}
