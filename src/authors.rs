use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct Author {
    #[serde(rename = "authorId:ID(Author-ID)")]
    author_id: i64,

    #[serde(rename = "rank:long")]
    rank: Option<i64>,

    #[serde(rename = "normalizedName:string")]
    normalized_name: Option<String>,

    #[serde(rename = "displayName:string")]
    display_name: Option<String>,

    #[serde(skip_serializing)]
    last_known_affiliation_id: Option<i64>,

    #[serde(rename = "paperCount:long")]
    paper_count: Option<i64>,

    #[serde(rename = "citationCount:long")]
    citation_count: Option<i64>,

    #[serde(rename = "createdDate:date")]
    created_date: Option<String>,

    #[serde(skip_deserializing, rename = ":LABEL")]
    label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AuthorLastAffiliation {
    #[serde(rename = ":START_ID(Author-ID)")]
    author_id: i64,

    #[serde(skip_deserializing, rename = ":TYPE")]
    relationship_type: Option<String>,

    #[serde(rename = ":END_ID(Affiliation-ID)")]
    last_known_affiliation_id: i64,
}

pub fn convert_authors() -> Result<(), Box<Error>> {
    println!("Converting Authors.txt");
    let output_f = File::create("Authors.csv").expect("Unable to create file Authors.csv");
    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let output_rel_f = File::create("AuthorAffiliations.csv")
        .expect("Unable to create file AuthorAffiliations.csv");
    let mut rel_wtr = csv::WriterBuilder::new().from_writer(output_rel_f);

    let input_f = File::open("Authors.txt").expect("Unable to open file Authors.txt");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let author: Author = result?;
        wtr.serialize(Author {
            label: Some("Author".to_string()),
            ..author
        })?;

        if let Some(lka_id) = author.last_known_affiliation_id {
            rel_wtr.serialize(AuthorLastAffiliation {
                author_id: author.author_id,
                relationship_type: Some("LAST_AFFILIATED_WITH".to_string()),
                last_known_affiliation_id: lka_id,
            })?;
        }
    }
    wtr.flush()?;
    rel_wtr.flush()?;
    Ok(())
}
