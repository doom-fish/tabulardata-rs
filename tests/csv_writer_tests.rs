mod common;

use std::fs;

use tabulardata::prelude::*;

#[test]
fn csv_writer_returns_string_and_writes_files() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let options = CSVWritingOptions::new()
        .with_delimiter(';')
        .with_nil_encoding("NA");
    let csv = frame.csv_string(&options)?;
    assert!(csv.contains("Ada"));
    assert!(csv.contains(';'));

    let path = common::test_output_dir().join("writer.csv");
    frame.write_csv(&path, &options)?;
    assert_eq!(fs::read_to_string(path)?, csv);
    Ok(())
}
