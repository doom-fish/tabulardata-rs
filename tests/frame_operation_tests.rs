use tabulardata::prelude::*;

const NESTED_JSON: &str = r#"[
  {"meta":{"k":1},"tags":["a","b"]},
  {"meta":null,"tags":[]},
  {"meta":{},"tags":null}
]"#;

const SCHEMA: [(&str, &str); 10] = [
    ("meta", "Dictionary<String, Optional<Any>>"),
    ("tags", "Array<Optional<Any>>"),
    ("int", "Int"),
    ("int32", "Int32"),
    ("float", "Float"),
    ("double", "Double"),
    ("string", "String"),
    ("bool", "Bool"),
    ("date", "Date"),
    ("data", "Data"),
];

fn invalid(result: Result<impl std::fmt::Debug, TabularDataError>) -> String {
    match result {
        Err(TabularDataError::InvalidArgument(message)) => message,
        other => panic!("expected an invalid-argument error, got {other:?}"),
    }
}

fn typed_columns() -> Vec<Column> {
    vec![
        Column::ints("int", vec![Some(1), None, Some(3)]),
        Column::int32s("int32", vec![Some(-7), Some(i32::MAX), None]),
        Column::floats("float", vec![Some(0.1), None, Some(2.5)]),
        Column::doubles("double", vec![None, Some(2.25), Some(f64::INFINITY)]),
        Column::strings("string", vec![Some("x".into()), Some("y".into()), None]),
        Column::bools("bool", vec![Some(true), None, Some(false)]),
        Column::dates("date", vec![Some(1_704_067_200.0), None, Some(0.5)]),
        Column::binary("data", vec![Some("AQI=".into()), None, Some("/w==".into())]),
    ]
}

fn every_type_frame() -> Result<DataFrame, TabularDataError> {
    let mut frame = DataFrame::from_json_string(NESTED_JSON, JSONReadingOptions::new())?;
    for column in typed_columns() {
        frame.append_column(&column)?;
    }
    frame.add_alias("tag_list", "tags")?;
    frame.add_alias("small", "int32")?;
    frame.add_alias("weight", "float")?;
    Ok(frame)
}

fn schema(frame: &DataFrame) -> Result<Vec<(String, String)>, TabularDataError> {
    Ok(frame
        .any_columns()?
        .into_iter()
        .map(|column| (column.name, column.type_name))
        .collect())
}

fn expected_schema() -> Vec<(String, String)> {
    SCHEMA
        .iter()
        .map(|(name, type_name)| ((*name).to_owned(), (*type_name).to_owned()))
        .collect()
}

fn assert_alias(frame: &DataFrame, alias: &str, column: &str) -> Result<(), TabularDataError> {
    assert_eq!(
        frame.column_names_for_alias(alias)?,
        [column],
        "alias '{alias}'"
    );
    let position = frame.column_names()?.iter().position(|name| name == column);
    assert!(position.is_some(), "column '{column}' is missing");
    assert_eq!(frame.index_of_column(alias)?, position, "alias '{alias}'");
    Ok(())
}

fn assert_aliases(frame: &DataFrame) -> Result<(), TabularDataError> {
    assert_alias(frame, "tag_list", "tags")?;
    assert_alias(frame, "small", "int32")?;
    assert_alias(frame, "weight", "float")
}

