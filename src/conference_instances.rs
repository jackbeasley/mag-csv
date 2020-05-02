use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct ConferenceInstance {
    #[serde(rename = "conferenceInstanceId:ID(ConferenceInstance-ID)")]
    conference_instance_id: i64,

    #[serde(rename = "normalizedName:string")]
    normalized_name: Option<String>,

    #[serde(rename = "displayName:string")]
    display_name: Option<String>,

    conference_series_id: Option<i64>,

    #[serde(rename = "location:string")]
    location: Option<String>,

    #[serde(rename = "officialUrl:string")]
    official_url: Option<String>,

    #[serde(rename = "startDate:date")]
    start_date: Option<String>,

    #[serde(rename = "endDate:date")]
    end_date: Option<String>,

    #[serde(rename = "abstractRegistrationDate:date")]
    abstract_registration_date: Option<String>,

    #[serde(rename = "submissionDeadlineDate:date")]
    submission_deadline_date: Option<String>,

    #[serde(rename = "notificationDueDate:date")]
    notification_due_date: Option<String>,

    #[serde(rename = "finalVersionDueDate:date")]
    final_version_due_date: Option<String>,

    #[serde(rename = "paperCount:long")]
    paper_count: Option<i64>,

    #[serde(rename = "citationCount:long")]
    citation_count: Option<i64>,

    #[serde(rename = "createdDate:date")]
    created_date: Option<String>,

    #[serde(skip_deserializing, rename = ":LABEL")]
    label: Option<String>,
}

pub fn convert_conference_instances() -> Result<(), Box<Error>> {
    println!("Converting ConferenceInstances.txt");
    let output_f = File::create("ConferenceInstances.csv")
        .expect("Unable to create file ConferenceInstances.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f =
        File::open("ConferenceInstances.txt").expect("Unable to open file ConferenceInstances.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let conference_instance: ConferenceInstance = result?;
        wtr.serialize(ConferenceInstance {
            label: Some("ConferenceInstance".to_string()),
            ..conference_instance
        })?;
    }
    wtr.flush()?;
    Ok(())
}
