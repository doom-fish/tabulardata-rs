mod common;

use tabulardata::prelude::*;

#[test]
fn sort_keys_order_rows_stably() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let sorted = frame.sorted_by(&[SortKey::ascending("team"), SortKey::descending("score")])?;
    assert_eq!(
        common::names(&sorted)?,
        vec!["Ada", "Grace", "Barbara", "Linus"]
    );
    Ok(())
}
