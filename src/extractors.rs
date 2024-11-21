use regex::Regex;
use scraper::Html;

use crate::constants::{
    BIANCHE, BOLLETTINI_RIENTRATI, NO, NULLE, PARTECIPAZIONE, SCHEDE_DI_VOTO_VALIDE, SI,
    SVIZZERI_ALL_ESTERO, TOTALE_ELETTORI, VOTO_DEI_CANTONI_NO, VOTO_DEI_CANTONI_SI,
};
use crate::converters::{
    convert_date_to_us_format, integer_and_fraction_to_f32, ratio_to_f32, string_to_u32,
};
use crate::Row;

pub fn extract_parsed_html_from(url: &String) -> Html {
    let response = reqwest::blocking::get(url.clone());
    let html_content = response.unwrap().text().unwrap();

    let document = Html::parse_document(&html_content);
    document
}

pub fn extract_information_from_summary_page() -> Vec<(String, String, String, String)> {
    let document = extract_parsed_html_from(
        &"https://www.bk.admin.ch/ch/i/pore/va/vab_2_2_4_1_gesamt.html".to_string(),
    );

    let row_selector = scraper::Selector::parse("tr").unwrap();
    let link_selector = scraper::Selector::parse("a").unwrap();

    // Store results in a vector of tuples
    let mut results: Vec<(String, String, String, String)> = Vec::new();

    // Iterate over rows (skip the first row, which is the header)
    for row in document.select(&row_selector).skip(1) {
        let columns: Vec<_> = row.select(&link_selector).collect();

        // Extract values if the structure matches
        if columns.len() >= 2 {
            let url = format!(
                "{}{}",
                "https://www.bk.admin.ch/ch/i/pore/va/",
                columns[1]
                    .value()
                    .attr("href")
                    .unwrap_or_default()
                    .to_string()
            );
            let date =
                convert_date_to_us_format(columns[0].text().collect::<String>().trim()).unwrap();
            let title = columns[1].text().collect::<String>().trim().to_string();

            let mut esito = String::new();
            for element in row.text().collect::<Vec<_>>() {
                if element.starts_with("L'oggetto") {
                    esito = element.trim().to_string();
                    break;
                }
            }

            results.push((url, date, title, esito));
        }
    }
    results
}

pub fn extract_number_votation_from_url(voting_hyperlink: &String) -> Option<u32> {
    // Define the regex pattern to capture the number after "det"
    let re = Regex::new(r"det(\d+)\.html").unwrap();

    // Check if the regex matches and extract the number
    if let Some(captures) = re.captures(voting_hyperlink.as_str()) {
        if let Some(number) = captures.get(1) {
            return string_to_u32(number.as_str().to_string());
        }
    }
    None
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

pub fn extract_data_from_table(document: Html, row: &mut Row) {
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
    while position < table_elements.len() {
        match table_elements[position].as_str() {
            TOTALE_ELETTORI => {
                position += 1;
                row.total_voters = Some(
                    table_elements[position]
                        .replace("'", "")
                        .parse::<u32>()
                        .unwrap(),
                );
            }
            SVIZZERI_ALL_ESTERO => {
                position += 1;
                row.overseas_voters = Some(
                    table_elements[position]
                        .replace("'", "")
                        .parse::<u32>()
                        .unwrap(),
                );
            }
            BOLLETTINI_RIENTRATI => {
                position += 1;
                row.ballots_returned = string_to_u32(table_elements[position].clone());
            }
            PARTECIPAZIONE => {
                position += 1;
                row.participation = ratio_to_f32(table_elements[position].clone());
            }
            BIANCHE => {
                position += 1;
                row.blank_voting_ballots = string_to_u32(table_elements[position].clone());
            }
            NULLE => {
                position += 1;
                row.invalid_voting_ballots = string_to_u32(table_elements[position].clone());
            }
            SCHEDE_DI_VOTO_VALIDE => {
                position += 1;
                row.valid_voting_ballots = string_to_u32(table_elements[position].clone());
            }
            SI => {
                position += 1;
                row.total_yes = string_to_u32(table_elements[position].clone());

                position += 1;
                row.ratio_yes = ratio_to_f32(table_elements[position].clone());
            }
            NO => {
                position += 1;
                row.total_no = string_to_u32(table_elements[position].clone());

                position += 1;
                row.ratio_no = ratio_to_f32(table_elements[position].clone());
            }
            VOTO_DEI_CANTONI_SI => {
                position += 1;
                row.cantons_voting_yes =
                    integer_and_fraction_to_f32(table_elements[position].clone());
            }
            VOTO_DEI_CANTONI_NO => {
                position += 1;
                row.cantons_voting_no =
                    integer_and_fraction_to_f32(table_elements[position].clone());
            }
            _ => {}
        }
        position += 1;
    }
}
