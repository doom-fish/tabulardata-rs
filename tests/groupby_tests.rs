mod common;

use tabulardata::prelude::*;

#[test]
fn group_by_counts_and_sums_work() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let grouped = frame.group_by(&["team"]);

    let counts = grouped.counts(None)?;
    assert_eq!(counts.row_count(), 2);

    let sums = grouped.sums("score", GroupValueType::Double, None)?;
    assert_eq!(sums.row_count(), 2);

    let compiler = grouped
        .group(&[AnyValue::from("compiler")])?
        .expect("compiler group");
    assert_eq!(compiler.row_count(), 3);
    Ok(())
}
