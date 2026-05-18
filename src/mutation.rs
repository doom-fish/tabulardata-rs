use crate::any_column::{AnyColumn, AnyValue};
use crate::any_row::AnyRow;
use crate::column::Column;
use crate::dataframe::DataFrame;
use crate::error::TabularDataError;

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.tryClone` counterpart.
    pub fn try_clone(&self) -> Result<Self, TabularDataError> {
        self.slice_rows(0..self.row_count())
    }

    /// Wraps the `TabularData` `DataFrame.columnAt` counterpart.
    pub fn column_at(&self, index: usize) -> Result<Column, TabularDataError> {
        let names = self.column_names()?;
        let name = names.get(index).ok_or_else(|| {
            TabularDataError::InvalidArgument(format!("column index {index} is out of bounds"))
        })?;
        self.column(name)
    }

    /// Wraps the `TabularData` `DataFrame.anyColumnAt` counterpart.
    pub fn any_column_at(&self, index: usize) -> Result<AnyColumn, TabularDataError> {
        let names = self.column_names()?;
        let name = names.get(index).ok_or_else(|| {
            TabularDataError::InvalidArgument(format!("column index {index} is out of bounds"))
        })?;
        self.any_column(name)
    }

    /// Wraps the `TabularData` `DataFrame.containsColumnId` counterpart.
    pub fn contains_column_id<T>(
        &self,
        id: &crate::protocols::ColumnId<T>,
    ) -> Result<bool, TabularDataError> {
        self.contains_column(id.name())
    }

    /// Wraps the `TabularData` `DataFrame.columnById` counterpart.
    pub fn column_by_id<T>(
        &self,
        id: &crate::protocols::ColumnId<T>,
    ) -> Result<Column, TabularDataError> {
        self.column(id.name())
    }

    /// Wraps the `TabularData` `DataFrame.maskRows` counterpart.
    pub fn mask_rows(&self, mask: &[bool]) -> Result<Self, TabularDataError> {
        if mask.len() != self.row_count() {
            return Err(TabularDataError::InvalidArgument(format!(
                "row mask length {} does not match row count {}",
                mask.len(),
                self.row_count()
            )));
        }
        let rows: Vec<AnyRow> = self
            .rows()?
            .into_iter()
            .zip(mask.iter().copied())
            .filter_map(|(row, include)| include.then_some(row))
            .collect();
        if rows.is_empty() {
            return self.slice_rows(0..0);
        }
        ordered_frame_from_rows(&rows, &self.column_names()?)
    }

    /// Wraps the `TabularData` `DataFrame.filteredByColumn` counterpart.
    pub fn filtered_by_column<F>(
        &self,
        name: &str,
        mut predicate: F,
    ) -> Result<Self, TabularDataError>
    where
        F: FnMut(&AnyValue) -> bool,
    {
        let column = self.any_column(name)?;
        let mask: Vec<bool> = column.values.iter().map(&mut predicate).collect();
        self.mask_rows(&mask)
    }

    /// Wraps the `TabularData` `DataFrame.appendValues` counterpart.
    pub fn append_values(&mut self, values: &[AnyValue]) -> Result<(), TabularDataError> {
        let column_names = self.column_names()?;
        if column_names.len() != values.len() {
            return Err(TabularDataError::InvalidArgument(format!(
                "expected {} values, got {}",
                column_names.len(),
                values.len()
            )));
        }
        let mut row = AnyRow::new();
        for (column, value) in column_names.into_iter().zip(values.iter().cloned()) {
            let _ = row.insert(column, value);
        }
        self.append_row(&row)
    }

    /// Wraps the `TabularData` `DataFrame.appendRowsOf` counterpart.
    pub fn append_rows_of(&mut self, other: &Self) -> Result<(), TabularDataError> {
        let expected = self.column_names()?;
        let actual = other.column_names()?;
        if expected != actual {
            return Err(TabularDataError::InvalidArgument(
                "frames must have the same columns to append rows".into(),
            ));
        }
        for row in other.rows()? {
            self.append_row(&row)?;
        }
        Ok(())
    }

    /// Wraps the `TabularData` `DataFrame.appendFrame` counterpart.
    pub fn append_frame(&mut self, other: &Self) -> Result<(), TabularDataError> {
        self.append_rows_of(other)
    }

    /// Wraps the `TabularData` `DataFrame.insertColumn` counterpart.
    pub fn insert_column(&mut self, index: usize, column: &Column) -> Result<(), TabularDataError> {
        self.validate_column_length(column.len())?;
        let mut columns = self.owned_columns()?;
        if index > columns.len() {
            return Err(TabularDataError::InvalidArgument(format!(
                "column index {index} is out of bounds"
            )));
        }
        columns.insert(index, column.clone());
        self.rebuild_from_columns(&columns)
    }

    /// Wraps the `TabularData` `DataFrame.replaceColumn` counterpart.
    pub fn replace_column(&mut self, name: &str, column: &Column) -> Result<(), TabularDataError> {
        self.validate_column_length(column.len())?;
        let mut columns = self.owned_columns()?;
        let index = columns
            .iter()
            .position(|existing| existing.name == name)
            .ok_or_else(|| {
                TabularDataError::InvalidArgument(format!("no column named '{name}'"))
            })?;
        columns[index] = column.clone();
        self.rebuild_from_columns(&columns)
    }

    /// Wraps the `TabularData` `DataFrame.removeColumn` counterpart.
    pub fn remove_column(&mut self, name: &str) -> Result<Column, TabularDataError> {
        let mut columns = self.owned_columns()?;
        let index = columns
            .iter()
            .position(|existing| existing.name == name)
            .ok_or_else(|| {
                TabularDataError::InvalidArgument(format!("no column named '{name}'"))
            })?;
        let removed = columns.remove(index);
        self.rebuild_from_columns(&columns)?;
        Ok(removed)
    }

    /// Wraps the `TabularData` `DataFrame.transformColumn` counterpart.
    pub fn transform_column<F>(
        &mut self,
        name: &str,
        mut transform: F,
    ) -> Result<(), TabularDataError>
    where
        F: FnMut(&AnyValue) -> AnyValue,
    {
        let column = self.any_column(name)?;
        let values: Vec<AnyValue> = column.values.iter().map(&mut transform).collect();
        let replacement = Column::from_any_values(name.to_string(), &column.type_name, &values)?;
        self.replace_column(name, &replacement)
    }

    /// Wraps the `TabularData` `DataFrame.transformNonNullColumn` counterpart.
    pub fn transform_non_null_column<F>(
        &mut self,
        name: &str,
        mut transform: F,
    ) -> Result<(), TabularDataError>
    where
        F: FnMut(&AnyValue) -> AnyValue,
    {
        self.transform_column(name, |value| {
            if value.is_null() {
                AnyValue::Null
            } else {
                transform(value)
            }
        })
    }

    /// Wraps the `TabularData` `DataFrame.combineColumns2` counterpart.
    pub fn combine_columns2<F>(
        &mut self,
        left: &str,
        right: &str,
        new_name: &str,
        mut transform: F,
    ) -> Result<(), TabularDataError>
    where
        F: FnMut(&AnyValue, &AnyValue) -> AnyValue,
    {
        let left_column = self.any_column(left)?;
        let right_column = self.any_column(right)?;
        if left_column.len() != right_column.len() {
            return Err(TabularDataError::InvalidArgument(
                "columns must have the same length".into(),
            ));
        }
        let values: Vec<AnyValue> = left_column
            .values
            .iter()
            .zip(&right_column.values)
            .map(|(left_value, right_value)| transform(left_value, right_value))
            .collect();
        self.upsert_derived_column(new_name, &values)
    }

    /// Wraps the `TabularData` `DataFrame.combineColumns3` counterpart.
    pub fn combine_columns3<F>(
        &mut self,
        first: &str,
        second: &str,
        third: &str,
        new_name: &str,
        mut transform: F,
    ) -> Result<(), TabularDataError>
    where
        F: FnMut(&AnyValue, &AnyValue, &AnyValue) -> AnyValue,
    {
        let first_column = self.any_column(first)?;
        let second_column = self.any_column(second)?;
        let third_column = self.any_column(third)?;
        if first_column.len() != second_column.len() || first_column.len() != third_column.len() {
            return Err(TabularDataError::InvalidArgument(
                "columns must have the same length".into(),
            ));
        }
        let values: Vec<AnyValue> = first_column
            .values
            .iter()
            .zip(&second_column.values)
            .zip(&third_column.values)
            .map(|((first_value, second_value), third_value)| {
                transform(first_value, second_value, third_value)
            })
            .collect();
        self.upsert_derived_column(new_name, &values)
    }

    /// Wraps the `TabularData` `DataFrame.explodeColumn` counterpart.
    pub fn explode_column(&mut self, name: &str) -> Result<(), TabularDataError> {
        let replacement = self.exploding_column(name)?;
        self.replace_with(replacement);
        Ok(())
    }

    /// Wraps the `TabularData` `DataFrame.explodingColumn` counterpart.
    pub fn exploding_column(&self, name: &str) -> Result<Self, TabularDataError> {
        let column_names = self.column_names()?;
        let mut rows = Vec::new();
        for row in self.rows()? {
            match row.get(name).cloned().unwrap_or_default() {
                AnyValue::Array(values) if !values.is_empty() => {
                    for value in values {
                        let mut expanded = row.clone();
                        let _ = expanded.insert(name.to_string(), value);
                        rows.push(expanded);
                    }
                }
                AnyValue::Array(_) => {
                    let mut expanded = row.clone();
                    let _ = expanded.insert(name.to_string(), AnyValue::Null);
                    rows.push(expanded);
                }
                _ => rows.push(row),
            }
        }
        if rows.is_empty() {
            return self.slice_rows(0..0);
        }
        ordered_frame_from_rows(&rows, &column_names)
    }

    fn owned_columns(&self) -> Result<Vec<Column>, TabularDataError> {
        self.column_names()?
            .into_iter()
            .map(|name| self.column(&name))
            .collect()
    }

    fn rebuild_from_columns(&mut self, columns: &[Column]) -> Result<(), TabularDataError> {
        let frame = if columns.is_empty() {
            let mut frame = Self::new()?;
            for _ in 0..self.row_count() {
                frame.append_empty_row()?;
            }
            frame
        } else {
            Self::from_columns(columns)?
        };
        self.replace_with(frame);
        Ok(())
    }

    fn validate_column_length(&self, len: usize) -> Result<(), TabularDataError> {
        if self.row_count() == 0 || self.row_count() == len {
            Ok(())
        } else {
            Err(TabularDataError::InvalidArgument(format!(
                "column length {len} does not match row count {}",
                self.row_count()
            )))
        }
    }

    fn upsert_derived_column(
        &mut self,
        new_name: &str,
        values: &[AnyValue],
    ) -> Result<(), TabularDataError> {
        let column = Column::from_any_values(new_name.to_string(), infer_type(values), values)?;
        if self.contains_column(new_name)? {
            self.replace_column(new_name, &column)
        } else {
            self.insert_column(self.column_count(), &column)
        }
    }
}

fn ordered_frame_from_rows(
    rows: &[AnyRow],
    column_names: &[String],
) -> Result<DataFrame, TabularDataError> {
    let frame = DataFrame::from_rows(rows)?;
    frame.select_columns(column_names)
}

fn infer_type(values: &[AnyValue]) -> &'static str {
    values
        .iter()
        .find(|value| !value.is_null())
        .map_or("string", AnyValue::type_name)
}

impl std::fmt::Debug for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataFrame")
            .field("shape", &self.shape())
            .finish()
    }
}

impl std::fmt::Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.description() {
            Ok(description) => f.write_str(&description),
            Err(_) => std::fmt::Debug::fmt(self, f),
        }
    }
}
