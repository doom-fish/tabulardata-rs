mod common;

use tabulardata::prelude::*;

#[test]
fn typed_column_protocols_and_shaped_data_work() -> Result<(), Box<dyn std::error::Error>> {
    let joined = Column::dates("joined_at", vec![Some(10.0), None, Some(30.0)]);
    assert_eq!(joined.type_name(), "Date");
    assert_eq!(joined.missing_count(), 1);
    assert_eq!(joined.min(), Some(AnyValue::Date(10.0)));
    assert_eq!(joined.max(), Some(AnyValue::Date(30.0)));

    let prototype = joined.prototype();
    assert_eq!(prototype.name(), "joined_at");
    assert_eq!(prototype.wrapped_element_type(), "Date");
    let empty = prototype.make_column(8);
    assert_eq!(empty.type_name(), "Date");
    assert!(empty.is_empty());

    let binary = Column::binary(
        "payload",
        vec![Some("SGVsbG8=".to_string()), Some("V29ybGQ=".to_string()), None],
    );
    let frame = DataFrame::from_columns(&[joined, binary])?;
    assert_eq!(frame.column("joined_at")?.type_name(), "Date");
    assert_eq!(frame.column("payload")?.type_name(), "Data");

    let any = frame.any_column("joined_at")?;
    assert_eq!(any.prototype().wrapped_element_type(), "Date");
    assert_eq!(any.argmax(), Some(2));
    assert_eq!(any.slice_values(0..2).count(), 2);

    let ids = ColumnId::<i64>::new("joined_at");
    assert!(frame.contains_column_id(&ids)?);
    assert_eq!(frame.column_by_id(&ids)?.name, "joined_at");
    assert_eq!(frame.base()?.shape(), frame.shape());

    let shaped = ShapedData::new(vec![2, 2], vec![2, 1], vec![1, 2, 3, 4])?;
    assert_eq!(*shaped.at(&[1, 0])?, 3);
    assert!(shaped.to_string().contains("shape=[2, 2]"));
    Ok(())
}
