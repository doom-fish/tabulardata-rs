mod common;

use tabulardata::prelude::*;

#[test]
fn filter_specs_select_expected_rows() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let filter = Filter::and(vec![
        Filter::eq("team", "compiler"),
        Filter::contains("city", "on"),
    ]);
    let filtered = frame.filtered(&filter)?;
    assert_eq!(common::names(&filtered)?, vec!["Ada", "Barbara"]);
    Ok(())
}
