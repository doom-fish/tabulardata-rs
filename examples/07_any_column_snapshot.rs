#[path = "shared/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    let column = frame.any_column("team")?;
    println!(
        "type = {} distinct = {}",
        column.type_name,
        column.distinct().len()
    );
    Ok(())
}
