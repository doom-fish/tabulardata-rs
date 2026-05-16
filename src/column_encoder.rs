use serde::{Deserialize, Serialize};

use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnElementType {
    String,
    Int,
    Double,
    Bool,
    Date,
    Data,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnCodec {
    Json,
    PropertyList,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct ColumnCodingRequest<'a> {
    column: &'a str,
    element_type: ColumnElementType,
    codec: ColumnCodec,
}

impl DataFrame {
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
