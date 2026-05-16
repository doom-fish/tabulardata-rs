mod common;

use tabulardata::prelude::*;

#[test]
fn column_encoding_round_trips_strings() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = common::fixture_frame()?;
    frame.encode_column("name", ColumnElementType::String, ColumnCodec::Json)?;
    assert_eq!(frame.any_column("name")?.type_name, "Data");

    frame.decode_column("name", ColumnElementType::String, ColumnCodec::Json)?;
    assert_eq!(frame.any_column("name")?.type_name, "String");
    assert_eq!(
        common::names(&frame)?,
        vec!["Ada", "Grace", "Linus", "Barbara"]
    );
    Ok(())
}
