mod data;
mod viz;

use data::data_frame::DataFrame;

pub fn create_data_frame(raw_data: String, first_row_header: bool) -> DataFrame {
    let csv_data: Vec<Vec<&str>> = raw_data
        .split("\n")
        .map(|row| row.split(",").collect())
        .collect();

    DataFrame::new(&csv_data, first_row_header)
}
