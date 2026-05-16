#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    let options = CSVWritingOptions::new().with_delimiter(';');
    let csv = frame.csv_string(&options)?;
    let path = support::output_path("writer.csv");
    frame.write_csv(&path, &options)?;
    println!("csv bytes = {} path = {}", csv.len(), path.display());
    Ok(())
}
