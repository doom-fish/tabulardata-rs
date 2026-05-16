use serde::Serialize;

use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::{decode_json, encode_json_cstring};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FormattingOptions {
    pub maximum_line_width: Option<usize>,
    pub maximum_cell_width: Option<usize>,
    pub maximum_row_count: Option<usize>,
    pub includes_column_types: Option<bool>,
    pub includes_row_indices: Option<bool>,
    pub includes_row_and_column_counts: Option<bool>,
    pub locale: Option<String>,
}

impl FormattingOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_maximum_line_width(mut self, width: usize) -> Self {
        self.maximum_line_width = Some(width);
        self
    }

    #[must_use]
    pub fn with_maximum_cell_width(mut self, width: usize) -> Self {
        self.maximum_cell_width = Some(width);
        self
    }

    #[must_use]
    pub fn with_maximum_row_count(mut self, count: usize) -> Self {
        self.maximum_row_count = Some(count);
        self
    }

    #[must_use]
    pub fn with_includes_column_types(mut self, includes: bool) -> Self {
        self.includes_column_types = Some(includes);
        self
    }

    #[must_use]
    pub fn with_includes_row_indices(mut self, includes: bool) -> Self {
        self.includes_row_indices = Some(includes);
        self
    }

    #[must_use]
    pub fn with_includes_row_and_column_counts(mut self, includes: bool) -> Self {
        self.includes_row_and_column_counts = Some(includes);
        self
    }

    #[must_use]
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
}

#[derive(Debug, Clone, Serialize)]
struct FormattingOptionsPayload {
    maximum_line_width: Option<usize>,
    maximum_cell_width: Option<usize>,
    maximum_row_count: Option<usize>,
    includes_column_types: Option<bool>,
    includes_row_indices: Option<bool>,
    includes_row_and_column_counts: Option<bool>,
    locale: Option<String>,
}

fn encode_formatting_options(
    options: &FormattingOptions,
) -> Result<std::ffi::CString, TabularDataError> {
    encode_json_cstring(
        &FormattingOptionsPayload {
            maximum_line_width: options.maximum_line_width,
            maximum_cell_width: options.maximum_cell_width,
            maximum_row_count: options.maximum_row_count,
            includes_column_types: options.includes_column_types,
            includes_row_indices: options.includes_row_indices,
            includes_row_and_column_counts: options.includes_row_and_column_counts,
            locale: options.locale.clone(),
        },
        "formatting options",
    )
}

impl DataFrame {
    pub fn description(&self) -> Result<String, TabularDataError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::td_dataframe_description(self.as_raw(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }

    pub fn format(&self, options: &FormattingOptions) -> Result<String, TabularDataError> {
        let options = encode_formatting_options(options)?;
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::td_dataframe_format_json(self.as_raw(), options.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json(payload)
        }
    }
}
