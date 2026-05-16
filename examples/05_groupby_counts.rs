#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    let grouped = frame.group_by(&["team"]);
    let counts = grouped.counts(None)?;
    println!("group counts rows = {}", counts.row_count());
    println!(
        "compiler rows = {}",
        grouped
            .group(&[AnyValue::from("compiler")])?
            .unwrap()
            .row_count()
    );
    Ok(())
}
