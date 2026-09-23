mod common;

use tabulardata::prelude::*;

fn invalid(result: Result<impl std::fmt::Debug, TabularDataError>) -> String {
    match result {
        Err(TabularDataError::InvalidArgument(message)) => message,
        other => panic!("expected an invalid-argument error, got {other:?}"),
    }
}

#[test]
fn column_name_typos_are_errors_in_every_operation() -> Result<(), TabularDataError> {
    let frame = common::fixture_frame()?;
    let other = common::department_frame()?;

    let message = invalid(frame.filtered(&Filter::eq("tema", "compiler")));
    assert!(message.contains("'tema'"), "{message}");
    invalid(frame.filtered(&Filter::and(vec![
        Filter::eq("team", "compiler"),
        Filter::negate(Filter::is_null("citty")),
    ])));
    invalid(frame.slice_rows(0..0)?.filtered(&Filter::gt("scroe", 1.0)));
    let not_london = frame.filtered(&Filter::negate(Filter::eq("city", "London")))?;
    assert_eq!(common::names(&not_london)?, ["Grace", "Linus"]);

    invalid(frame.sorted_by(&[SortKey::ascending("name"), SortKey::descending("scor")]));
    invalid(frame.select_columns(&["name", "nmae"]));
    invalid(frame.select_columns(&["name", "name"]));
    invalid(frame.summary_columns(&["nope"]));

    invalid(frame.joined(&other, "idd", JoinKind::Inner));
    invalid(frame.joined_on(&other, JoinColumns::new("id", "employe_id"), JoinKind::Left));
    let message = invalid(frame.joined_on(&other, JoinColumns::new("name", "employee_id"), JoinKind::Inner));
    assert!(message.contains("same type"), "{message}");

    invalid(frame.group_by(&["tema"]).counts(None));
    invalid(frame.group_by(&["team", "team"]).counts(None));
    invalid(frame.group_by(&["team"]).sums("scroe", GroupValueType::Double, None));
    invalid(frame.group_by(&["team"]).means("scroe", GroupValueType::Double, None));
    invalid(frame.group_by(&["team"]).group(&[AnyValue::from("compiler"), AnyValue::from("x")]));
    invalid(frame.group_by_time("joined_att", TimeUnit::Day).counts(None));
    invalid(frame.stratified_split(&["tema"], 0.5, Some(1)));

    let mut encoded = frame.try_clone()?;
    invalid(encoded.encode_column("nmae", ColumnElementType::String, ColumnCodec::Json));
    invalid(encoded.decode_column("name", ColumnElementType::String, ColumnCodec::Json));
    invalid(encoded.encode_column("id", ColumnElementType::String, ColumnCodec::Json));

    assert_eq!(frame.row_count(), 4);
    assert_eq!(common::names(&frame.sorted_by(&[SortKey::ascending("name")])?)?, [
        "Ada", "Barbara", "Grace", "Linus"
    ]);
    Ok(())
}

#[test]
fn appended_rows_are_checked_against_column_types() -> Result<(), TabularDataError> {
    let mut frame = DataFrame::from_columns(&[
        Column::ints("id", vec![Some(1)]),
        Column::strings("name", vec![Some("Ada".into())]),
    ])?;

    let message = invalid(frame.append_row(&AnyRow::new().with_value("id", 1.5)));
    assert!(message.contains("'id'"), "{message}");
    invalid(frame.append_row(&AnyRow::new().with_value("id", "two")));
    invalid(frame.append_row(&AnyRow::new().with_value("nmae", "Grace")));
    invalid(frame.insert_row(0, &AnyRow::new().with_value("name", 7_i64)));
    invalid(frame.replace_row(0, &AnyRow::new().with_value("id", true)));
    invalid(frame.append_values(&[AnyValue::from("x"), AnyValue::from("y")]));
    assert_eq!(frame.row_count(), 1);

    frame.append_row(&AnyRow::new().with_value("id", 2.0))?;
    frame.append_row(&AnyRow::new().with_value("name", "Linus"))?;
    frame.insert_row(0, &AnyRow::new().with_value("id", 0_i64).with_value("name", "Zero"))?;
    frame.replace_row(1, &AnyRow::new().with_value("id", 10_i64).with_value("name", "Ada Lovelace"))?;
    assert_eq!(frame.row_count(), 4);
    let ids: Vec<AnyValue> = frame.rows()?.iter().map(|row| row.get("id").cloned().unwrap_or_default()).collect();
    assert_eq!(ids, [AnyValue::Int(0), AnyValue::Int(10), AnyValue::Int(2), AnyValue::Null]);
    assert_eq!(frame.row(1)?.get("name"), Some(&AnyValue::from("Ada Lovelace")));

    let mut empty = DataFrame::new()?;
    invalid(empty.append_row(&AnyRow::new().with_value("x", 1_i64)));
    Ok(())
}

