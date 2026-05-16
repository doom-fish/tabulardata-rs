use core::ffi::c_void;
use std::path::Path;

use serde::Serialize;
use serde_json::Value;

use crate::column::{decode_column_json, encode_column_json, Column};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::{decode_json, to_cstring};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSVReadingOptions {
    pub has_header_row: bool,
    pub delimiter: char,
    pub ignores_empty_lines: bool,
    pub uses_quoting: bool,
    pub uses_escaping: bool,
}

impl CSVReadingOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_has_header_row(mut self, has_header_row: bool) -> Self {
        self.has_header_row = has_header_row;
        self
    }

    #[must_use]
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    #[must_use]
    pub fn with_ignores_empty_lines(mut self, ignores_empty_lines: bool) -> Self {
        self.ignores_empty_lines = ignores_empty_lines;
        self
    }

    #[must_use]
    pub fn with_uses_quoting(mut self, uses_quoting: bool) -> Self {
        self.uses_quoting = uses_quoting;
        self
    }

    #[must_use]
    pub fn with_uses_escaping(mut self, uses_escaping: bool) -> Self {
        self.uses_escaping = uses_escaping;
        self
    }
}

impl Default for CSVReadingOptions {
    fn default() -> Self {
        Self {
            has_header_row: true,
            delimiter: ',',
            ignores_empty_lines: true,
            uses_quoting: true,
            uses_escaping: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSVWritingOptions {
    pub includes_header: bool,
    pub nil_encoding: String,
    pub true_encoding: String,
    pub false_encoding: String,
    pub newline: String,
    pub delimiter: char,
}

impl CSVWritingOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_includes_header(mut self, includes_header: bool) -> Self {
        self.includes_header = includes_header;
        self
    }

    #[must_use]
    pub fn with_nil_encoding(mut self, nil_encoding: impl Into<String>) -> Self {
        self.nil_encoding = nil_encoding.into();
        self
    }

    #[must_use]
    pub fn with_true_encoding(mut self, true_encoding: impl Into<String>) -> Self {
        self.true_encoding = true_encoding.into();
        self
    }

    #[must_use]
    pub fn with_false_encoding(mut self, false_encoding: impl Into<String>) -> Self {
        self.false_encoding = false_encoding.into();
        self
    }

    #[must_use]
    pub fn with_newline(mut self, newline: impl Into<String>) -> Self {
        self.newline = newline.into();
        self
    }

    #[must_use]
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }
}

impl Default for CSVWritingOptions {
    fn default() -> Self {
        Self {
            includes_header: true,
            nil_encoding: String::new(),
            true_encoding: "true".into(),
            false_encoding: "false".into(),
            newline: "\n".into(),
            delimiter: ',',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum JoinKind {
    Inner = 0,
    Left = 1,
    Right = 2,
    Full = 3,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize)]
struct CSVReadingOptionsPayload {
    has_header_row: bool,
    delimiter: String,
    ignores_empty_lines: bool,
    uses_quoting: bool,
    uses_escaping: bool,
}

#[derive(Serialize)]
struct CSVWritingOptionsPayload {
    includes_header: bool,
    nil_encoding: String,
    true_encoding: String,
    false_encoding: String,
    newline: String,
    delimiter: String,
}

pub struct DataFrame {
    raw: *mut c_void,
}

unsafe impl Send for DataFrame {}
unsafe impl Sync for DataFrame {}

impl DataFrame {
    pub fn new() -> Result<Self, TabularDataError> {
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::td_dataframe_new(&mut raw, &mut error) };
        if status == ffi::status::OK {
            Ok(Self { raw })
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn from_columns(columns: &[Column]) -> Result<Self, TabularDataError> {
        let mut frame = Self::new()?;
        for column in columns {
            frame.append_column(column)?;
        }
        Ok(frame)
    }

    pub fn from_csv(
        path: impl AsRef<Path>,
        options: CSVReadingOptions,
    ) -> Result<Self, TabularDataError> {
        let path = path_to_cstring(path)?;
        let options = encode_json(
            &CSVReadingOptionsPayload::from(options),
            "CSV reading options",
        )?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_from_csv(path.as_ptr(), options.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self { raw })
        } else {
            Err(from_swift(status, error))
        }
    }

    #[must_use]
    pub fn shape(&self) -> (usize, usize) {
        let mut rows = 0;
        let mut columns = 0;
        unsafe { ffi::td_dataframe_shape(self.raw, &mut rows, &mut columns) };
        (rows, columns)
    }

    #[must_use]
    pub fn row_count(&self) -> usize {
        self.shape().0
    }

    #[must_use]
    pub fn column_count(&self) -> usize {
        self.shape().1
    }

    pub fn column_names(&self) -> Result<Vec<String>, TabularDataError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_column_names_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }

    pub fn append_column(&mut self, column: &Column) -> Result<(), TabularDataError> {
        let column = encode_column_json(column)?;
        let column = to_cstring(&column)?;
        let mut error = core::ptr::null_mut();
        let status =
            unsafe { ffi::td_dataframe_append_column(self.raw, column.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn rename_column(&mut self, name: &str, new_name: &str) -> Result<(), TabularDataError> {
        let name = to_cstring(name)?;
        let new_name = to_cstring(new_name)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_rename_column(self.raw, name.as_ptr(), new_name.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn column(&self, name: &str) -> Result<Column, TabularDataError> {
        let name = to_cstring(name)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_column_json(self.raw, name.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_column_json(payload)
        }
    }

    pub fn rows_json(&self) -> Result<Vec<Value>, TabularDataError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_rows_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }

    pub fn summary(&self) -> Result<Self, TabularDataError> {
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::td_dataframe_summary(self.raw, &mut raw, &mut error) };
        if status == ffi::status::OK {
            Ok(Self { raw })
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn joined(
        &self,
        other: &Self,
        column_name: &str,
        kind: JoinKind,
    ) -> Result<Self, TabularDataError> {
        let column_name = to_cstring(column_name)?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_joined(
                self.raw,
                other.raw,
                column_name.as_ptr(),
                kind as i32,
                &mut raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(Self { raw })
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn write_csv(
        &self,
        path: impl AsRef<Path>,
        options: CSVWritingOptions,
    ) -> Result<(), TabularDataError> {
        let path = path_to_cstring(path)?;
        let options = encode_json(
            &CSVWritingOptionsPayload::from(options),
            "CSV writing options",
        )?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_write_csv(self.raw, path.as_ptr(), options.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }
}

impl Drop for DataFrame {
    fn drop(&mut self) {
        unsafe { ffi::td_object_release(self.raw) };
    }
}

impl From<CSVReadingOptions> for CSVReadingOptionsPayload {
    fn from(value: CSVReadingOptions) -> Self {
        Self {
            has_header_row: value.has_header_row,
            delimiter: value.delimiter.to_string(),
            ignores_empty_lines: value.ignores_empty_lines,
            uses_quoting: value.uses_quoting,
            uses_escaping: value.uses_escaping,
        }
    }
}

impl From<CSVWritingOptions> for CSVWritingOptionsPayload {
    fn from(value: CSVWritingOptions) -> Self {
        Self {
            includes_header: value.includes_header,
            nil_encoding: value.nil_encoding,
            true_encoding: value.true_encoding,
            false_encoding: value.false_encoding,
            newline: value.newline,
            delimiter: value.delimiter.to_string(),
        }
    }
}

fn encode_json<T: Serialize>(
    value: &T,
    label: &str,
) -> Result<std::ffi::CString, TabularDataError> {
    let json = serde_json::to_string(value).map_err(|error| {
        TabularDataError::FrameworkError(format!("failed to encode {label}: {error}"))
    })?;
    to_cstring(&json)
}

fn path_to_cstring(path: impl AsRef<Path>) -> Result<std::ffi::CString, TabularDataError> {
    let path = path.as_ref().to_string_lossy().into_owned();
    to_cstring(&path)
}
