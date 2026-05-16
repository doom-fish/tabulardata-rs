use serde::{Deserialize, Serialize};

use crate::dataframe::{DataFrame, JoinKind};
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinColumns {
    pub left: String,
    pub right: String,
}

impl JoinColumns {
    #[must_use]
    pub fn new(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }

    #[must_use]
    pub fn same(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            left: name.clone(),
            right: name,
        }
    }
}

impl From<&str> for JoinColumns {
    fn from(value: &str) -> Self {
        Self::same(value)
    }
}

impl From<String> for JoinColumns {
    fn from(value: String) -> Self {
        Self::same(value)
    }
}

#[derive(Debug, Clone, Serialize)]
struct JoinRequest {
    columns: JoinColumns,
    kind: JoinKind,
}

impl DataFrame {
    pub fn joined_on(
        &self,
        other: &Self,
        columns: impl Into<JoinColumns>,
        kind: JoinKind,
    ) -> Result<Self, TabularDataError> {
        let request = encode_json_cstring(
            &JoinRequest {
                columns: columns.into(),
                kind,
            },
            "join request",
        )?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_join_json(
                self.as_raw(),
                other.as_raw(),
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
