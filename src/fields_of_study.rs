use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FieldOfStudy {
    #[serde(rename = "fieldOfStudyId:ID(FieldOfStudy-ID)")]
    field_of_study_id: i64,

    #[serde(rename = "rank:long")]
    rank: Option<i64>,

    #[serde(rename = "normalizedName:string")]
    normalized_name: Option<String>,

    #[serde(rename = "displayName:string")]
    display_name: Option<String>,

    #[serde(rename = "mainType:string")]
    main_type: Option<String>,

    #[serde(rename = "level:long")]
    level: Option<i64>, // 0-5

    #[serde(rename = "paperCount:long")]
    paper_count: Option<i64>,

    #[serde(rename = "citationCount:long")]
    citation_count: Option<i64>,

    #[serde(rename = "createdDate:date")]
    created_date: Option<String>,

    #[serde(skip_deserializing, rename = ":LABEL")]
    label: Option<String>,
}

pub fn convert_fields_of_study() -> Result<(), Box<Error>> {
    println!("Converting FieldsOfStudy.txt");
    let output_f =
        File::create("FieldsOfStudy.csv").expect("Unable to create file FieldsOfStudy.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f = File::open("FieldsOfStudy.txt").expect("Unable to open file FieldsOfStudy.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let field_of_study: FieldOfStudy = result?;
        wtr.serialize(FieldOfStudy {
            label: Some("FieldsOfStudy".to_string()),
            ..field_of_study
        })?;
    }
    wtr.flush()?;
    Ok(())
}
