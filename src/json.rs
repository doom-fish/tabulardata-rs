use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::csv_reader::DateParseStrategy;
use crate::csv_writer::DateWriteStrategy;
use crate::dataframe::{path_to_cstring, DataFrame};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::{decode_json, encode_json_cstring, to_cstring};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JSONType {
    Integer,
    Boolean,
    Double,
    Date,
    String,
    Array,
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JSONReadingOptions {
    pub date_parse_strategies: Vec<DateParseStrategy>,
}

impl JSONReadingOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_date_parse_strategy(mut self, strategy: DateParseStrategy) -> Self {
        self.date_parse_strategies.push(strategy);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONReadRequest {
    pub options: JSONReadingOptions,
    pub columns: Option<Vec<String>>,
    pub types: BTreeMap<String, JSONType>,
}

impl JSONReadRequest {
    #[must_use]
    pub fn new(options: JSONReadingOptions) -> Self {
        Self {
            options,
            columns: None,
            types: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_columns<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = Some(columns.into_iter().map(Into::into).collect());
        self
    }

    #[must_use]
    pub fn with_type_hint(mut self, column: impl Into<String>, column_type: JSONType) -> Self {
        self.types.insert(column.into(), column_type);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JSONWritingOptions {
    pub sort_keys: bool,
    pub pretty_print: bool,
    pub date_strategy: Option<DateWriteStrategy>,
}

impl JSONWritingOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_sort_keys(mut self, sort_keys: bool) -> Self {
        self.sort_keys = sort_keys;
        self
    }

    #[must_use]
    pub fn with_pretty_print(mut self, pretty_print: bool) -> Self {
        self.pretty_print = pretty_print;
        self
    }

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
    pub fn from_json(
        path: impl AsRef<Path>,
        options: JSONReadingOptions,
    ) -> Result<Self, TabularDataError> {
        Self::read_json_with(path, &JSONReadRequest::new(options))
    }

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

    pub fn from_json_data(
        data: &[u8],
        options: JSONReadingOptions,
    ) -> Result<Self, TabularDataError> {
        Self::read_json_data_with(data, &JSONReadRequest::new(options))
    }

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

    pub fn json_string(&self, options: &JSONWritingOptions) -> Result<String, TabularDataError> {
        let bytes = self.json_bytes(options)?;
        String::from_utf8(bytes).map_err(|error| {
            TabularDataError::FrameworkError(format!("JSON data was not valid UTF-8: {error}"))
        })
    }
}
