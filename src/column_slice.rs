use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::any_column::AnyValue;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::{encode_json_cstring, to_cstring};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnSlice {
    pub name: String,
    pub type_name: String,
    pub missing_count: usize,
    pub values: Vec<AnyValue>,
    pub contiguous: bool,
    pub indices: Vec<usize>,
}

impl ColumnSlice {
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        type_name: impl Into<String>,
        values: Vec<AnyValue>,
        contiguous: bool,
        indices: Vec<usize>,
    ) -> Self {
        let missing_count = values.iter().filter(|value| value.is_null()).count();
        Self {
            name: name.into(),
            type_name: type_name.into(),
            missing_count,
            values,
            contiguous,
            indices,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[must_use]
    pub fn value(&self, index: usize) -> Option<&AnyValue> {
        self.values.get(index)
    }

    #[must_use]
    pub fn range(&self, range: Range<usize>) -> Self {
        let start = range.start.min(self.values.len());
        let end = range.end.min(self.values.len());
        Self::new(
            self.name.clone(),
            self.type_name.clone(),
            self.values[start..end].to_vec(),
            self.contiguous,
            self.indices[start..end].to_vec(),
        )
    }

    #[must_use]
    pub fn distinct(&self) -> Self {
        let mut seen = std::collections::BTreeSet::new();
        let mut values = Vec::new();
        let mut indices = Vec::new();
        for (index, value) in self.values.iter().enumerate() {
            let key = value.stable_key();
            if seen.insert(key) {
                values.push(value.clone());
                indices.push(self.indices[index]);
            }
        }
        Self::new(
            self.name.clone(),
            self.type_name.clone(),
            values,
            false,
            indices,
        )
    }

    #[must_use]
    pub fn summary(&self) -> crate::summary::ColumnSummary {
        crate::summary::summarize_values(&self.values)
    }
}

impl DataFrame {
    pub fn column_slice(
        &self,
        name: &str,
        range: Range<usize>,
    ) -> Result<ColumnSlice, TabularDataError> {
        let name = to_cstring(name)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::td_dataframe_column_slice_json(
                self.as_raw(),
                name.as_ptr(),
                range.start,
                range.end,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            crate::private::decode_json(payload)
        }
    }

    pub fn column_mask(&self, name: &str, mask: &[bool]) -> Result<ColumnSlice, TabularDataError> {
        let name = to_cstring(name)?;
        let mask = encode_json_cstring(&mask, "column mask")?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::td_dataframe_column_mask_json(
                self.as_raw(),
                name.as_ptr(),
                mask.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            crate::private::decode_json(payload)
        }
    }
}
