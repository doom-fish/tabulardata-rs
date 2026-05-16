#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use tabulardata::prelude::*;

pub fn fixture_frame() -> Result<DataFrame, TabularDataError> {
    DataFrame::from_rows(&fixture_rows())
}

pub fn fixture_rows() -> Vec<AnyRow> {
    vec![
        AnyRow::new()
            .with_value("id", 1_i64)
            .with_value("name", "Ada")
            .with_value("team", "compiler")
            .with_value("score", 98.5)
            .with_value("active", true)
            .with_value("city", "London")
            .with_value("joined_at", AnyValue::Date(1_704_067_200.0)),
        AnyRow::new()
            .with_value("id", 2_i64)
            .with_value("name", "Grace")
            .with_value("team", "compiler")
            .with_value("score", 91.0)
            .with_value("active", false)
            .with_value("city", "New York")
            .with_value("joined_at", AnyValue::Date(1_704_153_600.0)),
        AnyRow::new()
            .with_value("id", 3_i64)
            .with_value("name", "Linus")
            .with_value("team", "kernel")
            .with_value("score", 99.0)
            .with_value("active", true)
            .with_value("city", "Helsinki")
            .with_value("joined_at", AnyValue::Date(1_704_240_000.0)),
        AnyRow::new()
            .with_value("id", 4_i64)
            .with_value("name", "Barbara")
            .with_value("team", "compiler")
            .with_value("score", 87.5)
            .with_value("active", true)
            .with_value("city", "London")
            .with_value("joined_at", AnyValue::Date(1_704_326_400.0)),
    ]
}

pub fn department_frame() -> Result<DataFrame, TabularDataError> {
    DataFrame::from_rows(&[
        AnyRow::new()
            .with_value("employee_id", 1_i64)
            .with_value("office", "Cambridge"),
        AnyRow::new()
            .with_value("employee_id", 2_i64)
            .with_value("office", "Arlington"),
        AnyRow::new()
            .with_value("employee_id", 4_i64)
            .with_value("office", "London"),
    ])
}

pub fn test_output_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-output");
    fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn csv_fixture_path(name: &str) -> PathBuf {
    let path = test_output_dir().join(name);
    let contents = [
        "id,name,score,active,joined_at",
        "1,Ada,98.5,true,2024-01-01",
        "2,Grace,91.0,false,2024-01-02",
        "3,Linus,99.0,true,2024-01-03",
        "4,Barbara,87.5,true,2024-01-04",
    ]
    .join("\n");
    fs::write(&path, contents).unwrap();
    path
}

pub fn ids(frame: &DataFrame) -> Result<Vec<i64>, TabularDataError> {
    frame
        .rows()?
        .into_iter()
        .map(|row| match row.get("id") {
            Some(AnyValue::Int(value)) => Ok(*value),
            other => Err(TabularDataError::FrameworkError(format!(
                "missing integer id: {other:?}"
            ))),
        })
        .collect()
}

pub fn names(frame: &DataFrame) -> Result<Vec<String>, TabularDataError> {
    frame
        .rows()?
        .into_iter()
        .map(|row| match row.get("name") {
            Some(AnyValue::String(value)) => Ok(value.clone()),
            other => Err(TabularDataError::FrameworkError(format!(
                "missing string name: {other:?}"
            ))),
        })
        .collect()
}
