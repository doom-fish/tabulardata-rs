#[path = "shared/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    let slice = frame.column_slice("score", 1..4)?;
    println!("slice indices = {:?}", slice.indices);
    println!("slice summary = {:?}", slice.summary());
    Ok(())
}
