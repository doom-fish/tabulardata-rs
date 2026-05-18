use serde::{Deserialize, Serialize};

use crate::dataframe::{encode_csv_write_options, CSVWritingOptions, DataFrame};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;

/// Wraps date-writing strategies accepted by `TabularData` `CSV` and `JSON` writer counterparts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DateWriteStrategy {
    /// Wraps the `TabularData` `DateWriteStrategy.iso8601` case.
    Iso8601,
    /// Wraps the `TabularData` `DateWriteStrategy.customFormat` case.
    CustomFormat(String),
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.csvString` counterpart.
    pub fn csv_string(&self, options: &CSVWritingOptions) -> Result<String, TabularDataError> {
        let options = encode_csv_write_options(options)?;
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::td_dataframe_csv_string(self.as_raw(), options.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            crate::private::decode_json(payload)
        }
    }
}
