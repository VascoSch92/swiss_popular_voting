use polars::export::chrono::TimeDelta;

#[derive(Default, Debug)]
pub struct Row {
    pub no: Option<u32>,
    pub date_of_voting: String,
    pub title_it: String,
    pub title_fr: String,
    pub title_de: String,
    pub typology: String,
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
    pub typology: Vec<String>,
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
        self.typology.push(row.typology);
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
