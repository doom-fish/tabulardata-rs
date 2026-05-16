#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    let sorted = frame.sorted_by(&[SortKey::ascending("team"), SortKey::descending("score")])?;
    println!("sorted names = {:?}", sorted.rows()?);
    Ok(())
}
