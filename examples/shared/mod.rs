#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use tabulardata::prelude::*;

pub fn fixture_frame() -> Result<DataFrame, TabularDataError> {
    DataFrame::from_rows(&[
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
    ])
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

pub fn output_path(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

pub fn csv_fixture(name: &str) -> PathBuf {
    let path = output_path(name);
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
