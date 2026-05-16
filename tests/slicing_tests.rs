mod common;

#[test]
fn slicing_helpers_materialize_expected_frames() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;

    let middle = frame.slice_rows(1..3)?;
    assert_eq!(common::names(&middle)?, vec!["Grace", "Linus"]);

    let prefix = frame.prefix_rows(2)?;
    assert_eq!(common::ids(&prefix)?, vec![1, 2]);

    let selected = frame.select_columns(&["name", "score"])?;
    assert_eq!(selected.column_count(), 2);
    Ok(())
}
