mod common;

use tabulardata::prelude::*;

#[test]
fn dataframe_mutation_helpers_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = common::fixture_frame()?;

    frame.insert_column(
        1,
        &Column::strings(
            "level",
            vec![
                Some("staff".into()),
                Some("staff".into()),
                Some("principal".into()),
                Some("staff".into()),
            ],
        ),
    )?;
    assert_eq!(frame.column_names()?[1], "level");

    frame.replace_column(
        "score",
        &Column::doubles(
            "score",
            vec![Some(100.0), Some(92.0), Some(99.5), Some(88.0)],
        ),
    )?;
    frame.transform_non_null_column("score", |value| {
        AnyValue::Double(value.as_f64().unwrap() + 1.0)
    })?;
    assert!(matches!(
        frame.column_summary("score")?,
        ColumnSummary::Numeric(_)
    ));

    frame.combine_columns2("name", "team", "label", |name, team| {
        AnyValue::String(format!(
            "{}:{}",
            name.as_str().unwrap(),
            team.as_str().unwrap()
        ))
    })?;
    assert!(frame.contains_column("label")?);

    let removed = frame.remove_column("city")?;
    assert_eq!(removed.name, "city");
    assert!(!frame.contains_column("city")?);

    let mut appended = frame.slice_rows(0..0)?;
    appended.append_rows_of(&frame)?;
    assert_eq!(appended.row_count(), frame.row_count());

    let tiny = DataFrame::from_columns(&[
        Column::ints("id", vec![Some(1)]),
        Column::strings("name", vec![Some("Ada".into())]),
    ])?;
    let mut tiny = tiny;
    tiny.append_values(&[AnyValue::from(2_i64), AnyValue::from("Grace")])?;
    assert_eq!(tiny.row_count(), 2);

    let json = r#"[
      {"id":1,"name":"Ada","tags":["compiler","swift"]},
      {"id":2,"name":"Grace","tags":[]},
      {"id":3,"name":"Linus","tags":["kernel"]}
    ]"#;
    let mut exploded = DataFrame::from_json_string(json, JSONReadingOptions::new())?;
    exploded.explode_column("tags")?;
    assert_eq!(exploded.row_count(), 4);
    let tags: Vec<String> = exploded
        .rows()?
        .into_iter()
        .map(|row| match row.get("tags") {
            Some(AnyValue::String(value)) => Ok(value.clone()),
            Some(AnyValue::Null) | None => Ok(String::new()),
            other => Err(TabularDataError::FrameworkError(format!(
                "unexpected tag {other:?}"
            ))),
        })
        .collect::<Result<_, _>>()?;
    assert!(tags.contains(&"compiler".to_string()));
    assert!(tags.contains(&"swift".to_string()));
    assert!(tags.contains(&"kernel".to_string()));
    Ok(())
}
