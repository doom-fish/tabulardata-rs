mod common;

use tabulardata::prelude::*;

#[test]
fn dataframe_core_methods_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = common::fixture_frame()?;
    assert_eq!(frame.shape(), (4, 7));
    assert_eq!(frame.row_count(), 4);
    assert_eq!(frame.column_count(), 7);
    assert!(frame.contains_column("score")?);
    assert!(frame.contains_column_of_type("score", "Double")?);
    assert!(frame.index_of_column("city")?.is_some());

    frame.add_alias("location", "city")?;
    assert_eq!(
        frame.column_names_for_alias("location")?,
        vec!["city".to_string()]
    );
    frame.remove_alias("location")?;
    assert!(frame.column_names_for_alias("location")?.is_empty());

    frame.rename_column("city", "location")?;
    assert!(frame.column_names()?.contains(&"location".to_string()));

    let names = frame.column("name")?;
    assert_eq!(names.len(), 4);
    assert_eq!(frame.rows_json()?.len(), 4);

    frame.append_empty_row()?;
    assert_eq!(frame.row_count(), 5);
    frame.remove_row(4)?;
    assert_eq!(frame.row_count(), 4);

    let description = frame.description()?;
    assert!(description.contains("Ada"));
    let formatted = frame.format(
        &FormattingOptions::new()
            .with_maximum_row_count(2)
            .with_includes_column_types(false),
    )?;
    assert!(!formatted.is_empty());

    let (left, right) = frame.random_split(0.5, Some(7))?;
    assert_eq!(left.row_count() + right.row_count(), frame.row_count());

    let (train, test) = frame.stratified_split(&["team"], 0.5, Some(11))?;
    assert_eq!(train.row_count() + test.row_count(), frame.row_count());
    assert_eq!(train.column_names()?, frame.column_names()?);
    Ok(())
}
