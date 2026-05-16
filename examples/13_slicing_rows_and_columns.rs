#[path = "shared/mod.rs"]
mod support;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = support::fixture_frame()?;
    let prefix = frame.prefix_rows(2)?;
    let selected = prefix.select_columns(&["name", "score"])?;
    println!("selected shape = {:?}", selected.shape());
    Ok(())
}
