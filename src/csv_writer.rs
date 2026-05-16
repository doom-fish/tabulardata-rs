use serde::{Deserialize, Serialize};

use crate::dataframe::{encode_csv_write_options, CSVWritingOptions, DataFrame};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DateWriteStrategy {
    Iso8601,
    CustomFormat(String),
}

impl DataFrame {
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
