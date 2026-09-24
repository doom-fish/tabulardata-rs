use crate::any_column::{AnyColumn, AnyValue};
use crate::any_row::AnyRow;
use crate::column::{encode_column_json, Column};
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::{decode_json, encode_json_cstring, to_cstring};

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
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_mask_rows(
                self.as_raw(),
                mask.as_ptr(),
                mask.len(),
                &raw mut raw,
                &raw mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
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
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_append_rows_of(self.as_raw(), other.as_raw(), &raw mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.appendFrame` counterpart.
    pub fn append_frame(&mut self, other: &Self) -> Result<(), TabularDataError> {
        self.append_rows_of(other)
    }

    /// Wraps the `TabularData` `DataFrame.insertColumn` counterpart.
    pub fn insert_column(&mut self, index: usize, column: &Column) -> Result<(), TabularDataError> {
        let column = to_cstring(&encode_column_json(column)?)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_insert_column_json(
                self.as_raw(),
                index,
                column.as_ptr(),
                &raw mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.replaceColumn` counterpart.
    pub fn replace_column(&mut self, name: &str, column: &Column) -> Result<(), TabularDataError> {
        let name = to_cstring(name)?;
        let column = to_cstring(&encode_column_json(column)?)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_replace_column_json(
                self.as_raw(),
                name.as_ptr(),
                column.as_ptr(),
                &raw mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.removeColumn` counterpart.
    pub fn remove_column(&mut self, name: &str) -> Result<AnyColumn, TabularDataError> {
        let name = to_cstring(name)?;
        let mut payload = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_remove_column_json(
                self.as_raw(),
                name.as_ptr(),
                &raw mut payload,
                &raw mut error,
            )
        };
        if status == ffi::status::OK {
            decode_json(payload)
        } else {
            Err(from_swift(status, error))
        }
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
        let values: Vec<AnyValue> = self
            .any_column(name)?
            .values
            .iter()
            .map(&mut transform)
            .collect();
        let name = to_cstring(name)?;
        let values = encode_json_cstring(&values, "transformed values")?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_transform_column_json(
                self.as_raw(),
                name.as_ptr(),
                values.as_ptr(),
                &raw mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
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
        let name = to_cstring(name)?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_exploding_column(
                self.as_raw(),
                name.as_ptr(),
                &raw mut raw,
                &raw mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
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
