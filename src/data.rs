use crate::constants::{
    BALLOTS_RETURNED, BLANK_VOTING_BALLOTS, CANTONS_VOTING_NO, CANTONS_VOTING_YES, DATE_OF_VOTING,
    INVALID_VOTING_BALLOTS, OUTCOME, OVERSEAS_VOTERS, PARTICIPATION, RATIO_NO, RATIO_YES, TITLE_DE,
    TITLE_FR, TITLE_IT, TOTAL_NO, TOTAL_VOTERS, TOTAL_YES, VALID_VOTING_BALLOTS,
};
use polars::df;
use polars::frame::DataFrame;
use std::fs::File;
use polars::prelude::*;

#[derive(Default, Debug)]
pub struct Row {
    pub no: Option<u32>,
    pub date_of_voting: String,
    pub title_it: String,
    pub title_fr: String,
    pub title_de: String,
    pub kind: String,
    pub recommendation: Option<String>,
    pub total_voters: Option<u32>,
    pub domestic_voters: Option<u32>,
    pub overseas_voters: Option<u32>,
    pub ballots_returned: Option<u32>,
    pub participation: Option<f32>,
    pub invalid_voting_ballots: Option<u32>,
    pub blank_voting_ballots: Option<u32>,
    pub valid_voting_ballots: Option<u32>,
    pub total_yes: Option<u32>,
    pub ratio_yes: Option<f32>,
    pub total_no: Option<u32>,
    pub ratio_no: Option<f32>,
    pub cantons_voting_yes: Option<f32>,
    pub cantons_voting_no: Option<f32>,
    pub outcome: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct Data {
    pub no: Vec<Option<u32>>,
    pub date_of_voting: Vec<String>,
    pub title_it: Vec<String>,
    pub title_fr: Vec<String>,
    pub title_de: Vec<String>,
    pub kind: Vec<String>,
    pub recommendation: Vec<Option<String>>,
    pub total_voters: Vec<Option<u32>>,
    pub domestic_voters: Vec<Option<u32>>,
    pub overseas_voters: Vec<Option<u32>>,
    pub ballots_returned: Vec<Option<u32>>,
    pub participation: Vec<Option<f32>>,
    pub invalid_voting_ballots: Vec<Option<u32>>,
    pub blank_voting_ballots: Vec<Option<u32>>,
    pub valid_voting_ballots: Vec<Option<u32>>,
    pub total_yes: Vec<Option<u32>>,
    pub ratio_yes: Vec<Option<f32>>,
    pub total_no: Vec<Option<u32>>,
    pub ratio_no: Vec<Option<f32>>,
    pub cantons_voting_yes: Vec<Option<f32>>,
    pub cantons_voting_no: Vec<Option<f32>>,
    pub outcome: Vec<Option<String>>,
}

impl Data {
    pub fn update(&mut self, row: Row) {
        self.no.push(row.no);
        self.date_of_voting.push(row.date_of_voting);
        self.title_it.push(row.title_it);
        self.title_fr.push(row.title_fr);
        self.title_de.push(row.title_de);
        self.kind.push(row.kind);
        self.recommendation.push(row.recommendation);
        self.total_voters.push(row.total_voters);
        self.domestic_voters.push(row.domestic_voters);
        self.overseas_voters.push(row.overseas_voters);
        self.ballots_returned.push(row.ballots_returned);
        self.participation.push(row.participation);
        self.invalid_voting_ballots.push(row.invalid_voting_ballots);
        self.blank_voting_ballots.push(row.blank_voting_ballots);
        self.valid_voting_ballots.push(row.valid_voting_ballots);
        self.total_yes.push(row.total_yes);
        self.ratio_yes.push(row.ratio_yes);
        self.total_no.push(row.total_no);
        self.ratio_no.push(row.ratio_no);
        self.cantons_voting_yes.push(row.cantons_voting_yes);
        self.cantons_voting_no.push(row.cantons_voting_no);
        self.outcome.push(row.outcome);
    }
}

pub fn create_dataframe_from(data: Data) -> DataFrame {
    let df: DataFrame = df!(
        "no" => data.no,
        DATE_OF_VOTING => data.date_of_voting,
        TITLE_IT => data.title_it,
        TITLE_FR => data.title_fr,
        TITLE_DE => data.title_de,
        "kind" => data.kind,
        "recommendation" => data.recommendation,
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

    df
}

pub fn save_as_csv(df: &mut DataFrame) {
    // Save the DataFrame to a CSV file
    let mut file = File::create("data.csv").expect("could not create file");

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(df)
        .expect("Could not save the dataframe");
}