#[test]
fn from_rows_widens_mixed_numbers_and_rejects_mixed_kinds() -> Result<(), TabularDataError> {
    let frame = DataFrame::from_rows(&[
        AnyRow::new().with_value("x", 1_i64),
        AnyRow::new().with_value("x", 1.5),
        AnyRow::new(),
    ])?;
    assert!(frame.contains_column_of_type("x", "Double")?);
    assert_eq!(frame.column("x")?.values(), [
        AnyValue::Double(1.0),
        AnyValue::Double(1.5),
        AnyValue::Null
    ]);

    let message = invalid(DataFrame::from_rows(&[
        AnyRow::new().with_value("x", 1_i64),
        AnyRow::new().with_value("x", "one"),
    ]));
    assert!(message.contains("mixes"), "{message}");
    assert_eq!(DataFrame::from_rows(&[AnyRow::new()])?.shape(), (0, 0));
    Ok(())
}

#[test]
fn append_rows_of_keeps_float_and_int_columns_intact() -> Result<(), TabularDataError> {
    let csv = b"id,weight\n1,1.25\n2,2.5\n";
    let options = CSVReadingOptions::new().with_floating_point_type(CSVType::Float);
    let floats = DataFrame::from_csv_data(csv, options)?;
    assert!(floats.contains_column_of_type("weight", "Float")?);

    let mut copy = floats.slice_rows(0..0)?;
    copy.append_rows_of(&floats)?;
    copy.append_rows_of(&floats)?;
    assert_eq!(copy.row_count(), 4);
    assert!(copy.contains_column_of_type("weight", "Float")?);

    let mut through_rows = floats.slice_rows(0..0)?;
    for row in floats.rows()? {
        through_rows.append_row(&row)?;
    }
    assert_eq!(
        through_rows.any_column("weight")?.values,
        floats.any_column("weight")?.values
    );
    assert!(through_rows.contains_column_of_type("weight", "Float")?);

    let mut doubles = DataFrame::from_columns(&[
        Column::ints("id", vec![Some(9)]),
        Column::doubles("weight", vec![Some(0.5)]),
    ])?;
    doubles.append_rows_of(&floats)?;
    assert_eq!(doubles.row_count(), 3);
    assert!(doubles.contains_column_of_type("weight", "Double")?);

    let mut ints = DataFrame::from_columns(&[
        Column::ints("id", vec![Some(1)]),
        Column::ints("weight", vec![Some(3)]),
    ])?;
    invalid(ints.append_rows_of(&floats));
    assert_eq!(ints.row_count(), 1);
    let renamed = DataFrame::from_columns(&[Column::ints("identifier", vec![Some(1)])])?;
    invalid(ints.append_rows_of(&renamed));
    Ok(())
}

#[test]
fn group_aggregates_follow_the_column_type() -> Result<(), TabularDataError> {
    let frame = DataFrame::from_rows(&[
        AnyRow::new().with_value("team", "a").with_value("age", 30_i64).with_value("score", 1.5),
        AnyRow::new().with_value("team", "a").with_value("age", 40_i64).with_value("score", 2.5),
        AnyRow::new().with_value("team", "b").with_value("age", 50_i64).with_value("score", 4.0),
    ])?;
    let groups = frame.group_by(&["team"]);

    let means = groups.means("age", GroupValueType::Int, Some(SortOrder::Ascending))?;
    assert_eq!(means.row_count(), 2);
    let mean_values: Vec<f64> = means
        .rows()?
        .iter()
        .filter_map(|row| row.values.iter().find(|(name, _)| name.contains("age")).and_then(|(_, value)| value.as_f64()))
        .collect();
    assert!(mean_values.contains(&35.0) && mean_values.contains(&50.0), "{mean_values:?}");

    groups.quantiles("age", 0.5, None)?;
    groups.means("score", GroupValueType::Double, None)?;
    groups.sums("score", GroupValueType::Int, None)?;
    groups.sums("age", GroupValueType::Double, None)?;
    groups.minimums("team", GroupValueType::String, None)?;
    invalid(groups.minimums("team", GroupValueType::Int, None));
    invalid(groups.sums("team", GroupValueType::String, None));
    invalid(groups.means("team", GroupValueType::String, None));
    invalid(groups.quantiles("age", 1.5, None));

    let message = invalid(frame.group_by_time("age", TimeUnit::Day).counts(None));
    assert!(message.contains("Date"), "{message}");

    let by_age = frame.group_by(&["age"]);
    let group = by_age.group(&[AnyValue::Double(40.0)])?.expect("40.0 should find the Int key 40");
    assert_eq!(group.row_count(), 1);
    invalid(by_age.group(&[AnyValue::Double(40.5)]));
    Ok(())
}

