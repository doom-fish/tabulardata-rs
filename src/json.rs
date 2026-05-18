use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::csv_reader::DateParseStrategy;
use crate::csv_writer::DateWriteStrategy;
use crate::dataframe::{path_to_cstring, DataFrame};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::{decode_json, encode_json_cstring, to_cstring};

/// Wraps `JSON`-reading errors surfaced by `TabularData` counterparts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONReadingError {
    /// Wraps the `TabularData` `JSONReadingError.message` counterpart.
    pub message: String,
}

impl JSONReadingError {
    /// Wraps the `TabularData` `JSONReadingError.init` counterpart.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Wraps the `TabularData` `JSONReadingError.fromError` counterpart.
    #[must_use]
    pub fn from_error(error: &TabularDataError) -> Self {
        Self::new(error.message())
    }
}

impl std::fmt::Display for JSONReadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for JSONReadingError {}

/// Wraps `JSON` type hints accepted by `TabularData` `JSON`-reading counterparts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JSONType {
    /// Wraps the `TabularData` `JSONType.integer` case.
    Integer,
    /// Wraps the `TabularData` `JSONType.boolean` case.
    Boolean,
    /// Wraps the `TabularData` `JSONType.double` case.
    Double,
    /// Wraps the `TabularData` `JSONType.date` case.
    Date,
    /// Wraps the `TabularData` `JSONType.string` case.
    String,
    /// Wraps the `TabularData` `JSONType.array` case.
    Array,
    /// Wraps the `TabularData` `JSONType.object` case.
    Object,
}

/// Wraps the `TabularData` `JSON`-reading options counterpart.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JSONReadingOptions {
    /// Wraps the `TabularData` `JSONReadingOptions.dateParseStrategies` counterpart.
    pub date_parse_strategies: Vec<DateParseStrategy>,
}

impl JSONReadingOptions {
    /// Wraps the `TabularData` `JSONReadingOptions.init` counterpart.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Wraps the `TabularData` `JSONReadingOptions.withDateParseStrategy` counterpart.
    #[must_use]
    pub fn with_date_parse_strategy(mut self, strategy: DateParseStrategy) -> Self {
        self.date_parse_strategies.push(strategy);
        self
    }
}

/// Wraps the `TabularData` `JSON` read request counterpart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONReadRequest {
    /// Wraps the `TabularData` `JSONReadRequest.options` counterpart.
    pub options: JSONReadingOptions,
    /// Wraps the `TabularData` `JSONReadRequest.columns` counterpart.
    pub columns: Option<Vec<String>>,
    /// Wraps the `TabularData` `JSONReadRequest.types` counterpart.
    pub types: BTreeMap<String, JSONType>,
}

impl JSONReadRequest {
    /// Wraps the `TabularData` `JSONReadRequest.init` counterpart.
    #[must_use]
    pub fn new(options: JSONReadingOptions) -> Self {
        Self {
            options,
            columns: None,
            types: BTreeMap::new(),
        }
    }

    /// Wraps the `TabularData` `JSONReadRequest.withColumns` counterpart.
    #[must_use]
    pub fn with_columns<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = Some(columns.into_iter().map(Into::into).collect());
        self
    }

    /// Wraps the `TabularData` `JSONReadRequest.withTypeHint` counterpart.
    #[must_use]
    pub fn with_type_hint(mut self, column: impl Into<String>, column_type: JSONType) -> Self {
        self.types.insert(column.into(), column_type);
        self
    }
}

/// Wraps the `TabularData` `JSON`-writing options counterpart.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JSONWritingOptions {
    /// Wraps the `TabularData` `JSONWritingOptions.sortKeys` counterpart.
    pub sort_keys: bool,
    /// Wraps the `TabularData` `JSONWritingOptions.prettyPrint` counterpart.
    pub pretty_print: bool,
    /// Wraps the `TabularData` `JSONWritingOptions.dateStrategy` counterpart.
    pub date_strategy: Option<DateWriteStrategy>,
}

impl JSONWritingOptions {
    /// Wraps the `TabularData` `JSONWritingOptions.init` counterpart.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Wraps the `TabularData` `JSONWritingOptions.withSortKeys` counterpart.
    #[must_use]
    pub fn with_sort_keys(mut self, sort_keys: bool) -> Self {
        self.sort_keys = sort_keys;
        self
    }

    /// Wraps the `TabularData` `JSONWritingOptions.withPrettyPrint` counterpart.
    #[must_use]
    pub fn with_pretty_print(mut self, pretty_print: bool) -> Self {
        self.pretty_print = pretty_print;
        self
    }