#[test]
fn mask_rows_keeps_every_column_type_and_alias() -> Result<(), TabularDataError> {
    let frame = every_type_frame()?;
    assert_eq!(schema(&frame)?, expected_schema());
    let columns = frame.any_columns()?;

    let masked = frame.mask_rows(&[true, false, true])?;
    assert_eq!(schema(&masked)?, expected_schema());
    assert_aliases(&masked)?;
    for column in &columns {
        let expected = vec![column.values[0].clone(), column.values[2].clone()];
        assert_eq!(
            masked.any_column(&column.name)?.values,
            expected,
            "{}",
            column.name
        );
    }
    assert_eq!(
        masked.column("small")?,
        Column::int32s("int32", vec![Some(-7), None])
    );
    assert_eq!(
        masked.column("weight")?,
        Column::floats("float", vec![Some(0.1), Some(2.5)])
    );

    let none = frame.mask_rows(&[false; 3])?;
    assert_eq!(none.shape(), (0, SCHEMA.len()));
    assert_eq!(schema(&none)?, expected_schema());
    assert_aliases(&none)?;

    let filtered = frame.filtered_by_column("small", |value| !value.is_null())?;
    assert_eq!(filtered.row_count(), 2);
    assert_eq!(schema(&filtered)?, expected_schema());
    assert_aliases(&filtered)?;

    let message = invalid(frame.mask_rows(&[true]));
    assert!(message.contains("mask"), "{message}");
    invalid(frame.mask_rows(&[true; 4]));
    assert_eq!(DataFrame::new()?.mask_rows(&[])?.shape(), (0, 0));
    Ok(())
}

#[test]
fn insert_column_adds_one_column_and_leaves_the_rest_alone() -> Result<(), TabularDataError> {
    let mut frame = every_type_frame()?;
    let inserted = Column::int32s("inserted", vec![Some(1), Some(2), Some(3)]);
    frame.insert_column(1, &inserted)?;
    let last = Column::floats("last", vec![None, Some(-0.5), Some(f32::NEG_INFINITY)]);
    frame.insert_column(frame.column_count(), &last)?;

    let mut expected = expected_schema();
    expected.insert(1, ("inserted".into(), "Int32".into()));
    expected.push(("last".into(), "Float".into()));
    assert_eq!(schema(&frame)?, expected);
    assert_aliases(&frame)?;
    assert_eq!(frame.column("inserted")?, inserted);
    assert_eq!(frame.column("last")?, last);

    let extra = Column::ints("extra", vec![Some(1), Some(2), Some(3)]);
    let message = invalid(frame.insert_column(frame.column_count() + 1, &extra));
    assert!(message.contains("out of bounds"), "{message}");
    invalid(frame.insert_column(usize::MAX, &extra));
    invalid(frame.insert_column(0, &Column::ints("int", vec![Some(1), Some(2), Some(3)])));
    invalid(frame.insert_column(0, &Column::ints("small", vec![Some(1), Some(2), Some(3)])));
    let message = invalid(frame.insert_column(0, &Column::ints("short", vec![Some(1)])));
    assert!(message.contains("rows"), "{message}");
    assert_eq!(schema(&frame)?, expected);

    let mut empty = DataFrame::new()?;
    empty.insert_column(0, &Column::floats("f", vec![Some(1.0), None]))?;
    assert_eq!(empty.shape(), (2, 1));
    Ok(())
}

#[test]
fn replace_column_swaps_one_column_and_keeps_its_aliases() -> Result<(), TabularDataError> {
    let mut frame = every_type_frame()?;
    let replacement = Column::floats("int32", vec![Some(1.5), None, Some(-2.0)]);
    frame.replace_column("small", &replacement)?;
    let renamed = Column::doubles("mass", vec![Some(1.0), Some(2.0), None]);
    frame.replace_column("weight", &renamed)?;

    let mut expected = expected_schema();
    expected[3].1 = "Float".into();
    expected[4] = ("mass".into(), "Double".into());
    assert_eq!(schema(&frame)?, expected);
    assert_alias(&frame, "tag_list", "tags")?;
    assert_alias(&frame, "small", "int32")?;
    assert_alias(&frame, "weight", "mass")?;
    assert_eq!(frame.column("int32")?, replacement);
    assert_eq!(frame.column("weight")?, renamed);

    let three = |name: &str| Column::ints(name, vec![Some(1), Some(2), Some(3)]);
    invalid(frame.replace_column("nope", &three("nope")));
    invalid(frame.replace_column("int", &three("string")));
    invalid(frame.replace_column("int", &three("tag_list")));
    let message = invalid(frame.replace_column("int", &Column::ints("int", vec![Some(1)])));
    assert!(message.contains("rows"), "{message}");
    assert_eq!(schema(&frame)?, expected);

    frame.replace_column("int", &three("int"))?;
    assert_eq!(frame.column("int")?, three("int"));
    Ok(())
}

