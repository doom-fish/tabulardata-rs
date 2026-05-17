mod common;

use std::fs;

use tabulardata::prelude::*;

#[test]
fn csv_data_and_sframe_helpers_work() -> Result<(), Box<dyn std::error::Error>> {
    let csv = b"id,name,score\n1,Ada,98.5\n2,Grace,91.0\n";
    let frame = DataFrame::from_csv_data(csv, CSVReadingOptions::new())?;
    assert_eq!(frame.shape(), (2, 3));

    let json_error = JSONReadingError::from_error(
        &DataFrame::from_json_string("{", JSONReadingOptions::new()).unwrap_err(),
    );
    assert!(!json_error.message.is_empty());

    let sframe_dir = common::test_output_dir().join("empty.sframe");
    fs::create_dir_all(&sframe_dir)?;
    let error = DataFrame::read_sframe_with(
        &sframe_dir,
        &SFrameReadRequest::new().with_columns(["id"]).with_rows(0..1),
    )
    .unwrap_err();
    let parsed = SFrameReadingError::parse(error.message());
    assert!(matches!(
        parsed,
        SFrameReadingError::MissingArchive
            | SFrameReadingError::BadArchive(_)
            | SFrameReadingError::UnsupportedArchive(_)
            | SFrameReadingError::Message(_)
    ));
    Ok(())
}
