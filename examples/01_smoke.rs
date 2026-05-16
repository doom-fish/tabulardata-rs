use std::path::PathBuf;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/tabular.csv");

    let left = DataFrame::from_columns(&[
        Column::strings(
            "name",
            vec![
                Some("Ada".to_string()),
                Some("Grace".to_string()),
                Some("Linus".to_string()),
            ],
        ),
        Column::doubles("score", vec![Some(98.5), Some(88.0), Some(91.25)]),
    ])?;
    left.write_csv(&csv_path, CSVWritingOptions::default())?;

    let reloaded = DataFrame::from_csv(&csv_path, CSVReadingOptions::default())?;
    let right = DataFrame::from_columns(&[
        Column::strings(
            "name",
            vec![Some("Ada".to_string()), Some("Grace".to_string())],
        ),
        Column::strings(
            "team",
            vec![Some("compiler".to_string()), Some("navy".to_string())],
        ),
    ])?;
    let joined = reloaded.joined(&right, "name", JoinKind::Left)?;

    println!("csv = {}", csv_path.display());
    println!("reloaded shape = {:?}", reloaded.shape());
    println!("joined shape = {:?}", joined.shape());
    println!("rows = {}", joined.rows_json()?.len());
    println!("✅ tabulardata dataframe + csv OK");
    Ok(())
}
