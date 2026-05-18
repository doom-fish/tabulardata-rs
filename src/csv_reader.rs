use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::dataframe::{encode_csv_read_request, path_to_cstring, CSVReadingOptions, DataFrame};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;

/// Wraps `CSV` type hints accepted by `TabularData` `CSV`-reading counterparts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CSVType {
    /// Wraps the `TabularData` `CSVType.integer` case.
    Integer,
    /// Wraps the `TabularData` `CSVType.boolean` case.
    Boolean,
    /// Wraps the `TabularData` `CSVType.float` case.
    Float,
    /// Wraps the `TabularData` `CSVType.double` case.
    Double,
    /// Wraps the `TabularData` `CSVType.date` case.
    Date,
    /// Wraps the `TabularData` `CSVType.string` case.
    String,
    /// Wraps the `TabularData` `CSVType.data` case.
    Data,
}

/// Wraps date-parsing strategies accepted by `TabularData` `CSV` and `JSON` reader counterparts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DateParseStrategy {
    /// Wraps the `TabularData` `DateParseStrategy.iso8601` case.
    Iso8601,
    /// Wraps the `TabularData` `DateParseStrategy.rfc3339` case.
    Rfc3339,
    /// Wraps the `TabularData` `DateParseStrategy.ymd` case.
    Ymd,
    /// Wraps the `TabularData` `DateParseStrategy.customFormat` case.
    CustomFormat(String),
}

/// Wraps the `TabularData` `CSV` read request counterpart used by `DataFrame(contentsOfCSVFile:)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSVReadRequest {
    /// Wraps the `TabularData` `CSVReadRequest.options` counterpart.
    pub options: CSVReadingOptions,
    /// Wraps the `TabularData` `CSVReadRequest.columns` counterpart.
    pub columns: Option<Vec<String>>,
    /// Wraps the `TabularData` `CSVReadRequest.rows` counterpart.
    pub rows: Option<(usize, usize)>,
    /// Wraps the `TabularData` `CSVReadRequest.types` counterpart.
    pub types: BTreeMap<String, CSVType>,
}

impl CSVReadRequest {
    /// Wraps the `TabularData` `CSVReadRequest.init` counterpart.
    #[must_use]
    pub fn new(options: CSVReadingOptions) -> Self {
        Self {
            options,
            columns: None,
            rows: None,
            types: BTreeMap::new(),
        }
    }

    /// Wraps the `TabularData` `CSVReadRequest.withColumns` counterpart.
    #[must_use]
    pub fn with_columns<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = Some(columns.into_iter().map(Into::into).collect());
        self
    }

    /// Wraps the `TabularData` `CSVReadRequest.withRows` counterpart.
    #[must_use]
    pub fn with_rows(mut self, rows: Range<usize>) -> Self {
        self.rows = Some((rows.start, rows.end));
        self
    }

    /// Wraps the `TabularData` `CSVReadRequest.withTypeHint` counterpart.
    #[must_use]
    pub fn with_type_hint(mut self, column: impl Into<String>, column_type: CSVType) -> Self {
        self.types.insert(column.into(), column_type);
        self
    }
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.readCsvWith` counterpart.
    pub fn read_csv_with(
        path: impl AsRef<Path>,
        request: &CSVReadRequest,
    ) -> Result<Self, TabularDataError> {
        let path = path_to_cstring(path)?;
        let request = encode_csv_read_request(request)?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_from_csv(path.as_ptr(), request.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.fromCsvData` counterpart.
    pub fn from_csv_data(
        data: &[u8],
        options: CSVReadingOptions,
    ) -> Result<Self, TabularDataError> {
        Self::read_csv_data_with(data, &CSVReadRequest::new(options))
    }

    /// Wraps the `TabularData` `DataFrame.readCsvDataWith` counterpart.
    pub fn read_csv_data_with(
        data: &[u8],
        request: &CSVReadRequest,
    ) -> Result<Self, TabularDataError> {
        let data = std::str::from_utf8(data).map_err(|_| {
            TabularDataError::InvalidArgument("CSV data must be valid UTF-8".into())
        })?;
        let data = crate::private::to_cstring(data)?;
        let request = encode_csv_read_request(request)?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_from_csv_data(data.as_ptr(), request.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }
}
