use serde::{Deserialize, Serialize};

use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

/// Wraps element-type hints used by `TabularData` `ColumnEncoder` counterparts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnElementType {
    /// Wraps the `TabularData` `ColumnElementType.string` case.
    String,
    /// Wraps the `TabularData` `ColumnElementType.int` case.
    Int,
    /// Wraps the `TabularData` `ColumnElementType.double` case.
    Double,
    /// Wraps the `TabularData` `ColumnElementType.bool` case.
    Bool,
    /// Wraps the `TabularData` `ColumnElementType.date` case.
    Date,
    /// Wraps the `TabularData` `ColumnElementType.data` case.
    Data,
}

/// Wraps codec values used by `TabularData` `ColumnEncoder` counterparts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnCodec {
    /// Wraps the `TabularData` `ColumnCodec.json` case.
    Json,
    /// Wraps the `TabularData` `ColumnCodec.propertyList` case.
    PropertyList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct ColumnCodingRequest<'a> {
    column: &'a str,
    element_type: ColumnElementType,
    codec: ColumnCodec,
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.encodeColumn` counterpart.
    pub fn encode_column(
        &mut self,
        column: &str,
        element_type: ColumnElementType,
        codec: ColumnCodec,
    ) -> Result<(), TabularDataError> {
        let request = encode_json_cstring(
            &ColumnCodingRequest {
                column,
                element_type,
                codec,
            },
            "column encoding request",
        )?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_encode_column_json(self.as_raw(), request.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.decodeColumn` counterpart.
    pub fn decode_column(
        &mut self,
        column: &str,
        element_type: ColumnElementType,
        codec: ColumnCodec,
    ) -> Result<(), TabularDataError> {
        let request = encode_json_cstring(
            &ColumnCodingRequest {
                column,
                element_type,
                codec,
            },
            "column decoding request",
        )?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_decode_column_json(self.as_raw(), request.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }
}
