mod common;

use tabulardata::prelude::*;

#[test]
fn any_row_snapshots_can_be_appended_and_replaced() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = common::fixture_frame()?;
    let first = frame.row(0)?;
    assert_eq!(
        first.get("name"),
        Some(&AnyValue::String("Ada".to_string()))
    );

    frame.append_row(
        &AnyRow::new()
            .with_value("id", 5_i64)
            .with_value("name", "Edsger")
            .with_value("team", "algorithms")
            .with_value("score", 96.0)
            .with_value("active", true)
            .with_value("city", "Amsterdam")
            .with_value("joined_at", AnyValue::Date(1_704_412_800.0)),
    )?;
    assert_eq!(frame.row_count(), 5);

    frame.replace_row(
        0,
        &AnyRow::new()
            .with_value("id", 1_i64)
            .with_value("name", "Alan")
            .with_value("team", "compiler")
            .with_value("score", 97.0)
            .with_value("active", true)
            .with_value("city", "Manchester")
            .with_value("joined_at", AnyValue::Date(1_704_067_200.0)),
    )?;
    assert_eq!(
        frame.row(0)?.get("name"),
        Some(&AnyValue::String("Alan".to_string()))
    );
    Ok(())
}
