use serde::Serialize;

use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

#[derive(Debug, Clone, Serialize)]
struct RandomSplitPayload {
    proportion: f64,
    seed: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
struct StratifiedSplitPayload {
    columns: Vec<String>,
    proportion: f64,
    random_seed: Option<i64>,
}

impl DataFrame {
    pub fn random_split(
        &self,
        proportion: f64,
        seed: Option<i64>,
    ) -> Result<(Self, Self), TabularDataError> {
        let payload =
            encode_json_cstring(&RandomSplitPayload { proportion, seed }, "random split")?;
        let mut left = core::ptr::null_mut();
        let mut right = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_random_split(
                self.as_raw(),
                payload.as_ptr(),
                &mut left,
                &mut right,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok((Self::from_raw(left), Self::from_raw(right)))
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn stratified_split<S: AsRef<str>>(
        &self,
        columns: &[S],
        proportion: f64,
        random_seed: Option<i64>,
    ) -> Result<(Self, Self), TabularDataError> {
        let payload = encode_json_cstring(
            &StratifiedSplitPayload {
                columns: columns
                    .iter()
                    .map(|column| column.as_ref().to_string())
                    .collect(),
                proportion,
                random_seed,
            },
            "stratified split",
        )?;
        let mut left = core::ptr::null_mut();
        let mut right = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_stratified_split_json(
                self.as_raw(),
                payload.as_ptr(),
                &mut left,
                &mut right,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok((Self::from_raw(left), Self::from_raw(right)))
        } else {
            Err(from_swift(status, error))
        }
    }
}
