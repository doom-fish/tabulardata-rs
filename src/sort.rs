use serde::{Deserialize, Serialize};

use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortKey {
    pub column: String,
    pub order: SortOrder,
}

impl SortKey {
    #[must_use]
    pub fn ascending(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            order: SortOrder::Ascending,
        }
    }

    #[must_use]
    pub fn descending(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            order: SortOrder::Descending,
        }
    }
}

impl DataFrame {
    pub fn sorted_by(&self, keys: &[SortKey]) -> Result<Self, TabularDataError> {
        let keys = encode_json_cstring(&keys, "sort keys")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_sort_json(self.as_raw(), keys.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn sort_by(&mut self, keys: &[SortKey]) -> Result<(), TabularDataError> {
        let sorted = self.sorted_by(keys)?;
        self.replace_with(sorted);
        Ok(())
    }
}
