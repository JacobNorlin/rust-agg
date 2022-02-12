use data::data_frame::DataFrame;

mod data;
fn main() {
    let raw_data = "a,b\na,2";

    let csv_data: Vec<Vec<&str>> = raw_data
        .split("\n")
        .map(|row| row.split(",").collect())
        .collect();

    let frame = DataFrame::new(&csv_data, true);

    for field in frame.schema().fields() {
        for row in frame.rows() {
            print!("{:?}", field.read(row));
        }
    }
    print!("{:?}", frame.schema())
}
