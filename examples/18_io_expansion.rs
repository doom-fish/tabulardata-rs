mod shared;

use std::fs;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let csv = b"id,name,score\n1,Ada,98.5\n2,Grace,91.0\n";
    let frame = DataFrame::from_csv_data(csv, CSVReadingOptions::new())?;
    println!("csv shape = {:?}", frame.shape());

    let sframe_dir = shared::output_path("missing.sframe");
    fs::create_dir_all(&sframe_dir)?;
    match DataFrame::from_sframe(&sframe_dir) {
        Ok(frame) => println!("unexpected sframe shape = {:?}", frame.shape()),
        Err(error) => println!("sframe error = {}", SFrameReadingError::parse(error.message())),
    }
    Ok(())
}