#[test]
fn remove_column_returns_columns_of_every_type() -> Result<(), TabularDataError> {
    let mut frame = every_type_frame()?;
    let columns = frame.any_columns()?;

    let removed = frame.remove_column("small")?;
    assert_eq!(removed.type_name, "Int32");
    assert_eq!(&removed, &columns[3]);
    assert!(frame.column_names_for_alias("small")?.is_empty());
    assert_eq!(frame.index_of_column("small")?, None);
    assert_alias(&frame, "tag_list", "tags")?;
    assert_alias(&frame, "weight", "float")?;

    let removed = frame.remove_column("meta")?;
    assert_eq!(&removed, &columns[0]);
    assert_alias(&frame, "tag_list", "tags")?;
    assert_alias(&frame, "weight", "float")?;

    for column in columns
        .iter()
        .filter(|column| column.name != "int32" && column.name != "meta")
    {
        assert_eq!(
            &frame.remove_column(&column.name)?,
            column,
            "{}",
            column.name
        );
    }
    assert_eq!(frame.shape(), (0, 0));

    let mut frame = every_type_frame()?;
    let message = invalid(frame.remove_column("nope"));
    assert!(message.contains("'nope'"), "{message}");
    assert_eq!(schema(&frame)?, expected_schema());
    assert_eq!(
        frame.remove_column("weight")?.to_column()?,
        typed_columns()[2]
    );
    Ok(())
}

#[test]
#[allow(clippy::too_many_lines)]
fn transform_column_keeps_each_element_type() -> Result<(), TabularDataError> {
    let mut frame = every_type_frame()?;
    frame.transform_column("int", |value| {
        value
            .as_i64()
            .map_or(AnyValue::Null, |n| AnyValue::Int(n * 10))
    })?;
    frame.transform_column("small", |value| {
        AnyValue::Int(value.as_i64().map_or(0, |n| n / 7))
    })?;
    frame.transform_non_null_column("weight", |value| {
        AnyValue::Double(value.as_f64().unwrap_or_default() * 2.0)
    })?;
    frame.transform_column("double", |value| {
        value
            .as_f64()
            .map_or(AnyValue::Int(4), |x| AnyValue::Double(-x))
    })?;
    frame.transform_column("string", |value| {
        value.as_str().map_or_else(
            || AnyValue::from("none"),
            |text| AnyValue::from(text.to_uppercase()),
        )
    })?;
    frame.transform_column("bool", |value| {
        value
            .as_bool()
            .map_or(AnyValue::Null, |flag| AnyValue::Bool(!flag))
    })?;
    frame.transform_column("date", |value| {
        value
            .as_f64()
            .map_or(AnyValue::Null, |seconds| AnyValue::Date(seconds + 60.0))
    })?;
    frame.transform_column("data", |value| {
        if value.is_null() {
            AnyValue::Data("AA==".into())
        } else {
            value.clone()
        }
    })?;
    frame.transform_column("tag_list", |value| match value {
        AnyValue::Array(items) => AnyValue::Array(items.iter().rev().cloned().collect()),
        _ => AnyValue::Array(vec![AnyValue::from("new")]),
    })?;
    frame.transform_column("meta", |value| match value {
        AnyValue::Object(map) => AnyValue::Object(
            map.iter()
                .map(|(key, value)| (key.to_uppercase(), value.clone()))
                .collect(),
        ),
        _ => AnyValue::Null,
    })?;

    assert_eq!(schema(&frame)?, expected_schema());
    assert_aliases(&frame)?;
    assert_eq!(
        frame.column("int")?,
        Column::ints("int", vec![Some(10), None, Some(30)])
    );
    assert_eq!(
        frame.column("int32")?,
        Column::int32s("int32", vec![Some(-1), Some(i32::MAX / 7), Some(0)])
    );
    assert_eq!(
        frame.column("float")?,
        Column::floats("float", vec![Some(0.2), None, Some(5.0)])
    );
    assert_eq!(
        frame.column("double")?,
        Column::doubles(
            "double",
            vec![Some(4.0), Some(-2.25), Some(f64::NEG_INFINITY)]
        )
    );
    assert_eq!(
        frame.column("string")?,
        Column::strings(
            "string",
            vec![Some("X".into()), Some("Y".into()), Some("none".into())]
        )
    );
    assert_eq!(
        frame.column("bool")?,
        Column::bools("bool", vec![Some(false), None, Some(true)])
    );
    assert_eq!(
        frame.column("date")?,
        Column::dates("date", vec![Some(1_704_067_260.0), None, Some(60.5)])
    );
    assert_eq!(
        frame.column("data")?,
        Column::binary(
            "data",
            vec![
                Some("AQI=".into()),
                Some("AA==".into()),
                Some("/w==".into())
            ]
        )
    );
    assert_eq!(
        frame.any_column("tags")?.values,
        [
            AnyValue::Array(vec![AnyValue::from("b"), AnyValue::from("a")]),
            AnyValue::Array(Vec::new()),
            AnyValue::Array(vec![AnyValue::from("new")]),
        ]
    );
    let meta = frame.any_column("meta")?.values;
    assert_eq!(
        meta[0],
        AnyValue::Object([("K".to_owned(), AnyValue::Int(1))].into())
    );
    assert_eq!(meta[1], AnyValue::Null);

    let before = frame.any_columns()?;
    let message = invalid(frame.transform_column("small", |_| AnyValue::Int(i64::MAX)));
    assert!(message.contains("'int32'"), "{message}");
    invalid(frame.transform_column("int", |_| AnyValue::from("x")));
    invalid(frame.transform_column("bool", |_| AnyValue::Int(1)));
    invalid(frame.transform_column("data", |_| AnyValue::from("not base64")));
    invalid(frame.transform_column("tags", |_| AnyValue::from("x")));
    assert!(frame.transform_column("nope", Clone::clone).is_err());
    assert_eq!(frame.any_columns()?, before);
    Ok(())
}

