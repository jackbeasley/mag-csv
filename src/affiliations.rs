use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Affiliation {
    #[serde(rename = "affiliationId:ID(Affiliation-ID)")]
    affiliation_id: i64,

    #[serde(rename = "rank:long")]
    rank: Option<i64>,

    #[serde(rename = "normalizedName:string")]
    normalized_name: Option<String>,

    #[serde(rename = "displayName:string")]
    display_name: Option<String>,

    #[serde(rename = "gridId:string")]
    grid_id: Option<String>,

    #[serde(rename = "officialPage:string")]
    official_page: Option<String>,

    #[serde(rename = "wikiPage:string")]
    wiki_page: Option<String>,

    #[serde(rename = "paperCount:long")]
    paper_count: Option<i64>,

    #[serde(rename = "citationCount:long")]
    citation_count: Option<i64>,

    #[serde(rename = "createdDate:date")]
    created_date: Option<String>,

    #[serde(skip_deserializing, rename = ":LABEL")]
    label: Option<String>,
}

pub fn convert_affiliations() -> Result<(), Box<Error>> {
    println!("Converting Affiliations.txt");
    let output_f =
        File::create("Affiliations.csv").expect("Unable to create file Affiliations.csv");
    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f = File::open("Affiliations.txt").expect("Unable to open file Affiliations.txt");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let affiliation: Affiliation = result?;
        wtr.serialize(Affiliation {
            label: Some("Affiliation".to_string()),
            ..affiliation
        })?;
    }
    wtr.flush()?;
    Ok(())
}
