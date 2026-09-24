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

const NAN: &str = "NaN";
const INFINITY: &str = "Infinity";
const NEG_INFINITY: &str = "-Infinity";

fn non_finite_name(value: f64) -> &'static str {
    if value.is_nan() {
        NAN
    } else if value.is_sign_positive() {
        INFINITY
    } else {
        NEG_INFINITY
    }
}

fn non_finite_value(text: &str) -> Option<f64> {
    match text {
        NAN => Some(f64::NAN),
        INFINITY => Some(f64::INFINITY),
        NEG_INFINITY => Some(f64::NEG_INFINITY),
        _ => None,
    }
}

pub fn float_to_json(value: f64) -> serde_json::Value {
    serde_json::Number::from_f64(value).map_or_else(
        || serde_json::Value::String(non_finite_name(value).to_owned()),
        serde_json::Value::Number,
    )
}

pub fn float_from_json(value: &serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::Number(number) => number.as_f64(),
        serde_json::Value::String(text) => non_finite_value(text),
        _ => None,
    }
}

pub mod json_float {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
        if value.is_finite() {
            serializer.serialize_f64(*value)
        } else {
            serializer.serialize_str(super::non_finite_name(*value))
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        super::float_from_json(&value)
            .ok_or_else(|| D::Error::custom(format!("expected a number, NaN or Infinity, got {value}")))
    }
}
