extern crate core;
use std::fs::File;
use std::string::ToString;

use polars::prelude::*;
use reqwest;

use extractors::{
    extract_data_from_table, extract_information_from_summary_page,
    extract_number_votation_from_url, extract_parsed_html_from,
};
use tools::{Data, Row};

mod constants;
mod converters;
mod extractors;
mod tools;

fn main() {
    let results = extract_information_from_summary_page();

    let mut data: Data = Data::default();
    let mut spinning_circle = progress::SpinningCircle::new();
    for (idx, (url, date_of_voting, title, outcome)) in results.clone().into_iter().enumerate() {
        spinning_circle.set_job_title(
            format!("Parsing page: {} ({} of {})", url, idx, results.len(),).as_str(),
        );

        let document = extract_parsed_html_from(&url);

        let mut row: Row = Row::default();

        row.no = extract_number_votation_from_url(&url);
        row.date_of_voting = date_of_voting;
        row.title = title;
        row.outcome = match outcome.as_str() {
            "L'oggetto è stato accettato" => Some("accepted".to_string()),
            "L'oggetto è stato respinto" => Some("not accepted".to_string()),
            _ => None,
        };
        extract_data_from_table(document, &mut row);

        data.update(row);
    }

    let mut df = create_dataframe_from(data);
    save_as_csv(&mut df);
}

fn create_dataframe_from(data: Data) -> DataFrame {
    let df: DataFrame = df!(
        "no" => data.no,
        "date_of_voting" => data.date_of_voting,
        "title" => data.title,
        "total_voters" => data.total_voters,
        "domestic_voters" => data.domestic_voters,
        "overseas_voters" => data.overseas_voters,
        "ballots_returned" => data.ballots_returned,
        "participation" => data.participation,
        "invalid_voting_ballots" => data.invalid_voting_ballots,
        "blank_voting_ballots" => data.blank_voting_ballots,
        "valid_voting_ballots" => data.valid_voting_ballots,
        "total_yes" => data.total_yes,
        "ratio_yes" => data.ratio_yes,
        "total_no" => data.total_no,
        "ratio_no" => data.ratio_no,
        "cantons_voting_yes" => data.cantons_voting_yes,
        "cantons_voting_no" => data.cantons_voting_no,
        "outcome" => data.outcome,
    )
    .unwrap();

    println!(
        "Created dataframe with {} rows and {} columns from data",
        df.shape().0,
        df.shape().1,
    );

    df
}

fn save_as_csv(df: &mut DataFrame) {
    // Save the DataFrame to a CSV file
    let mut file = File::create("data.csv").expect("could not create file");

    _ = CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(df)
        .expect("Could not save the dataframe");
}
