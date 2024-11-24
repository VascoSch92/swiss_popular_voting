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
use crate::converters::{
    convert_date_to_us_format, integer_and_fraction_to_f32, ratio_to_f32, string_to_u32,
};
use crate::data::{Data, Row};

pub fn execute_extractions_of_data() -> Data {
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
        row.kind = extract_typology_of_the_voting(results.get(TITLE_IT).unwrap()[idx].clone());
        row.outcome = extract_outcome(results.get(OUTCOME).unwrap()[idx].clone());

        let table_data = extract_data_from_table(document.clone());
        row.recommendation = extract_recommendation(table_data.get("supplementary_information"));
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
    println!();
    data
}

fn extract_parsed_html_from(url: &String) -> Html {
    let response = reqwest::blocking::get(url.clone());
    let html_content = response.unwrap().text().unwrap();

    Html::parse_document(&html_content)
}

fn extract_information_from_summary_page() -> HashMap<&'static str, Vec<String>> {
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

fn extract_number_votation_from_url(voting_hyperlink: &String) -> Option<u32> {
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

fn extract_typology_of_the_voting(title: String) -> String {
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

fn extract_outcome(outcome: String) -> Option<String> {
    match outcome.as_str() {
        "L'oggetto è stato accettato" => Some("accepted".to_string()),
        "L'oggetto è stato respinto" => Some("not accepted".to_string()),
        _ => None,
    }
}

fn extract_data_from_table(document: Html) -> HashMap<&'static str, String> {
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

    let selector = scraper::Selector::parse("a").unwrap();

    // Find the link with the desired text
    for element in document.select(&selector) {
        if let Some(text) = element.text().next() {
            if text.trim() == "Informazioni supplementari sull'iniziativa popolare" {
                if let Some(href) = element.value().attr("href") {
                    data.insert(
                        "supplementary_information",
                        format!("{}{}", "https://www.bk.admin.ch", href.to_string(),),
                    );
                    break;
                }
            }
        }
    }
    data
}

fn extract_recommendation(url: Option<&String>) -> Option<String> {
    if url.is_none() {
        return None;
    }
    let supplementary_info = extract_parsed_html_from(url.unwrap());
    let td_selector = scraper::Selector::parse("td").unwrap();

    // Iterate over all <td> elements to find the one containing "Raccomandazione"
    for element in supplementary_info.select(&td_selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if text.contains("Raccomandazione") {
            if text.contains("Rigetto") {
                return Some("reject".to_string());
            } else {
                return Some("accept".to_string());
            }
        }
    }
    None
}

fn extract_domestic_voters(
    total_voters: Option<u32>,
    overseas_voters: Option<u32>,
) -> Option<u32> {
    if !total_voters.is_none() && !overseas_voters.is_none() {
        return Some(total_voters.unwrap() + overseas_voters.unwrap());
    }
    None
}
