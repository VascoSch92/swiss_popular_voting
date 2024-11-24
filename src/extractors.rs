use collections::HashMap;
use std::collections;

use regex::Regex;
use scraper::Html;

use crate::constants::{
    BALLOTS_RETURNED, BLANK_VOTING_BALLOTS, CANTONS_VOTING_NO, CANTONS_VOTING_YES, DATE_OF_VOTING,
    INVALID_VOTING_BALLOTS, OUTCOME, OVERSEAS_VOTERS, PARTICIPATION, RATIO_NO, RATIO_YES, TITLE_DE,
    TITLE_FR, TITLE_IT, TOTAL_NO, TOTAL_VOTERS, TOTAL_YES, URL_SUMMARY_PAGE_DE,
    URL_SUMMARY_PAGE_FR, URL_SUMMARY_PAGE_IT, VALID_VOTING_BALLOTS,
};
use crate::converters::{convert_date_to_us_format, string_to_u32};
use crate::Row;

pub fn extract_parsed_html_from(url: &String) -> Html {
    let response = reqwest::blocking::get(url.clone());
    let html_content = response.unwrap().text().unwrap();

    let document = Html::parse_document(&html_content);
    document
}

pub fn extract_information_from_summary_page() -> HashMap<&'static str, Vec<String>> {
    let document = extract_parsed_html_from(&URL_SUMMARY_PAGE_IT.to_string());

    let row_selector = scraper::Selector::parse("tr").unwrap();
    let link_selector = scraper::Selector::parse("a").unwrap();

    // Store results in a vector of tuples
    let mut data: HashMap<&str, Vec<String>> = HashMap::from([
        ("url", Vec::new()),
        (DATE_OF_VOTING, Vec::new()),
        (OUTCOME, Vec::new()),
    ]);

    data.insert(TITLE_IT, extract_title(URL_SUMMARY_PAGE_IT));
    data.insert(TITLE_DE, extract_title(URL_SUMMARY_PAGE_DE));
    data.insert(TITLE_FR, extract_title(URL_SUMMARY_PAGE_FR));

    // Iterate over rows (skip the first row, which is the header)
    for row in document.select(&row_selector).skip(1) {
        let columns: Vec<_> = row.select(&link_selector).collect();

        // Extract values if the structure matches
        if columns.len() >= 2 {
            data.get_mut("url").unwrap().push(format!(
                "{}{}",
                "https://www.bk.admin.ch/ch/i/pore/va/",
                columns[1]
                    .value()
                    .attr("href")
                    .unwrap_or_default()
                    .to_string()
            ));
            data.get_mut(DATE_OF_VOTING).unwrap().push(
                convert_date_to_us_format(columns[0].text().collect::<String>().trim()).unwrap(),
            );

            let mut esito = String::new();
            for element in row.text().collect::<Vec<_>>() {
                if element.starts_with("L'oggetto") {
                    esito = element.trim().to_string();
                    break;
                }
            }
            data.get_mut(OUTCOME).unwrap().push(esito);
        }
    }
    data
}

fn extract_title(url: &str) -> Vec<String> {
    let document = extract_parsed_html_from(&url.to_string());

    let row_selector = scraper::Selector::parse("tr").unwrap();
    let link_selector = scraper::Selector::parse("a").unwrap();

    let mut titles: Vec<String> = Vec::new();
    for row in document.select(&row_selector).skip(1) {
        let columns: Vec<_> = row.select(&link_selector).collect();
        if columns.len() >= 2 {
            titles.push(columns[1].text().collect::<String>().trim().to_string());
        }
    }
    titles
}

pub fn extract_number_votation_from_url(voting_hyperlink: &String) -> Option<u32> {
    // Define the regex pattern to capture the number after "det"
    let re = Regex::new(r"det(\d+)\.html").unwrap();

    // Check if the regex matches and extract the number
    if let Some(captures) = re.captures(voting_hyperlink.as_str()) {
        if let Some(number) = captures.get(1) {
            return string_to_u32(Some(&number.as_str().to_string()));
        }
    }
    None
}

pub fn extract_typology_of_the_voting(title: String) -> String {
    if title.contains("Iniziativa") {
        "initiative".to_string()
    } else if title.contains("Decreto") {
        "decree".to_string()
    } else if title.contains("Legge") {
        "referendum".to_string()
    } else if title.contains("Controprogetto") {
        "counterproposal".to_string()
    } else {
        "".to_string()
    }
}

pub fn extract_outcome(outcome: String) -> Option<String> {
    match outcome.as_str() {
        "L'oggetto è stato accettato" => Some("accepted".to_string()),
        "L'oggetto è stato respinto" => Some("not accepted".to_string()),
        _ => None,
    }
}

pub fn extract_date_of_voting_from_url(url: &str) -> Option<String> {
    let re = Regex::new(r"/(\d{8})/").unwrap();
    if let Some(captures) = re.captures(url) {
        let number = &captures[1];
        let formatted_date = format!(
            "{}-{}-{}",
            &number[0..4], // Year
            &number[4..6], // Month
            &number[6..8]  // Day
        );
        return Some(formatted_date);
    }
    None
}

pub fn extract_data_from_table(document: Html) -> HashMap<&'static str, String> {
    // Selector for all td elements
    let td_selector = scraper::Selector::parse("td").unwrap();

    // Variables to store extracted values
    let mut table_elements: Vec<String> = vec![];

    // Iterate through each td element
    for element in document.select(&td_selector) {
        table_elements.push(
            element
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string(),
        );
    }

    let mut position: usize = 0;
    let mut data: HashMap<&str, String> = HashMap::new();
    while position < table_elements.len() {
        match table_elements[position].as_str() {
            "Totale elettori" => {
                position += 1;
                data.insert(TOTAL_VOTERS, table_elements[position].clone());
            }
            "di cui Svizzeri all'estero" => {
                position += 1;
                data.insert(OVERSEAS_VOTERS, table_elements[position].clone());
            }
            "Bollettini rientrati" => {
                position += 1;
                data.insert(BALLOTS_RETURNED, table_elements[position].clone());
            }
            "Partecipazione" => {
                position += 1;
                data.insert(PARTICIPATION, table_elements[position].clone());
            }
            "bianche" => {
                position += 1;
                data.insert(BLANK_VOTING_BALLOTS, table_elements[position].clone());
            }
            "nulle" => {
                position += 1;
                data.insert(INVALID_VOTING_BALLOTS, table_elements[position].clone());
            }
            "Schede di voto valide" => {
                position += 1;
                data.insert(VALID_VOTING_BALLOTS, table_elements[position].clone());
            }
            "Sì" => {
                position += 1;
                data.insert(TOTAL_YES, table_elements[position].clone());

                position += 1;
                data.insert(RATIO_YES, table_elements[position].clone());
            }
            "No" => {
                position += 1;
                data.insert(TOTAL_NO, table_elements[position].clone());

                position += 1;
                data.insert(RATIO_NO, table_elements[position].clone());
            }
            "Voto dei Cantoni sì" => {
                position += 1;
                data.insert(CANTONS_VOTING_YES, table_elements[position].clone());
            }
            "Voto dei Cantoni no" => {
                position += 1;
                data.insert(CANTONS_VOTING_NO, table_elements[position].clone());
            }
            _ => {}
        }
        position += 1;
    }
    data
}

pub fn extract_domestic_voters(
    total_voters: Option<u32>,
    overseas_voters: Option<u32>,
) -> Option<u32> {
    if !total_voters.is_none() && !overseas_voters.is_none() {
        return Some(total_voters.unwrap() + overseas_voters.unwrap());
    }
    None
}