    /// Wraps the `TabularData` `JSONWritingOptions.withDateStrategy` counterpart.
    #[must_use]
    pub fn with_date_strategy(mut self, date_strategy: DateWriteStrategy) -> Self {
        self.date_strategy = Some(date_strategy);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
struct JSONReadingOptionsPayload {
    date_parse_strategies: Vec<DateParseStrategy>,
}

#[derive(Debug, Clone, Serialize)]
struct JSONReadRequestPayload {
    columns: Option<Vec<String>>,
    types: BTreeMap<String, JSONType>,
    options: JSONReadingOptionsPayload,
}

#[derive(Debug, Clone, Serialize)]
struct JSONWritingOptionsPayload {
    sort_keys: bool,
    pretty_print: bool,
    date_strategy: Option<DateWriteStrategy>,
}

impl From<JSONReadingOptions> for JSONReadingOptionsPayload {
    fn from(value: JSONReadingOptions) -> Self {
        Self {
            date_parse_strategies: value.date_parse_strategies,
        }
    }
}

fn encode_json_read_request(
    request: &JSONReadRequest,
) -> Result<std::ffi::CString, TabularDataError> {
    encode_json_cstring(
        &JSONReadRequestPayload {
            columns: request.columns.clone(),
            types: request.types.clone(),
            options: request.options.clone().into(),
        },
        "JSON read request",
    )
}

fn encode_json_write_options(
    options: &JSONWritingOptions,
) -> Result<std::ffi::CString, TabularDataError> {
    encode_json_cstring(
        &JSONWritingOptionsPayload {
            sort_keys: options.sort_keys,
            pretty_print: options.pretty_print,
            date_strategy: options.date_strategy.clone(),
        },
        "JSON writing options",
    )
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.fromJson` counterpart.
    pub fn from_json(
        path: impl AsRef<Path>,
        options: JSONReadingOptions,
    ) -> Result<Self, TabularDataError> {
        Self::read_json_with(path, &JSONReadRequest::new(options))
    }

    /// Wraps the `TabularData` `DataFrame.fromJsonString` counterpart.
    pub fn from_json_string(
        json: &str,
        options: JSONReadingOptions,
    ) -> Result<Self, TabularDataError> {
        Self::read_json_string_with(json, &JSONReadRequest::new(options))
    }

    /// Wraps the `TabularData` `DataFrame.readJsonWith` counterpart.
    pub fn read_json_with(
        path: impl AsRef<Path>,
        request: &JSONReadRequest,
    ) -> Result<Self, TabularDataError> {
        let path = path_to_cstring(path)?;
        let request = encode_json_read_request(request)?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_from_json_file(path.as_ptr(), request.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.fromJsonData` counterpart.
    pub fn from_json_data(
        data: &[u8],
        options: JSONReadingOptions,
    ) -> Result<Self, TabularDataError> {
        Self::read_json_data_with(data, &JSONReadRequest::new(options))
    }

    /// Wraps the `TabularData` `DataFrame.readJsonStringWith` counterpart.
    pub fn read_json_string_with(
        json: &str,
        request: &JSONReadRequest,
    ) -> Result<Self, TabularDataError> {
        Self::read_json_data_with(json.as_bytes(), request)
    }

    /// Wraps the `TabularData` `DataFrame.readJsonDataWith` counterpart.
    pub fn read_json_data_with(
        data: &[u8],
        request: &JSONReadRequest,
    ) -> Result<Self, TabularDataError> {
        let json_data = std::str::from_utf8(data).map_err(|_| {
            TabularDataError::InvalidArgument("JSON data must be valid UTF-8".into())
        })?;
        let json_data = to_cstring(json_data)?;
        let request = encode_json_read_request(request)?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_from_json_data(
                json_data.as_ptr(),
                request.as_ptr(),
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

    /// Wraps the `TabularData` `DataFrame.writeJson` counterpart.
    pub fn write_json(
        &self,
        path: impl AsRef<Path>,
        options: &JSONWritingOptions,
    ) -> Result<(), TabularDataError> {
        let path = path_to_cstring(path)?;
        let options = encode_json_write_options(options)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_write_json(self.as_raw(), path.as_ptr(), options.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.jsonBytes` counterpart.
    pub fn json_bytes(&self, options: &JSONWritingOptions) -> Result<Vec<u8>, TabularDataError> {
        let options = encode_json_write_options(options)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::td_dataframe_json_data_json(self.as_raw(), options.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }

    /// Wraps the `TabularData` `DataFrame.jsonString` counterpart.
    pub fn json_string(&self, options: &JSONWritingOptions) -> Result<String, TabularDataError> {
        let bytes = self.json_bytes(options)?;
        String::from_utf8(bytes).map_err(|error| {
            TabularDataError::FrameworkError(format!("JSON data was not valid UTF-8: {error}"))
        })
    }
}
