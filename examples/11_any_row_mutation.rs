#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = support::fixture_frame()?;
    frame.append_row(
        &AnyRow::new()
            .with_value("id", 5_i64)
            .with_value("name", "Margaret")
            .with_value("team", "compiler")
            .with_value("score", 95.0)
            .with_value("active", true)
            .with_value("city", "Houston")
            .with_value("joined_at", AnyValue::Date(1_704_412_800.0)),
    )?;
    println!("last row = {:?}", frame.row(frame.row_count() - 1)?);
    Ok(())
}
