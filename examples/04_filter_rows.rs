#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    let filtered = frame.filtered(&Filter::and(vec![
        Filter::eq("team", "compiler"),
        Filter::contains("city", "on"),
    ]))?;
    println!("filtered shape = {:?}", filtered.shape());
    Ok(())
}
