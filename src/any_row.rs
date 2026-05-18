use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::any_column::AnyValue;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

/// Wraps the `TabularData` `DataFrame.Row` counterpart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnyRow {
    /// Wraps the `TabularData` `AnyRow.index` counterpart.
    pub index: Option<usize>,
    /// Wraps the `TabularData` `AnyRow.values` counterpart.
    pub values: BTreeMap<String, AnyValue>,
}

impl AnyRow {
    /// Wraps the `TabularData` `AnyRow.init` counterpart.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Wraps the `TabularData` `AnyRow.withValue` counterpart.
    #[must_use]
    pub fn with_value(mut self, column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        self.values.insert(column.into(), value.into());
        self
    }

    /// Wraps the `TabularData` `AnyRow.insert` counterpart.
    #[must_use]
    pub fn insert(
        &mut self,
        column: impl Into<String>,
        value: impl Into<AnyValue>,
    ) -> Option<AnyValue> {
        self.values.insert(column.into(), value.into())
    }

    /// Wraps the `TabularData` `AnyRow.get` counterpart.
    #[must_use]
    pub fn get(&self, column: &str) -> Option<&AnyValue> {
        self.values.get(column)
    }

    /// Wraps the `TabularData` `AnyRow.len` counterpart.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Wraps the `TabularData` `AnyRow.isEmpty` counterpart.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.fromRows` counterpart.
    pub fn from_rows(rows: &[AnyRow]) -> Result<Self, TabularDataError> {
        let rows = encode_json_cstring(&rows, "row payload")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status =
            unsafe { ffi::td_dataframe_from_rows_json(rows.as_ptr(), &mut raw, &mut error) };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.row` counterpart.
    pub fn row(&self, index: usize) -> Result<AnyRow, TabularDataError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_row_json(self.as_raw(), index, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            crate::private::decode_json(payload)
        }
    }

    /// Wraps the `TabularData` `DataFrame.rows` counterpart.
    pub fn rows(&self) -> Result<Vec<AnyRow>, TabularDataError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_any_rows_json(self.as_raw(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            crate::private::decode_json(payload)
        }
    }

    /// Wraps the `TabularData` `DataFrame.appendRow` counterpart.
    pub fn append_row(&mut self, row: &AnyRow) -> Result<(), TabularDataError> {
        let row = encode_json_cstring(row, "row payload")?;
        let mut error = core::ptr::null_mut();
        let status =
            unsafe { ffi::td_dataframe_append_row_json(self.as_raw(), row.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.insertRow` counterpart.
    pub fn insert_row(&mut self, index: usize, row: &AnyRow) -> Result<(), TabularDataError> {
        let row = encode_json_cstring(row, "row payload")?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_insert_row_json(self.as_raw(), index, row.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.replaceRow` counterpart.
    pub fn replace_row(&mut self, index: usize, row: &AnyRow) -> Result<(), TabularDataError> {
        let row = encode_json_cstring(row, "row payload")?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_replace_row_json(self.as_raw(), index, row.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.appendEmptyRow` counterpart.
    pub fn append_empty_row(&mut self) -> Result<(), TabularDataError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::td_dataframe_append_empty_row(self.as_raw(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.removeRow` counterpart.
    pub fn remove_row(&mut self, index: usize) -> Result<(), TabularDataError> {
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::td_dataframe_remove_row(self.as_raw(), index, &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }
}
