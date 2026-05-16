#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = support::csv_fixture("reader.csv");
    let request = CSVReadRequest::new(
        CSVReadingOptions::new().with_date_parse_strategy(DateParseStrategy::Ymd),
    )
    .with_columns(["id", "name", "joined_at"])
    .with_rows(0..2)
    .with_type_hint("id", CSVType::Integer)
    .with_type_hint("joined_at", CSVType::Date);
    let frame = DataFrame::read_csv_with(&path, &request)?;
    println!("csv subset shape = {:?}", frame.shape());
    Ok(())
}
