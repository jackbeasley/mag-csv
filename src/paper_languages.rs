use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PaperLanguage {
    paper_id: i64,
    language_code: String,
}

pub fn convert_paper_languages() -> Result<(), Box<Error>> {
    println!("Converting PaperLanguages.txt");
    let output_f = File::create("PaperLanguages.csv").expect("Unable to create file PaperLanguages.csv");

    let mut wtr = csv::WriterBuilder::new().from_writer(output_f);

    let input_f = File::open("PaperLanguages.txt").expect("Unable to open file PaperLanguages.txt");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(input_f);

    for result in rdr.deserialize() {
        let paper_language: PaperLanguage = result?;
        wtr.serialize(paper_language)?;
    }
    wtr.flush()?;
    Ok(())
}
