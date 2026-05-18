use serde::{Deserialize, Serialize};

use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

/// Wraps sort-order cases accepted by `TabularData` `DataFrame.sorted` counterparts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Wraps the `TabularData` `SortOrder.ascending` case.
    Ascending,
    /// Wraps the `TabularData` `SortOrder.descending` case.
    Descending,
}

/// Wraps sort keys accepted by `TabularData` `DataFrame.sorted` counterparts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortKey {
    /// Wraps the `TabularData` `SortKey.column` counterpart.
    pub column: String,
    /// Wraps the `TabularData` `SortKey.order` counterpart.
    pub order: SortOrder,
}

impl SortKey {
    /// Wraps the `TabularData` `SortKey.ascending` counterpart.
    #[must_use]
    pub fn ascending(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            order: SortOrder::Ascending,
        }
    }

    /// Wraps the `TabularData` `SortKey.descending` counterpart.
    #[must_use]
    pub fn descending(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            order: SortOrder::Descending,
        }
    }
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.sortedBy` counterpart.
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

    /// Wraps the `TabularData` `DataFrame.sortBy` counterpart.
    pub fn sort_by(&mut self, keys: &[SortKey]) -> Result<(), TabularDataError> {
        let sorted = self.sorted_by(keys)?;
        self.replace_with(sorted);
        Ok(())
    }
}
