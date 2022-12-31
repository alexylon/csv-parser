#[macro_use]
extern crate pest_derive;

mod numeric;
mod pest_csv_parser;
mod csv_crate_parser;


fn main() {
    pest_csv_parser::pest_csv_parser();
    csv_crate_parser::csv_crate_parser();
}
