extern crate core;

use std::time::Instant;

use log::info;

use data::{create_dataframe_from, save_as_csv};
use extractors::execute_extractions_of_data;

mod constants;
mod converters;
mod data;
mod extractors;

fn main() {
    env_logger::init();
    info!("Start creation of dataset 'Swiss Popular Voting'");
    let start_time = Instant::now();

    info!("Extraction of data started");
    let extraction_time = Instant::now();
    let data = execute_extractions_of_data();
    info!("Extraction from data completed in {:.2?}", extraction_time.elapsed());

    let df_time = Instant::now();
    let mut df = create_dataframe_from(data);
    info!(
        "Created dataframe with {} rows and {} columns from extracted data in {:.2?}",
        df.shape().0,
        df.shape().1,
        df_time.elapsed(),
    );

    save_as_csv(&mut df);
    info!("Dataset successfully saved");

    info!("Program completed in {:.2?}", start_time.elapsed())
}