#[test]
fn explode_keeps_other_column_types_and_aliases() -> Result<(), TabularDataError> {
    let json = r#"[
      {"id":1,"tags":["a","b"]},
      {"id":2,"tags":[]},
      {"id":3,"tags":null},
      {"id":4,"tags":["c",null]}
    ]"#;
    let mut frame = DataFrame::from_json_string(json, JSONReadingOptions::new())?;
    frame.append_column(&Column::int32s(
        "rank",
        vec![Some(10), Some(20), Some(30), Some(40)],
    ))?;
    frame.append_column(&Column::floats(
        "weight",
        vec![Some(0.5), None, Some(1.5), Some(2.5)],
    ))?;
    frame.add_alias("key", "id")?;
    frame.add_alias("labels", "tags")?;

    let exploded = frame.exploding_column("labels")?;
    assert_eq!(
        schema(&exploded)?,
        [
            ("id".to_owned(), "Int".to_owned()),
            ("tags".to_owned(), "String".to_owned()),
            ("rank".to_owned(), "Int32".to_owned()),
            ("weight".to_owned(), "Float".to_owned()),
        ]
    );
    assert_alias(&exploded, "key", "id")?;
    assert_alias(&exploded, "labels", "tags")?;
    assert_eq!(
        exploded.column("id")?,
        Column::ints("id", vec![Some(1), Some(1), Some(4), Some(4)])
    );
    assert_eq!(
        exploded.column("tags")?,
        Column::strings(
            "tags",
            vec![Some("a".into()), Some("b".into()), Some("c".into()), None]
        )
    );
    assert_eq!(
        exploded.column("rank")?,
        Column::int32s("rank", vec![Some(10), Some(10), Some(40), Some(40)])
    );
    assert_eq!(
        exploded.column("weight")?,
        Column::floats("weight", vec![Some(0.5), Some(0.5), Some(2.5), Some(2.5)])
    );
    assert_eq!(frame.row_count(), 4);

    let mut in_place = frame.try_clone()?;
    in_place.explode_column("tags")?;
    assert_eq!(in_place.any_columns()?, exploded.any_columns()?);
    assert_alias(&in_place, "labels", "tags")?;

    let rebuilt = DataFrame::from_rows(&frame.rows()?)?;
    assert!(rebuilt.contains_column_of_type("tags", "Array<Optional<Any>>")?);

    let numbers =
        DataFrame::from_json_string(r#"[{"v":[1,2.5]},{"v":[3]}]"#, JSONReadingOptions::new())?;
    assert_eq!(
        numbers.exploding_column("v")?.column("v")?,
        Column::doubles("v", vec![Some(1.0), Some(2.5), Some(3.0)])
    );
    let nested = DataFrame::from_json_string(r#"[{"v":[[1],[2,3]]}]"#, JSONReadingOptions::new())?;
    assert_eq!(
        schema(&nested.exploding_column("v")?)?,
        [("v".to_owned(), "Array<Optional<Any>>".to_owned())]
    );
    let empty = DataFrame::from_json_string(r#"[{"v":[]}]"#, JSONReadingOptions::new())?;
    assert_eq!(empty.exploding_column("v")?.shape(), (0, 1));

    let mixed = DataFrame::from_json_string(r#"[{"v":["a",1]}]"#, JSONReadingOptions::new())?;
    let message = invalid(mixed.exploding_column("v"));
    assert!(message.contains("mixes"), "{message}");
    let message = invalid(frame.exploding_column("key"));
    assert!(message.contains("array"), "{message}");
    invalid(every_type_frame()?.exploding_column("meta"));
    invalid(frame.exploding_column("nope"));
    Ok(())
}

#[test]
fn sorting_splitting_and_cloning_keep_aliases() -> Result<(), TabularDataError> {
    let mut frame = every_type_frame()?;
    let sorted = frame.sorted_by(&[SortKey::descending("small")])?;
    assert_eq!(schema(&sorted)?, expected_schema());
    assert_aliases(&sorted)?;
    assert_eq!(
        sorted.column("int32")?,
        Column::int32s("int32", vec![Some(i32::MAX), Some(-7), None])
    );

    frame.sort_by(&[SortKey::ascending("weight")])?;
    assert_aliases(&frame)?;
    assert_eq!(
        frame.column("float")?,
        Column::floats("float", vec![None, Some(0.1), Some(2.5)])
    );

    for proportion in [0.0, 1.0] {
        let (left, right) = frame.random_split(proportion, Some(1))?;
        assert_aliases(&left)?;
        assert_aliases(&right)?;
        assert_eq!(schema(&left)?, expected_schema());
        assert_eq!(schema(&right)?, expected_schema());
    }
    assert_aliases(&frame.try_clone()?)?;
    Ok(())
}

#[test]
fn null_elements_inside_arrays_and_objects_stay_null() -> Result<(), TabularDataError> {
    let json = r#"[{"t":["c",null],"o":{"j":1,"k":null}}]"#;
    let mut frame = DataFrame::from_json_string(json, JSONReadingOptions::new())?;
    let array = AnyValue::Array(vec![AnyValue::from("c"), AnyValue::Null]);
    let object = AnyValue::Object(
        [
            ("j".to_owned(), AnyValue::Int(1)),
            ("k".to_owned(), AnyValue::Null),
        ]
        .into(),
    );
    assert_eq!(frame.any_column("t")?.values, std::slice::from_ref(&array));
    assert_eq!(frame.any_column("o")?.values, std::slice::from_ref(&object));
    assert_eq!(frame.row(0)?.get("t"), Some(&array));
    assert_eq!(
        frame.rows_json()?,
        [serde_json::json!({"o": {"j": 1, "k": null}, "t": ["c", null]})]
    );

    frame.transform_column("t", Clone::clone)?;
    frame.transform_column("o", Clone::clone)?;
    let rebuilt = DataFrame::from_rows(&frame.rows()?)?;
    for frame in [&frame, &rebuilt] {
        assert_eq!(frame.any_column("t")?.values, std::slice::from_ref(&array));
        assert_eq!(frame.any_column("o")?.values, std::slice::from_ref(&object));
    }
    Ok(())
}
