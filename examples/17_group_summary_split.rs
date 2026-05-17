mod shared;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = shared::fixture_frame()?;
    let grouped = frame.group_by(&["team"]);
    let summaries = grouped.summary()?;
    println!("{}", summaries.description());
    let (left, right) = grouped.random_split(0.5, Some(42))?;
    println!("split = ({}, {})", left.row_count(), right.row_count());
    Ok(())
}
