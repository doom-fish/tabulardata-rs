mod common;

use tabulardata::prelude::*;

#[test]
fn group_summaries_and_group_splits_work() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let grouped = frame.group_by(&["team"]);

    assert_eq!(grouped.group_count()?, 2);
    assert_eq!(grouped.ungrouped()?.shape(), frame.shape());

    let filtered = grouped.filter_groups(|group| Ok(group.row_count() >= 3))?;
    assert_eq!(filtered.row_count(), 3);

    let mapped = grouped.map_groups(|group| group.prefix_rows(1))?;
    assert_eq!(mapped.row_count(), 2);

    let (left, right) = grouped.random_split(0.5, Some(42))?;
    assert_eq!(left.row_count() + right.row_count(), frame.row_count());

    let summaries = grouped.summary()?;
    assert_eq!(summaries.len(), 2);
    let compiler = summaries
        .group(&[AnyValue::from("compiler")])
        .expect("compiler group summary");
    assert!(compiler.row_count() >= 1);
    assert!(summaries.description().contains("columns=[\"team\"]"));
    let rendered = summaries.format(&FormattingOptions::new().with_maximum_row_count(2))?;
    assert!(rendered.contains("compiler"));

    let score_summaries = grouped.summary_of(&["score"])?;
    assert_eq!(score_summaries.len(), 2);
    Ok(())
}
