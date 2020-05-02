use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct ConferenceSeries {
    #[serde(rename = "conferenceSeriesId:ID(ConferenceSeries-ID)")]
    conference_series_id: i64,

    #[serde(rename = "rank:long")]
    rank: Option<i64>,

    #[serde(rename = "normalizedName:string")]
    normalized_name: Option<String>,

    #[serde(rename = "displayName:string")]
    display_name: Option<String>,

    #[serde(rename = "paperCount:long")]
    paper_count: Option<i64>,

    #[serde(rename = "citationCount:long")]
    citation_count: Option<i64>,

    #[serde(rename = "createdDate:date")]
    created_date: Option<String>,

    #[serde(skip_deserializing, rename = ":LABEL")]
    label: Option<String>,
}

pub fn convert_conference_series() -> Result<(), Box<Error>> {
    println!("Converting ConferenceSeries.txt");
    let output_f =
        File::create("ConferenceSeries.csv").expect("Unable to create file ConferenceSeries.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f =
        File::open("ConferenceSeries.txt").expect("Unable to open file ConferenceSeries.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let conference_series: ConferenceSeries = result?;
        wtr.serialize(ConferenceSeries {
            label: Some("ConferenceSeries".to_string()),
            ..conference_series
        })?;
    }
    wtr.flush()?;
    Ok(())
}
