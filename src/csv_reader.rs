use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::dataframe::{encode_csv_read_request, path_to_cstring, CSVReadingOptions, DataFrame};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CSVType {
    Integer,
    Boolean,
    Float,
    Double,
    Date,
    String,
    Data,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DateParseStrategy {
    Iso8601,
    Rfc3339,
    Ymd,
    CustomFormat(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSVReadRequest {
    pub options: CSVReadingOptions,
    pub columns: Option<Vec<String>>,
    pub rows: Option<(usize, usize)>,
    pub types: BTreeMap<String, CSVType>,
}

impl CSVReadRequest {
    #[must_use]
    pub fn new(options: CSVReadingOptions) -> Self {
        Self {
            options,
            columns: None,
            rows: None,
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
    pub fn with_rows(mut self, rows: Range<usize>) -> Self {
        self.rows = Some((rows.start, rows.end));
        self
    }

    #[must_use]
    pub fn with_type_hint(mut self, column: impl Into<String>, column_type: CSVType) -> Self {
        self.types.insert(column.into(), column_type);
        self
    }
}

impl DataFrame {
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
}
