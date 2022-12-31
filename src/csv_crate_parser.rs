use crate::pest_csv_parser::create_big_csv::create_big_csv;
use csv::{StringRecord};

#[path = "./create_big_csv.rs"]
pub mod create_big_csv;

pub fn csv_crate_parser() {
    println!("Running csv crate parsing...");
    let t0 = std::time::Instant::now();

    let unparsed_file = create_big_csv();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(unparsed_file.as_bytes());

    let mut headers: StringRecord = StringRecord::new();
    let mut records: Vec<StringRecord> = vec!();

    let mut row_number: usize = 1;
    for record in reader.records() {
        match record {
            Ok(string_record) => {
                if row_number == 1 {
                    headers = string_record;
                } else {
                    records.push(string_record);
                }
            }
            Err(error) => { println!("Error: {}", &error.to_string()); }
        }
        row_number = row_number + 1;
    }

    row_number = row_number - 1;

    // println!("headers2: {:?}", headers);
    // println!("records2: {:?}", records);

    println!("{} rows parsed: ", row_number);
    println!("csv crate parsing elapsed: {}s", t0.elapsed().as_secs_f64());
    println!();
}