use std::path::Path;

use serde::Serialize;

use crate::dataframe::{path_to_cstring, DataFrame};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SFrameReadRequest {
    pub columns: Option<Vec<String>>,
    pub rows: Option<std::ops::Range<usize>>,
}

impl SFrameReadRequest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
    pub fn with_rows(mut self, rows: std::ops::Range<usize>) -> Self {
        self.rows = Some(rows);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SFrameReadingError {
    MissingArchive,
    BadArchive(String),
    UnsupportedArchive(String),
    UnsupportedType(i64),
    UnsupportedLayout(String),
    BadEncoding(String),
    MissingColumn(String),
    Message(String),
}

impl SFrameReadingError {
    #[must_use]
    pub fn parse(message: &str) -> Self {
        let message = message.trim();
        if message.contains("missingArchive") || message.contains("missing archive") {
            Self::MissingArchive
        } else if let Some(value) = parse_wrapped(message, "badArchive(", ")") {
            Self::BadArchive(value)
        } else if let Some(value) = parse_wrapped(message, "unsupportedArchive(", ")") {
            Self::UnsupportedArchive(value)
        } else if let Some(value) = parse_wrapped(message, "unsupportedType(", ")") {
            Self::UnsupportedType(value.parse().unwrap_or_default())
        } else if let Some(value) = parse_wrapped(message, "unsupportedLayout(", ")") {
            Self::UnsupportedLayout(value)
        } else if let Some(value) = parse_wrapped(message, "badEncoding(", ")") {
            Self::BadEncoding(value)
        } else if let Some(value) = parse_wrapped(message, "missingColumn(", ")") {
            Self::MissingColumn(value)
        } else {
            Self::Message(message.to_string())
        }
    }
}

impl std::fmt::Display for SFrameReadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingArchive => f.write_str("missingArchive"),
            Self::BadArchive(value) => write!(f, "badArchive({value})"),
            Self::UnsupportedArchive(value) => write!(f, "unsupportedArchive({value})"),
            Self::UnsupportedType(value) => write!(f, "unsupportedType({value})"),
            Self::UnsupportedLayout(value) => write!(f, "unsupportedLayout({value})"),
            Self::BadEncoding(value) => write!(f, "badEncoding({value})"),
            Self::MissingColumn(value) => write!(f, "missingColumn({value})"),
            Self::Message(value) => f.write_str(value),
        }
    }
}

impl std::error::Error for SFrameReadingError {}

#[derive(Debug, Clone, Serialize)]
struct SFrameReadRequestPayload {
    columns: Option<Vec<String>>,
    rows: Option<[usize; 2]>,
}

impl DataFrame {
    pub fn from_sframe(path: impl AsRef<Path>) -> Result<Self, TabularDataError> {
        Self::read_sframe_with(path, &SFrameReadRequest::new())
    }

    pub fn read_sframe_with(
        path: impl AsRef<Path>,
        request: &SFrameReadRequest,
    ) -> Result<Self, TabularDataError> {
        let path = path_to_cstring(path)?;
        let request = encode_json_cstring(
            &SFrameReadRequestPayload {
                columns: request.columns.clone(),
                rows: request.rows.clone().map(|rows| [rows.start, rows.end]),
            },
            "SFrame read request",
        )?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_from_sframe_directory(
                path.as_ptr(),
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
}

fn parse_wrapped(message: &str, prefix: &str, suffix: &str) -> Option<String> {
    let start = message.find(prefix)? + prefix.len();
    let end = message.rfind(suffix)?;
    (start <= end).then(|| message[start..end].trim_matches('"').to_string())
}
