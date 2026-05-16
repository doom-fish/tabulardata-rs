#[path = "shared/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    println!("score summary = {:?}", frame.column_summary("score")?);
    println!("team summary = {:?}", frame.column_summary("team")?);
    Ok(())
}
