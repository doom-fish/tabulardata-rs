use core::ffi::c_void;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::path::Path;

use serde::Serialize;
use serde_json::Value;

use crate::column::{decode_column_json, encode_column_json, Column};
use crate::csv_reader::{CSVReadRequest, CSVType, DateParseStrategy};
use crate::csv_writer::DateWriteStrategy;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::join::JoinColumns;
use crate::private::{decode_json, encode_json_cstring, to_cstring};

/// Wraps the `TabularData` `CSVReadingOptions` counterpart.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSVReadingOptions {
    /// Wraps the `TabularData` `CSVReadingOptions.hasHeaderRow` counterpart.
    pub has_header_row: bool,
    /// Wraps the `TabularData` `CSVReadingOptions.nilEncodings` counterpart.
    pub nil_encodings: Vec<String>,
    /// Wraps the `TabularData` `CSVReadingOptions.trueEncodings` counterpart.
    pub true_encodings: Vec<String>,
    /// Wraps the `TabularData` `CSVReadingOptions.falseEncodings` counterpart.
    pub false_encodings: Vec<String>,
    /// Wraps the `TabularData` `CSVReadingOptions.floatingPointType` counterpart.
    pub floating_point_type: CSVType,
    /// Wraps the `TabularData` `CSVReadingOptions.dateParseStrategies` counterpart.
    pub date_parse_strategies: Vec<DateParseStrategy>,
    /// Wraps the `TabularData` `CSVReadingOptions.ignoresEmptyLines` counterpart.
    pub ignores_empty_lines: bool,
    /// Wraps the `TabularData` `CSVReadingOptions.usesQuoting` counterpart.
    pub uses_quoting: bool,
    /// Wraps the `TabularData` `CSVReadingOptions.usesEscaping` counterpart.
    pub uses_escaping: bool,
    /// Wraps the `TabularData` `CSVReadingOptions.delimiter` counterpart.
    pub delimiter: char,
    /// Wraps the `TabularData` `CSVReadingOptions.escapeCharacter` counterpart.
    pub escape_character: char,
}

