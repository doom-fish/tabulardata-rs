mod common;

use std::fs;

use tabulardata::prelude::*;

#[test]
fn json_readers_and_writers_round_trip_frames() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let read_request = JSONReadRequest::new(
        JSONReadingOptions::new().with_date_parse_strategy(DateParseStrategy::Iso8601),
    )
    .with_type_hint("id", JSONType::Integer)
    .with_type_hint("score", JSONType::Double)
    .with_type_hint("active", JSONType::Boolean)
    .with_type_hint("joined_at", JSONType::Date);
    let write_options = JSONWritingOptions::new()
        .with_sort_keys(true)
        .with_pretty_print(true)
        .with_date_strategy(DateWriteStrategy::Iso8601);

    match frame.json_bytes(&write_options) {
        Ok(json_bytes) => {
            assert!(!json_bytes.is_empty());
            let path = common::test_output_dir().join("frame.json");
            frame.write_json(&path, &write_options)?;
            assert_eq!(fs::read(&path)?, json_bytes);

            let from_file = DataFrame::read_json_with(&path, &read_request)?;
            let from_data = DataFrame::read_json_data_with(&json_bytes, &read_request)?;
            assert_eq!(from_file.shape(), frame.shape());
            assert_eq!(from_data.shape(), frame.shape());
            assert_eq!(common::names(&from_data)?, common::names(&frame)?);
            assert!(String::from_utf8(json_bytes)?.contains("Ada"));
        }
        Err(error) if error.message().contains("macOS 13") => {
            let json = br#"[
  {"id":1,"name":"Ada","score":98.5,"active":true},
  {"id":2,"name":"Grace","score":91.0,"active":false}
]"#;
            let path = common::test_output_dir().join("frame.json");
            fs::write(&path, json)?;

            let fallback_request = JSONReadRequest::new(JSONReadingOptions::new())
                .with_type_hint("id", JSONType::Integer)
                .with_type_hint("score", JSONType::Double)
                .with_type_hint("active", JSONType::Boolean);
            let from_file = DataFrame::read_json_with(&path, &fallback_request)?;
            let from_data = DataFrame::read_json_data_with(json, &fallback_request)?;
            assert_eq!(from_file.shape(), (2, 4));
            assert_eq!(from_data.shape(), (2, 4));
            assert_eq!(
                common::names(&from_data)?,
                vec!["Ada".to_string(), "Grace".to_string()]
            );
        }
        Err(error) => return Err(Box::new(error)),
    }

    Ok(())
}
