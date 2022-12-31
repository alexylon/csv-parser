use std::fs;
use pest::error::Error;
use pest::Parser;
use crate::pest_csv_parser::create_big_csv::create_big_csv;
use crate::pest_csv_parser::csv_parser_semicolon::Rule as RuleSemicolon;
use crate::pest_csv_parser::csv_parser_comma::Rule as RuleComma;

#[path = "./create_big_csv.rs"]
pub mod create_big_csv;

#[path = "./parsers/csv_parser_semicolon.rs"]
pub mod csv_parser_semicolon;
#[path = "./parsers/csv_parser_comma.rs"]
pub mod csv_parser_comma;

pub fn pest_csv_parser() {
//     let source = "Price;\"Fo\"\"od\"
// 4,6;\"\"\"App\"\"le\"\"\"
// 10,9;\"Pear;Peach;Orange\"
// ";

    // let mut unparsed_file = fs::read_to_string("src/test_files/test_comma.csv").expect("cannot read file");
    println!("Running pest parser...");
    let t0 = std::time::Instant::now();

    let mut unparsed_file = create_big_csv();

    let normalized_string = unparsed_file.replace("\r\n", "\n").replace("\r", "\n");
    let lines: Vec<&str> = normalized_string.split("\n").collect();
    let mut separator = ";";

    if let Some(line1) = lines.get(0) {
        if line1.find(",") != None { separator = "," }
        if line1 == &format!("$csv{}value", separator) {
            if let Some(line2) = lines.get(1) {
                let line2_cell1: String = line2.chars().skip(0).take(5).collect();
                if line2_cell1 == format!("file{}", separator) {
                    let imported_file_path: String = line2.chars().skip(5).take(line2.len() - 5).collect();
                    match fs::read_to_string(&imported_file_path) {
                        Ok(_) => { unparsed_file = fs::read_to_string(&imported_file_path).expect("cannot read file"); }
                        Err(_) => { println!("There is no such file"); }
                    }
                }
            }
        }
    }

    #[allow(unused_assignments)]
        let mut rows: Vec<Vec<String>> = vec!();
    let mut headers: Vec<String> = vec!();
    let mut records: Vec<Vec<String>> = vec!();

    if separator == ";" {
        match get_rows_semicolon(unparsed_file) {
            Ok(rows_semicolon) => { rows = rows_semicolon }
            Err(error) => { println!("Error: {}", &error.to_string()); }
        }
    } else {
        match get_rows_comma(unparsed_file) {
            Ok(rows_comma) => { rows = rows_comma }
            Err(error) => { println!("Error: {}", &error.to_string()); }
        }
    }

    let mut row_number: usize = rows.len();

    if let Some(row1) = rows.get(0) {
        headers = row1.to_vec();
        rows.remove(0);
    }

    for row in rows {
        records.push(row);
    }

    // println!("headers: {:?}", headers);
    // println!("records: {:?}", records);

    println!("{} rows parsed: ", row_number);
    println!("pest parser elapsed: {}s", t0.elapsed().as_secs_f64());
    println!();
}

fn get_rows_semicolon(unparsed_file: String) -> Result<Vec<Vec<String>>, Error<RuleSemicolon>> {
    let mut rows: Vec<Vec<String>> = vec!();
    match csv_parser_semicolon::CSVParserSemicolon::parse(RuleSemicolon::file, &unparsed_file) {
        Ok(mut f) => {
            let file = f.next().unwrap();

            for record in file.into_inner() {
                match record.as_rule() {
                    RuleSemicolon::record => {
                        let fields = record.into_inner().map(|field| {
                            // println!("field: {:?}", &field);
                            let str = field.as_str().to_string();
                            clean_quotes(str)
                        }).collect();

                        rows.push(fields);
                    }

                    RuleSemicolon::EOI => (),
                    _ => unreachable!(),
                }
            }
        }
        Err(error) => { return Err(error); }
    }
    Ok(rows)
}

fn get_rows_comma(unparsed_file: String) -> Result<Vec<Vec<String>>, Error<RuleComma>> {
    let mut rows: Vec<Vec<String>> = vec!();
    match csv_parser_comma::CSVParserComma::parse(RuleComma::file, &unparsed_file) {
        Ok(mut f) => {
            let file = f.next().unwrap();

            for record in file.into_inner() {
                match record.as_rule() {
                    RuleComma::record => {
                        let fields = record.into_inner().map(|field| {
                            let str = field.as_str().to_string();
                            clean_quotes(str)
                        }).collect();

                        rows.push(fields);
                    }

                    RuleComma::EOI => (),
                    _ => unreachable!(),
                }
            }
        }
        Err(error) => { return Err(error); }
    }
    Ok(rows)
}

fn clean_quotes(mut str: String) -> String {
    if str.len() > 0 {
        let chars_number = str.chars().count();
        let first_char: String = str.chars().skip(0).take(1).collect();
        let last_char: String = str.chars().skip(chars_number - 1).take(1).collect();

        if first_char == "\"" && last_char == "\"" {
            str = rem_first_and_last_char(&str);
        }

        str = str.replace("\"\"", "\"");
    }
    str
}

fn rem_first_and_last_char(value: &str) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

fn _parse_line(string_record: Vec<String>) {
    let mut cell_number: usize = 0;
    let mut start_position: usize = 0;
    for cell in string_record.iter() {
        let end_position = start_position + &cell.len();
        println!("cell_number: {:?}", &cell_number);
        println!("cell: {:?}", &cell);
        println!("start_position: {:?}", &start_position);
        println!("end_position: {:?}", &end_position);
        cell_number = cell_number + 1;
        start_position = end_position + 1;
    }
}
