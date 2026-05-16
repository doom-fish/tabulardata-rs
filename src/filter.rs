use serde::{Deserialize, Serialize};

use crate::any_column::AnyValue;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Filter {
    Compare {
        column: String,
        op: ComparisonOp,
        value: AnyValue,
    },
    Between {
        column: String,
        lower: AnyValue,
        upper: AnyValue,
    },
    In {
        column: String,
        values: Vec<AnyValue>,
    },
    Contains {
        column: String,
        value: AnyValue,
    },
    IsNull {
        column: String,
    },
    IsNotNull {
        column: String,
    },
    And {
        filters: Vec<Self>,
    },
    Or {
        filters: Vec<Self>,
    },
    Not {
        filter: Box<Self>,
    },
}

impl Filter {
    #[must_use]
    pub fn eq(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Eq,
            value: value.into(),
        }
    }

    #[must_use]
    pub fn ne(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Ne,
            value: value.into(),
        }
    }

    #[must_use]
    pub fn gt(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Gt,
            value: value.into(),
        }
    }

    #[must_use]
    pub fn gte(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Gte,
            value: value.into(),
        }
    }

    #[must_use]
    pub fn lt(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Lt,
            value: value.into(),
        }
    }

    #[must_use]
    pub fn lte(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Lte,
            value: value.into(),
        }
    }

    #[must_use]
    pub fn between(
        column: impl Into<String>,
        lower: impl Into<AnyValue>,
        upper: impl Into<AnyValue>,
    ) -> Self {
        Self::Between {
            column: column.into(),
            lower: lower.into(),
            upper: upper.into(),
        }
    }

    #[must_use]
    pub fn one_of(column: impl Into<String>, values: Vec<AnyValue>) -> Self {
        Self::In {
            column: column.into(),
            values,
        }
    }

    #[must_use]
    pub fn contains(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Contains {
            column: column.into(),
            value: value.into(),
        }
    }

    #[must_use]
    pub fn is_null(column: impl Into<String>) -> Self {
        Self::IsNull {
            column: column.into(),
        }
    }

    #[must_use]
    pub fn is_not_null(column: impl Into<String>) -> Self {
        Self::IsNotNull {
            column: column.into(),
        }
    }

    #[must_use]
    pub fn and(filters: Vec<Self>) -> Self {
        Self::And { filters }
    }

    #[must_use]
    pub fn or(filters: Vec<Self>) -> Self {
        Self::Or { filters }
    }

    #[must_use]
    pub fn negate(filter: Self) -> Self {
        Self::Not {
            filter: Box::new(filter),
        }
    }
}

impl DataFrame {
    pub fn filtered(&self, filter: &Filter) -> Result<Self, TabularDataError> {
        let filter = encode_json_cstring(filter, "filter payload")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_filter_json(self.as_raw(), filter.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }
}
