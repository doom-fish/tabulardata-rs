use std::ops::Range;

use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

impl DataFrame {
    pub fn slice_rows(&self, range: Range<usize>) -> Result<Self, TabularDataError> {
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_slice_rows(
                self.as_raw(),
                range.start,
                range.end,
                &mut raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn prefix_rows(&self, len: usize) -> Result<Self, TabularDataError> {
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status =
            unsafe { ffi::td_dataframe_prefix_rows(self.as_raw(), len, &mut raw, &mut error) };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn suffix_rows(&self, len: usize) -> Result<Self, TabularDataError> {
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status =
            unsafe { ffi::td_dataframe_suffix_rows(self.as_raw(), len, &mut raw, &mut error) };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn select_columns<S: AsRef<str>>(&self, columns: &[S]) -> Result<Self, TabularDataError> {
        let columns: Vec<String> = columns
            .iter()
            .map(|column| column.as_ref().to_string())
            .collect();
        let columns = encode_json_cstring(&columns, "column selection")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_select_columns_json(
                self.as_raw(),
                columns.as_ptr(),
                &mut raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }
}
