use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct Paper {
    #[serde(rename = "paperId:ID(Paper-ID)")]
    paper_id: i64,

    #[serde(rename = "rank:long")]
    rank: i64,

    #[serde(rename = "doi:string")]
    doi: Option<String>,

    #[serde(rename = "docType:string")]
    doc_type: Option<String>, // Book, BookChapter, Conference, Dataset, Journal, Patent, NULL, unknown

    #[serde(rename = "paperTitle:string")]
    paper_title: Option<String>,

    #[serde(rename = "originalTitle:string")]
    original_title: Option<String>,

    #[serde(rename = "bookTitle:string")]
    book_title: Option<String>,

    #[serde(rename = "year:long")]
    year: Option<i64>,

    #[serde(rename = "date:date")]
    date: Option<String>,

    #[serde(rename = "publisher:string")]
    publisher: Option<String>,

    #[serde(skip_serializing)]
    journal_id: Option<i64>,

    #[serde(skip_serializing)]
    conference_series_id: Option<i64>,

    #[serde(skip_serializing)]
    conference_instance_id: Option<i64>,

    #[serde(rename = "volume:string")]
    volume: Option<String>,

    #[serde(rename = "issue:string")]
    issue: Option<String>,

    #[serde(rename = "firstPage:string")]
    first_page: Option<String>,

    #[serde(rename = "lastPage:string")]
    last_page: Option<String>,

    #[serde(rename = "referenceCount:long")]
    reference_count: Option<i64>,

    #[serde(rename = "citationCount:long")]
    citation_count: Option<i64>,

    #[serde(rename = "estimatedCitation:long")]
    estimated_citation: Option<i64>,

    #[serde(rename = "createdDate:date")]
    created_date: Option<String>,

    #[serde(skip_deserializing, rename = ":LABEL")]
    label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PaperJournal {
    #[serde(rename = ":START_ID(Paper-ID)")]
    paper_id: i64,

    #[serde(rename = ":END_ID(Journal-ID)")]
    journal_id: i64,

    #[serde(rename = ":TYPE")]
    relationship_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PaperConferenceSeries {
    #[serde(rename = ":START_ID(Paper-ID)")]
    paper_id: i64,

    #[serde(rename = ":END_ID(ConferenceSeries-ID)")]
    conference_series_id: i64,

    #[serde(rename = ":TYPE")]
    relationship_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PaperConferenceInstance {
    #[serde(rename = ":START_ID(Paper-ID)")]
    paper_id: i64,

    #[serde(rename = ":END_ID(ConferenceInstance-ID)")]
    conference_instance_id: i64,

    #[serde(rename = ":TYPE")]
    relationship_type: String,
}

pub fn convert_papers() -> Result<(), Box<Error>> {
    println!("Converting Papers.txt");
    let node_output_f = File::create("Papers.csv").expect("Unable to create file Papers.csv");

    let journal_rel_f =
        File::create("PaperJournals.csv").expect("Unable to create file PaperJournals.csv");

    let conf_series_rel_f = File::create("PaperConferenceSeries.csv")
        .expect("Unable to create file PaperConferenceSeries.csv");

    let conf_inst_rel_f = File::create("PaperConferenceInstance.csv")
        .expect("Unable to create file PaperConferenceInstance.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(node_output_f);

    let mut journal_rel_wtr = csv::WriterBuilder::new().from_writer(journal_rel_f);

    let mut conf_series_rel_wtr = csv::WriterBuilder::new().from_writer(conf_series_rel_f);

    let mut conf_inst_rel_wtr = csv::WriterBuilder::new().from_writer(conf_inst_rel_f);

    let input_f = File::open("Papers.txt").expect("Unable to open file Papers.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let paper: Paper = result?;
        wtr.serialize(Paper {
            label: Some("Paper".to_string()),
            ..paper
        })?;

        if let Some(journal_id) = paper.journal_id {
            journal_rel_wtr.serialize(PaperJournal {
                paper_id: paper.paper_id,
                relationship_type: "PUBLISHED_IN".to_string(),
                journal_id: journal_id,
            })?;
        }

        if let Some(conference_series_id) = paper.conference_series_id {
            conf_series_rel_wtr.serialize(PaperConferenceSeries {
                paper_id: paper.paper_id,
                relationship_type: "PUBLISHED_AT".to_string(),
                conference_series_id: conference_series_id,
            })?;
        }

        if let Some(conference_instance_id) = paper.conference_instance_id {
            conf_inst_rel_wtr.serialize(PaperConferenceInstance {
                paper_id: paper.paper_id,
                relationship_type: "PUBLISHED_AT".to_string(),
                conference_instance_id: conference_instance_id,
            })?;
        }
    }
    wtr.flush()?;
    journal_rel_wtr.flush()?;
    conf_series_rel_wtr.flush()?;
    conf_inst_rel_wtr.flush()?;
    Ok(())
}
