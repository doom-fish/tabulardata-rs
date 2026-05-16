mod common;

#[test]
fn any_column_snapshots_are_usable() -> Result<(), Box<dyn std::error::Error>> {
    let frame = common::fixture_frame()?;
    let teams = frame.any_column("team")?;
    assert_eq!(teams.type_name, "String");
    assert_eq!(teams.distinct().len(), 2);
    assert_eq!(teams.mask(&[true, false, true, false]).len(), 2);
    Ok(())
}
