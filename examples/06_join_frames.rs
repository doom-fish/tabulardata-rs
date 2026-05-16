#[path = "shared/mod.rs"]
mod support;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let people = support::fixture_frame()?;
    let offices = support::department_frame()?;
    let joined = people.joined_on(
        &offices,
        JoinColumns::new("id", "employee_id"),
        JoinKind::Left,
    )?;
    println!("joined columns = {:?}", joined.column_names()?);
    Ok(())
}
