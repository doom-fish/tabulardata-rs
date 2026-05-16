mod common;

use tabulardata::prelude::*;

#[test]
fn column_slices_capture_ranges_and_masks() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let scores = frame.column_slice("score", 1..3)?;
    assert_eq!(scores.indices, vec![1, 2]);
    assert_eq!(scores.len(), 2);

    match scores.summary() {
        ColumnSummary::Numeric(summary) => {
            assert_eq!(summary.some_count, 2);
            assert_eq!(summary.min, Some(91.0));
            assert_eq!(summary.max, Some(99.0));
        }
        other @ ColumnSummary::Categorical(_) => {
            panic!("expected numeric summary, got {other:?}")
        }
    }

    let londoners = frame.column_mask("city", &[true, false, false, true])?;
    assert_eq!(londoners.indices, vec![0, 3]);
    assert_eq!(londoners.distinct().len(), 1);
    Ok(())
}
