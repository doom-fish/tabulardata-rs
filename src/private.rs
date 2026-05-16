use std::ffi::CString;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{take_owned_c_string, TabularDataError};

pub fn to_cstring(value: &str) -> Result<CString, TabularDataError> {
    CString::new(value).map_err(|_| {
        TabularDataError::InvalidArgument("strings must not contain interior NUL bytes".into())
    })
}

pub fn decode_json<T: DeserializeOwned>(
    ptr: *mut core::ffi::c_char,
) -> Result<T, TabularDataError> {
    let json = take_owned_c_string(ptr);
    serde_json::from_str(&json).map_err(|error| {
        TabularDataError::FrameworkError(format!("failed to decode bridge JSON payload: {error}"))
    })
}

pub fn encode_json_cstring<T: Serialize>(
    value: &T,
    label: &str,
) -> Result<CString, TabularDataError> {
    let json = serde_json::to_string(value).map_err(|error| {
        TabularDataError::FrameworkError(format!("failed to encode {label}: {error}"))
    })?;
    to_cstring(&json)
}
