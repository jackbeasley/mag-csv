extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod affiliations;
mod authors;
mod conference_instances;
mod conference_series;
mod field_of_study_children;
mod fields_of_study;
mod journal;
mod paper_author_affiliations;
mod paper_fields_of_study;
mod paper_languages;
mod paper_references;
mod papers;

use std::process;
use std::thread;

fn main() {
    let paper_child = thread::spawn(move || {
        if let Err(err) = papers::convert_papers() {
            println!("{}", err);
            panic!()
        }
    });

    let journal_child = thread::spawn(move || {
        if let Err(err) = journal::convert_journals() {
            println!("{}", err);
            panic!()
        }
    });

    let author_child = thread::spawn(move || {
        if let Err(err) = authors::convert_authors() {
            println!("{}", err);
            panic!()
        }
    });

    let affiliation_child = thread::spawn(move || {
        if let Err(err) = affiliations::convert_affiliations() {
            println!("{}", err);
            panic!()
        }
    });

    let ci_child = thread::spawn(move || {
        if let Err(err) = conference_instances::convert_conference_instances() {
            println!("{}", err);
            panic!();
        }
    });

    let cs_child = thread::spawn(move || {
        if let Err(err) = conference_series::convert_conference_series() {
            println!("{}", err);
            panic!();
        }
    });

    let fos_child = thread::spawn(move || {
        if let Err(err) = fields_of_study::convert_fields_of_study() {
            println!("{}", err);
            panic!();
        }
    });

    let fosc_child = thread::spawn(move || {
        if let Err(err) = field_of_study_children::convert_field_of_study_children() {
            println!("{}", err);
            panic!();
        }
    });

    let paa_child = thread::spawn(move || {
        if let Err(err) = paper_author_affiliations::convert_paper_author_affilations() {
            println!("{}", err);
            panic!();
        }
    });

    let pfos_child = thread::spawn(move || {
        if let Err(err) = paper_fields_of_study::convert_paper_fields_of_study() {
            println!("{}", err);
            panic!();
        }
    });

    let pl_child = thread::spawn(move || {
        if let Err(err) = paper_languages::convert_paper_languages() {
            println!("{}", err);
            panic!();
        }
    });

    let pr_child = thread::spawn(move || {
        if let Err(err) = paper_references::convert_paper_references() {
            println!("{}", err);
            panic!();
        }
    });

    if let Err(err) = paper_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = journal_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = author_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = ci_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = cs_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = fos_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = fosc_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = paa_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = pfos_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = pl_child.join() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = pr_child.join() {
        println!("{}", err);
        process::exit(1);
    }
}
