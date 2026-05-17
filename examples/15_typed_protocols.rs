mod shared;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = DataFrame::from_columns(&[
        Column::dates(
            "joined_at",
            vec![Some(1_704_067_200.0), Some(1_704_153_600.0)],
        ),
        Column::binary(
            "payload",
            vec![Some("SGVsbG8=".into()), Some("V29ybGQ=".into())],
        ),
    ])?;
    let column = frame.any_column("joined_at")?;
    println!("{}", column.description());
    println!("prototype={}", column.prototype().wrapped_element_type());

    let shaped = ShapedData::new(vec![2, 2], vec![2, 1], vec![1, 2, 3, 4])?;
    println!("value[1,0] = {}", shaped.at(&[1, 0])?);
    Ok(())
}