impl CSVReadingOptions {
    /// Wraps the `TabularData` `CSVReadingOptions.init` counterpart.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withHasHeaderRow` counterpart.
    #[must_use]
    pub fn with_has_header_row(mut self, has_header_row: bool) -> Self {
        self.has_header_row = has_header_row;
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withNilEncodings` counterpart.
    #[must_use]
    pub fn with_nil_encodings<I, S>(mut self, nil_encodings: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.nil_encodings = nil_encodings.into_iter().map(Into::into).collect();
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withTrueEncodings` counterpart.
    #[must_use]
    pub fn with_true_encodings<I, S>(mut self, true_encodings: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.true_encodings = true_encodings.into_iter().map(Into::into).collect();
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withFalseEncodings` counterpart.
    #[must_use]
    pub fn with_false_encodings<I, S>(mut self, false_encodings: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.false_encodings = false_encodings.into_iter().map(Into::into).collect();
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withFloatingPointType` counterpart.
    #[must_use]
    pub fn with_floating_point_type(mut self, floating_point_type: CSVType) -> Self {
        self.floating_point_type = floating_point_type;
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withDateParseStrategy` counterpart.
    #[must_use]
    pub fn with_date_parse_strategy(mut self, strategy: DateParseStrategy) -> Self {
        self.date_parse_strategies.push(strategy);
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withDelimiter` counterpart.
    #[must_use]
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withEscapeCharacter` counterpart.
    #[must_use]
    pub fn with_escape_character(mut self, escape_character: char) -> Self {
        self.escape_character = escape_character;
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withIgnoresEmptyLines` counterpart.
    #[must_use]
    pub fn with_ignores_empty_lines(mut self, ignores_empty_lines: bool) -> Self {
        self.ignores_empty_lines = ignores_empty_lines;
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withUsesQuoting` counterpart.
    #[must_use]
    pub fn with_uses_quoting(mut self, uses_quoting: bool) -> Self {
        self.uses_quoting = uses_quoting;
        self
    }

    /// Wraps the `TabularData` `CSVReadingOptions.withUsesEscaping` counterpart.
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
            nil_encodings: vec![
                "", "#N/A", "#N/A N/A", "#NA", "N/A", "NA", "NULL", "n/a", "nil", "null",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            true_encodings: vec!["1", "True", "TRUE", "true"]
                .into_iter()
                .map(String::from)
                .collect(),
            false_encodings: vec!["0", "False", "FALSE", "false"]
                .into_iter()
                .map(String::from)
                .collect(),
            floating_point_type: CSVType::Double,
            date_parse_strategies: Vec::new(),
            ignores_empty_lines: true,
            uses_quoting: true,
            uses_escaping: false,
            delimiter: ',',
            escape_character: '\\',
        }
    }
}

/// Wraps the `TabularData` `CSVWritingOptions` counterpart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSVWritingOptions {
    /// Wraps the `TabularData` `CSVWritingOptions.includesHeader` counterpart.
    pub includes_header: bool,
    /// Wraps the `TabularData` `CSVWritingOptions.dateStrategy` counterpart.
    pub date_strategy: Option<DateWriteStrategy>,
    /// Wraps the `TabularData` `CSVWritingOptions.nilEncoding` counterpart.
    pub nil_encoding: String,
    /// Wraps the `TabularData` `CSVWritingOptions.trueEncoding` counterpart.
    pub true_encoding: String,
    /// Wraps the `TabularData` `CSVWritingOptions.falseEncoding` counterpart.
    pub false_encoding: String,
    /// Wraps the `TabularData` `CSVWritingOptions.newline` counterpart.
    pub newline: String,
    /// Wraps the `TabularData` `CSVWritingOptions.delimiter` counterpart.
    pub delimiter: char,
}

impl CSVWritingOptions {
    /// Wraps the `TabularData` `CSVWritingOptions.init` counterpart.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Wraps the `TabularData` `CSVWritingOptions.withIncludesHeader` counterpart.
    #[must_use]
    pub fn with_includes_header(mut self, includes_header: bool) -> Self {
        self.includes_header = includes_header;
        self
    }

    /// Wraps the `TabularData` `CSVWritingOptions.withDateStrategy` counterpart.
    #[must_use]
    pub fn with_date_strategy(mut self, date_strategy: DateWriteStrategy) -> Self {
        self.date_strategy = Some(date_strategy);
        self
    }

    /// Wraps the `TabularData` `CSVWritingOptions.withNilEncoding` counterpart.
    #[must_use]
    pub fn with_nil_encoding(mut self, nil_encoding: impl Into<String>) -> Self {
        self.nil_encoding = nil_encoding.into();
        self
    }

    /// Wraps the `TabularData` `CSVWritingOptions.withTrueEncoding` counterpart.
    #[must_use]
    pub fn with_true_encoding(mut self, true_encoding: impl Into<String>) -> Self {
        self.true_encoding = true_encoding.into();
        self
    }

    /// Wraps the `TabularData` `CSVWritingOptions.withFalseEncoding` counterpart.
    #[must_use]
    pub fn with_false_encoding(mut self, false_encoding: impl Into<String>) -> Self {
        self.false_encoding = false_encoding.into();
        self
    }

    /// Wraps the `TabularData` `CSVWritingOptions.withNewline` counterpart.
    #[must_use]
    pub fn with_newline(mut self, newline: impl Into<String>) -> Self {
        self.newline = newline.into();
        self
    }

    /// Wraps the `TabularData` `CSVWritingOptions.withDelimiter` counterpart.
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
            date_strategy: None,
            nil_encoding: String::new(),
            true_encoding: "true".into(),
            false_encoding: "false".into(),
            newline: "\n".into(),
            delimiter: ',',
        }
    }
}

/// Wraps join kinds accepted by the `TabularData` `DataFrame.joined` counterpart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum JoinKind {
    /// Wraps the `TabularData` `JoinKind.inner` case.
    Inner = 0,
    /// Wraps the `TabularData` `JoinKind.left` case.
    Left = 1,
    /// Wraps the `TabularData` `JoinKind.right` case.
    Right = 2,
    /// Wraps the `TabularData` `JoinKind.full` case.
    Full = 3,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize)]
pub(crate) struct CSVReadingOptionsPayload {
    has_header_row: bool,
    nil_encodings: Vec<String>,
    true_encodings: Vec<String>,
    false_encodings: Vec<String>,
    floating_point_type: CSVType,
    date_parse_strategies: Vec<DateParseStrategy>,
    ignores_empty_lines: bool,
    uses_quoting: bool,
    uses_escaping: bool,
    delimiter: String,
    escape_character: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CSVWritingOptionsPayload {
    includes_header: bool,
    date_strategy: Option<DateWriteStrategy>,
    nil_encoding: String,
    true_encoding: String,
    false_encoding: String,
    newline: String,
    delimiter: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CSVReadRequestPayload {
    columns: Option<Vec<String>>,
    rows: Option<[usize; 2]>,
    types: BTreeMap<String, CSVType>,
    options: CSVReadingOptionsPayload,
}

/// Wraps the `TabularData` `DataFrame` counterpart.
pub struct DataFrame {
    raw: *mut c_void,
}

/// SAFETY: `DataFrame` wraps a Swift object handle from the ``TabularData`` framework.
/// The handle is thread-safe (Obj-C reference-counted) and the FFI contract guarantees
/// that calling ``TabularData`` APIs from different threads is safe.
unsafe impl Send for DataFrame {}
/// SAFETY: See `Send` impl above. Additionally, multiple threads can safely hold
/// shared references to the same `DataFrame`.
unsafe impl Sync for DataFrame {}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.init` counterpart.
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

    /// Wraps the `TabularData` `DataFrame.fromColumns` counterpart.
    pub fn from_columns(columns: &[Column]) -> Result<Self, TabularDataError> {
        let mut frame = Self::new()?;
        for column in columns {
            frame.append_column(column)?;
        }
        Ok(frame)
    }

    /// Wraps the `TabularData` `DataFrame.fromCsv` counterpart.
    pub fn from_csv(
        path: impl AsRef<Path>,
        options: CSVReadingOptions,
    ) -> Result<Self, TabularDataError> {
        Self::read_csv_with(path, &CSVReadRequest::new(options))
    }

    /// Wraps the `TabularData` `DataFrame.shape` counterpart.
    #[must_use]
    pub fn shape(&self) -> (usize, usize) {
        let mut rows = 0;
        let mut columns = 0;
        // SAFETY: We own the valid DataFrame handle and it's guaranteed valid by
        // the constructors (new(), from_csv(), etc.). The FFI call only reads metadata.
        unsafe { ffi::td_dataframe_shape(self.raw, &mut rows, &mut columns) };
        (rows, columns)
    }

    /// Wraps the `TabularData` `DataFrame.rowCount` counterpart.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.shape().0
    }

    /// Wraps the `TabularData` `DataFrame.columnCount` counterpart.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.shape().1
    }

    /// Wraps the `TabularData` `DataFrame.columnNames` counterpart.
    pub fn column_names(&self) -> Result<Vec<String>, TabularDataError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_column_names_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }

    /// Wraps the `TabularData` `DataFrame.indexOfColumn` counterpart.
    pub fn index_of_column(&self, name: &str) -> Result<Option<usize>, TabularDataError> {
        let name = to_cstring(name)?;
        let mut found = 0;
        let mut index = 0;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_index_of_column(
                self.raw,
                name.as_ptr(),
                &mut found,
                &mut index,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok((found != 0).then_some(index))
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.containsColumn` counterpart.
    pub fn contains_column(&self, name: &str) -> Result<bool, TabularDataError> {
        let name = to_cstring(name)?;
        let mut contains = 0;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_contains_column(self.raw, name.as_ptr(), &mut contains, &mut error)
        };
        if status == ffi::status::OK {
            Ok(contains != 0)
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.containsColumnOfType` counterpart.
    pub fn contains_column_of_type(
        &self,
        name: &str,
        type_name: &str,
    ) -> Result<bool, TabularDataError> {
        let name = to_cstring(name)?;
        let type_name = to_cstring(type_name)?;
        let mut contains = 0;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_contains_column_type(
                self.raw,
                name.as_ptr(),
                type_name.as_ptr(),
                &mut contains,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(contains != 0)
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.columnNamesForAlias` counterpart.
    pub fn column_names_for_alias(&self, alias: &str) -> Result<Vec<String>, TabularDataError> {
        let alias = to_cstring(alias)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::td_dataframe_column_names_for_alias_json(self.raw, alias.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }

    /// Wraps the `TabularData` `DataFrame.addAlias` counterpart.
    pub fn add_alias(&mut self, alias: &str, column_name: &str) -> Result<(), TabularDataError> {
        let alias = to_cstring(alias)?;
        let column_name = to_cstring(column_name)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_add_alias(self.raw, alias.as_ptr(), column_name.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.removeAlias` counterpart.
    pub fn remove_alias(&mut self, alias: &str) -> Result<(), TabularDataError> {
        let alias = to_cstring(alias)?;
        let mut error = core::ptr::null_mut();
        let status =
            unsafe { ffi::td_dataframe_remove_alias(self.raw, alias.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.appendColumn` counterpart.
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

    /// Wraps the `TabularData` `DataFrame.renameColumn` counterpart.
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

    /// Wraps the `TabularData` `DataFrame.column` counterpart.
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

    /// Wraps the `TabularData` `DataFrame.rowsJson` counterpart.
    pub fn rows_json(&self) -> Result<Vec<Value>, TabularDataError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_rows_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }

    /// Wraps the `TabularData` `DataFrame.summary` counterpart.
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

    /// Wraps the `TabularData` `DataFrame.joined` counterpart.
    pub fn joined(
        &self,
        other: &Self,
        column_name: &str,
        kind: JoinKind,
    ) -> Result<Self, TabularDataError> {
        self.joined_on(other, JoinColumns::same(column_name), kind)
    }

    /// Wraps the `TabularData` `DataFrame.writeCsv` counterpart.
    pub fn write_csv(
        &self,
        path: impl AsRef<Path>,
        options: &CSVWritingOptions,
    ) -> Result<(), TabularDataError> {
        let path = path_to_cstring(path)?;
        let options = encode_csv_write_options(options)?;
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

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw
    }

    pub(crate) fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) fn replace_with(&mut self, mut other: Self) {
        // SAFETY: We own self.raw and release it exactly once before replacing.
        // The FFI call handles the reference count decrement for a valid handle.
        unsafe { ffi::td_object_release(self.raw) };
        self.raw = other.raw;
        other.raw = core::ptr::null_mut();
    }
}

impl Drop for DataFrame {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: We own the handle and are about to destroy the DataFrame.
            // The FFI call handles the reference count decrement properly.
            unsafe { ffi::td_object_release(self.raw) };
        }
    }
}

impl From<CSVReadingOptions> for CSVReadingOptionsPayload {
    fn from(value: CSVReadingOptions) -> Self {
        Self {
            has_header_row: value.has_header_row,
            nil_encodings: value.nil_encodings,
            true_encodings: value.true_encodings,
            false_encodings: value.false_encodings,
            floating_point_type: value.floating_point_type,
            date_parse_strategies: value.date_parse_strategies,
            ignores_empty_lines: value.ignores_empty_lines,
            uses_quoting: value.uses_quoting,
            uses_escaping: value.uses_escaping,
            delimiter: value.delimiter.to_string(),
            escape_character: value.escape_character.to_string(),
        }
    }
}

impl From<CSVWritingOptions> for CSVWritingOptionsPayload {
    fn from(value: CSVWritingOptions) -> Self {
        Self {
            includes_header: value.includes_header,
            date_strategy: value.date_strategy,
            nil_encoding: value.nil_encoding,
            true_encoding: value.true_encoding,
            false_encoding: value.false_encoding,
            newline: value.newline,
            delimiter: value.delimiter.to_string(),
        }
    }
}

pub(crate) fn encode_csv_read_request(
    request: &CSVReadRequest,
) -> Result<CString, TabularDataError> {
    let payload = CSVReadRequestPayload {
        columns: request.columns.clone(),
        rows: request.rows.map(Into::into),
        types: request.types.clone(),
        options: request.options.clone().into(),
    };
    encode_json_cstring(&payload, "CSV read request")
}

pub(crate) fn encode_csv_write_options(
    options: &CSVWritingOptions,
) -> Result<CString, TabularDataError> {
    encode_json_cstring(
        &CSVWritingOptionsPayload::from(options.clone()),
        "CSV writing options",
    )
}

pub(crate) fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString, TabularDataError> {
    let path = path.as_ref().to_string_lossy().into_owned();
    to_cstring(&path)
}
