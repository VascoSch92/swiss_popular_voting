extern crate core;
use std::fs::File;
use std::string::ToString;

use polars::prelude::*;
use reqwest;

use constants::{
    BALLOTS_RETURNED, BLANK_VOTING_BALLOTS, CANTONS_VOTING_NO, CANTONS_VOTING_YES, DATE_OF_VOTING,
    INVALID_VOTING_BALLOTS, OUTCOME, OVERSEAS_VOTERS, PARTICIPATION, RATIO_NO, RATIO_YES, TITLE_DE,
    TITLE_FR, TITLE_IT, TOTAL_NO, TOTAL_VOTERS, TOTAL_YES, VALID_VOTING_BALLOTS,
};
use converters::string_to_u32;
use converters::{integer_and_fraction_to_f32, ratio_to_f32};
use extractors::{
    extract_data_from_table, extract_domestic_voters, extract_information_from_summary_page,
    extract_number_votation_from_url, extract_outcome, extract_parsed_html_from,
    extract_typology_of_the_voting,
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

    let number_of_results = results.get("url").unwrap().len();
    for idx in 0..number_of_results {
        spinning_circle.set_job_title(
            format!(
                "Parsing page: {} ({} of {})",
                results.get("url").unwrap()[idx],
                idx,
                number_of_results
            )
            .as_str(),
        );

        let document = extract_parsed_html_from(&results.get("url").unwrap()[idx]);

        let mut row: Row = Row::default();

        row.no = extract_number_votation_from_url(&results.get("url").unwrap()[idx]);
        row.date_of_voting = results.get(DATE_OF_VOTING).unwrap()[idx].clone();
        row.title_it = results.get(TITLE_IT).unwrap()[idx].clone();
        row.title_fr = results.get(TITLE_FR).unwrap()[idx].clone();
        row.title_de = results.get(TITLE_DE).unwrap()[idx].clone();
        row.typology = extract_typology_of_the_voting(results.get(TITLE_IT).unwrap()[idx].clone());
        row.outcome = extract_outcome(results.get(OUTCOME).unwrap()[idx].clone());

        let table_data = extract_data_from_table(document);
        row.total_voters = string_to_u32(table_data.get(TOTAL_VOTERS));
        row.overseas_voters = string_to_u32(table_data.get(OVERSEAS_VOTERS));
        row.domestic_voters = extract_domestic_voters(row.total_voters, row.overseas_voters);
        row.ballots_returned = string_to_u32(table_data.get(BALLOTS_RETURNED));
        row.participation = ratio_to_f32(table_data.get(PARTICIPATION));
        row.blank_voting_ballots = string_to_u32(table_data.get(BLANK_VOTING_BALLOTS));
        row.invalid_voting_ballots = string_to_u32(table_data.get(INVALID_VOTING_BALLOTS));
        row.valid_voting_ballots = string_to_u32(table_data.get(VALID_VOTING_BALLOTS));
        row.total_yes = string_to_u32(table_data.get(TOTAL_YES));
        row.ratio_yes = ratio_to_f32(table_data.get(RATIO_YES));
        row.total_no = string_to_u32(table_data.get(TOTAL_NO));
        row.ratio_no = ratio_to_f32(table_data.get(RATIO_NO));
        row.cantons_voting_yes = integer_and_fraction_to_f32(table_data.get(CANTONS_VOTING_YES));
        row.cantons_voting_yes = integer_and_fraction_to_f32(table_data.get(CANTONS_VOTING_NO));

        data.update(row);
    }

    let mut df = create_dataframe_from(data);
    save_as_csv(&mut df);
}

fn create_dataframe_from(data: Data) -> DataFrame {
    let df: DataFrame = df!(
        "no" => data.no,
        DATE_OF_VOTING => data.date_of_voting,
        TITLE_IT => data.title_it,
        TITLE_FR => data.title_fr,
        TITLE_DE => data.title_de,
        "typology" => data.typology,
        TOTAL_VOTERS => data.total_voters,
        "domestic_voters" => data.domestic_voters,
        OVERSEAS_VOTERS => data.overseas_voters,
        BALLOTS_RETURNED => data.ballots_returned,
        PARTICIPATION => data.participation,
        INVALID_VOTING_BALLOTS => data.invalid_voting_ballots,
        BLANK_VOTING_BALLOTS => data.blank_voting_ballots,
        VALID_VOTING_BALLOTS => data.valid_voting_ballots,
        TOTAL_YES => data.total_yes,
        RATIO_YES => data.ratio_yes,
        TOTAL_NO => data.total_no,
        RATIO_NO => data.ratio_no,
        CANTONS_VOTING_YES => data.cantons_voting_yes,
        CANTONS_VOTING_NO => data.cantons_voting_no,
        OUTCOME => data.outcome,
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

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(df)
        .expect("Could not save the dataframe");
}
