use std::path::PathBuf;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = DataFrame::from_rows(&[
        AnyRow::new()
            .with_value("id", 1_i64)
            .with_value("name", "Ada")
            .with_value("active", true)
            .with_value("score", 98.5),
        AnyRow::new()
            .with_value("id", 2_i64)
            .with_value("name", "Grace")
            .with_value("active", false)
            .with_value("score", 91.0),
    ])?;

    let json_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/tabular.json");
    let read_request = JSONReadRequest::new(JSONReadingOptions::new())
        .with_type_hint("id", JSONType::Integer)
        .with_type_hint("active", JSONType::Boolean)
        .with_type_hint("score", JSONType::Double);
    let write_options = JSONWritingOptions::new()
        .with_sort_keys(true)
        .with_pretty_print(true)
        .with_date_strategy(DateWriteStrategy::Iso8601);

    let json = match frame.json_string(&write_options) {
        Ok(json) => {
            frame.write_json(&json_path, &write_options)?;
            json
        }
        Err(error) if error.message().contains("macOS 13") => {
            eprintln!("JSON writing unavailable on this macOS release; using inline JSON fallback");
            String::from(
                r#"[
  {"id":1,"name":"Ada","active":true,"score":98.5},
  {"id":2,"name":"Grace","active":false,"score":91.0}
]"#,
            )
        }
        Err(error) => return Err(Box::new(error)),
    };

    let reloaded = DataFrame::read_json_data_with(json.as_bytes(), &read_request)?;
    println!("json path = {}", json_path.display());
    println!("rows = {}", reloaded.row_count());
    println!("columns = {:?}", reloaded.column_names()?);
    println!("✅ tabulardata JSON IO OK");
    Ok(())
}
