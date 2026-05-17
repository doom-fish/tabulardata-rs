mod shared;

use tabulardata::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut frame = shared::fixture_frame()?;
    frame.insert_column(
        1,
        &Column::strings(
            "level",
            vec![
                Some("staff".into()),
                Some("staff".into()),
                Some("principal".into()),
                Some("staff".into()),
            ],
        ),
    )?;
    frame.combine_columns2("name", "team", "label", |name, team| {
        AnyValue::String(format!("{}:{}", name.as_str().unwrap(), team.as_str().unwrap()))
    })?;
    frame.transform_non_null_column("score", |value| AnyValue::Double(value.as_f64().unwrap() + 1.0))?;
    println!("{frame}");
    Ok(())
}
