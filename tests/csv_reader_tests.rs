mod common;

use tabulardata::prelude::*;

#[test]
fn csv_reader_supports_projection_ranges_and_type_hints() -> Result<(), Box<dyn std::error::Error>>
{
    let path = common::csv_fixture_path("reader.csv");
    let request = CSVReadRequest::new(
        CSVReadingOptions::new().with_date_parse_strategy(DateParseStrategy::Ymd),
    )
    .with_columns(["id", "name", "joined_at", "score"])
    .with_rows(1..3)
    .with_type_hint("id", CSVType::Integer)
    .with_type_hint("joined_at", CSVType::Date)
    .with_type_hint("score", CSVType::Double);

    let frame = DataFrame::read_csv_with(&path, &request)?;
    assert_eq!(frame.shape(), (2, 4));
    assert_eq!(frame.any_column("joined_at")?.type_name, "Date");
    Ok(())
}