#[test]
fn append_and_rename_column_reject_bad_shapes_and_collisions() -> Result<(), TabularDataError> {
    let mut frame = DataFrame::from_columns(&[Column::ints("id", vec![Some(1), Some(2)])])?;
    let message = invalid(frame.append_column(&Column::ints("short", vec![Some(1)])));
    assert!(message.contains("rows"), "{message}");
    invalid(frame.append_column(&Column::strings("id", vec![None, None])));
    frame.append_column(&Column::strings("name", vec![Some("a".into()), None]))?;
    assert_eq!(frame.shape(), (2, 2));

    invalid(frame.rename_column("id", "name"));
    invalid(frame.rename_column("missing", "other"));
    frame.rename_column("id", "id")?;
    frame.rename_column("id", "key")?;
    assert_eq!(frame.column_names()?, ["key", "name"]);

    invalid(frame.insert_column(0, &Column::ints("extra", vec![Some(1)])));
    invalid(frame.replace_column("name", &Column::ints("key", vec![Some(1), Some(2)])));
    assert_eq!(frame.column_names()?, ["key", "name"]);

    let mut rowless = DataFrame::from_columns(&[Column::ints("id", vec![])])?;
    invalid(rowless.append_column(&Column::ints("other", vec![Some(1)])));
    let mut empty = DataFrame::new()?;
    empty.append_column(&Column::ints("id", vec![Some(1), Some(2), Some(3)]))?;
    assert_eq!(empty.shape(), (3, 1));
    Ok(())
}

#[test]
fn json_bytes_are_raw_bytes_and_inputs_accept_any_utf8() -> Result<(), TabularDataError> {
    let frame = common::fixture_frame()?.select_columns(&["id", "name"])?;
    let options = JSONWritingOptions::new().with_sort_keys(true);
    let bytes = frame.json_bytes(&options)?;
    assert_eq!(bytes.first(), Some(&b'['));
    assert_eq!(String::from_utf8(bytes.clone()).ok(), Some(frame.json_string(&options)?));
    let round_trip = DataFrame::from_json_data(&bytes, JSONReadingOptions::new())?;
    assert_eq!(round_trip.shape(), (4, 2));

    let with_nul = DataFrame::from_csv_data(b"a,b\n1,x\0y\n2,z\n", CSVReadingOptions::new());
    if let Ok(parsed) = with_nul {
        assert_eq!(parsed.row_count(), 2);
    }
    invalid(DataFrame::from_csv_data(b"a\n\xFF\n", CSVReadingOptions::new()));
    Ok(())
}

#[test]
fn non_utf8_paths_are_rejected() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    let path = Path::new(OsStr::from_bytes(b"target/not-\xFF-utf8.csv"));
    invalid(DataFrame::from_csv(path, CSVReadingOptions::new()));
    let frame = common::fixture_frame().unwrap();
    invalid(frame.write_csv(path, &CSVWritingOptions::new()));
}

#[test]
fn aliases_resolve_for_reads_but_not_as_alias_targets_or_row_keys() -> Result<(), TabularDataError> {
    let mut frame = DataFrame::from_columns(&[Column::ints("id", vec![Some(2), Some(1)])])?;
    frame.add_alias("key", "id")?;
    invalid(frame.add_alias("other", "key"));
    assert_eq!(frame.sorted_by(&[SortKey::ascending("key")])?.column("id")?.values(), [
        AnyValue::Int(1),
        AnyValue::Int(2)
    ]);
    invalid(frame.append_row(&AnyRow::new().with_value("key", 3_i64)));
    assert_eq!(frame.row_count(), 2);
    Ok(())
}

#[test]
fn edge_split_proportions_non_ascii_csv_options_and_int_sum_overflow_are_errors_or_handled(
) -> Result<(), TabularDataError> {
    let frame = common::fixture_frame()?;
    let (left, right) = frame.random_split(0.0, Some(1))?;
    assert_eq!((left.row_count(), right.row_count()), (0, 4));
    let (left, right) = frame.random_split(1.0, None)?;
    assert_eq!((left.row_count(), right.row_count()), (4, 0));
    let (left, right) = frame.stratified_split(&["team"], 0.0, Some(1))?;
    assert_eq!((left.row_count(), right.row_count()), (0, 4));
    let (left, right) = frame.stratified_split(&["team"], 1.0, Some(1))?;
    assert_eq!((left.row_count(), right.row_count()), (4, 0));
    assert_eq!(right.column_names()?, frame.column_names()?);
    invalid(frame.random_split(1.5, None));
    invalid(frame.stratified_split::<&str>(&[], 0.5, None));

    invalid(DataFrame::from_csv_data(
        "a§b\n1§2\n".as_bytes(),
        CSVReadingOptions::new().with_delimiter('§'),
    ));
    invalid(DataFrame::from_csv_data(
        b"a,b\n1,2\n",
        CSVReadingOptions::new().with_escape_character('€'),
    ));

    let big = DataFrame::from_columns(&[
        Column::strings("k", vec![Some("x".into()), Some("x".into())]),
        Column::ints("v", vec![Some(i64::MAX), Some(1)]),
    ])?;
    let message = invalid(big.group_by(&["k"]).sums("v", GroupValueType::Int, None));
    assert!(message.contains("overflow"), "{message}");
    big.group_by(&["k"]).means("v", GroupValueType::Int, None)?;
    let fine = DataFrame::from_columns(&[
        Column::strings("k", vec![Some("x".into()), Some("y".into())]),
        Column::ints("v", vec![Some(i64::MAX), Some(i64::MIN)]),
    ])?;
    fine.group_by(&["k"]).sums("v", GroupValueType::Int, None)?;
    Ok(())
}
