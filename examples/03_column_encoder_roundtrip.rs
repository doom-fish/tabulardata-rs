#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = support::fixture_frame()?;
    frame.encode_column("name", ColumnElementType::String, ColumnCodec::Json)?;
    println!("encoded type = {}", frame.any_column("name")?.type_name);
    frame.decode_column("name", ColumnElementType::String, ColumnCodec::Json)?;
    println!("decoded names = {:?}", frame.any_column("name")?.values);
    Ok(())
}
