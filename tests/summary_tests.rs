mod common;

use tabulardata::prelude::*;

#[test]
fn summaries_cover_numeric_and_categorical_columns() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;

    match frame.column_summary("score")? {
        ColumnSummary::Numeric(summary) => {
            assert_eq!(summary.some_count, 4);
            assert_eq!(summary.min, Some(87.5));
            assert_eq!(summary.max, Some(99.0));
        }
        other @ ColumnSummary::Categorical(_) => {
            panic!("expected numeric summary, got {other:?}")
        }
    }

    match frame.column_summary("team")? {
        ColumnSummary::Categorical(summary) => {
            assert_eq!(summary.unique_count, 2);
            assert_eq!(summary.some_count, 4);
        }
        other @ ColumnSummary::Numeric(_) => {
            panic!("expected categorical summary, got {other:?}")
        }
    }

    let summary = frame.summary_columns(&["score", "team"])?;
    assert_eq!(summary.row_count(), 2);
    Ok(())
}
