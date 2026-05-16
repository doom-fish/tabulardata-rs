mod common;

#[test]
fn dataframe_core_methods_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = common::fixture_frame()?;
    assert_eq!(frame.shape(), (4, 7));
    assert_eq!(frame.row_count(), 4);
    assert_eq!(frame.column_count(), 7);

    frame.rename_column("city", "location")?;
    assert!(frame.column_names()?.contains(&"location".to_string()));

    let names = frame.column("name")?;
    assert_eq!(names.len(), 4);
    assert_eq!(frame.rows_json()?.len(), 4);
    Ok(())
}
