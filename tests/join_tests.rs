mod common;

use tabulardata::prelude::*;

#[test]
fn joins_support_different_left_and_right_keys() -> Result<(), Box<dyn std::error::Error>> {
    let people = common::fixture_frame()?;
    let departments = common::department_frame()?;
    let joined = people.joined_on(
        &departments,
        JoinColumns::new("id", "employee_id"),
        JoinKind::Left,
    )?;

    assert_eq!(joined.row_count(), 4);
    assert_eq!(joined.any_column("office")?.missing_count, 1);
    Ok(())
}
