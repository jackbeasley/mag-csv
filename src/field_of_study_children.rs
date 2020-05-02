use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct FieldOfStudyChild {
    #[serde(rename = ":START_ID(FieldOfStudy-ID)")]
    field_of_study_id: i64,

    #[serde(rename = ":END_ID(FieldOfStudy-ID)")]
    child_field_of_study_id: i64,

    #[serde(skip_deserializing, rename = ":TYPE")]
    relationship_type: Option<String>,
}

pub fn convert_field_of_study_children() -> Result<(), Box<Error>> {
    println!("Converting FieldOfStudyChildren.txt");
    let output_f = File::create("FieldOfStudyChildren.csv")
        .expect("Unable to create file FieldOfStudyChildren.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f = File::open("FieldOfStudyChildren.txt")
        .expect("Unable to open file FieldOfStudyChildren.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let field_of_study_child: FieldOfStudyChild = result?;
        wtr.serialize(FieldOfStudyChild {
            field_of_study_id: field_of_study_child.field_of_study_id,
            relationship_type: Some("PARENT".to_string()),
            child_field_of_study_id: field_of_study_child.child_field_of_study_id,
        })?;
    }
    wtr.flush()?;
    Ok(())
}
