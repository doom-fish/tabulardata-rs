use serde::{Deserialize, Serialize};

use crate::any_column::AnyValue;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

/// Wraps comparison operators accepted by `TabularData` `Filter` counterparts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOp {
    /// Wraps the `TabularData` `ComparisonOp.eq` case.
    Eq,
    /// Wraps the `TabularData` `ComparisonOp.ne` case.
    Ne,
    /// Wraps the `TabularData` `ComparisonOp.lt` case.
    Lt,
    /// Wraps the `TabularData` `ComparisonOp.lte` case.
    Lte,
    /// Wraps the `TabularData` `ComparisonOp.gt` case.
    Gt,
    /// Wraps the `TabularData` `ComparisonOp.gte` case.
    Gte,
}

/// Wraps `TabularData` `Filter` counterparts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Filter {
    /// Wraps the `TabularData` `Filter.compare` case.
    Compare {
        column: String,
        op: ComparisonOp,
        value: AnyValue,
    },
    /// Wraps the `TabularData` `Filter.between` case.
    Between {
        column: String,
        lower: AnyValue,
        upper: AnyValue,
    },
    /// Wraps the `TabularData` `Filter.in` case.
    In {
        column: String,
        values: Vec<AnyValue>,
    },
    /// Wraps the `TabularData` `Filter.contains` case.
    Contains { column: String, value: AnyValue },
    /// Wraps the `TabularData` `Filter.isNull` case.
    IsNull { column: String },
    /// Wraps the `TabularData` `Filter.isNotNull` case.
    IsNotNull { column: String },
    /// Wraps the `TabularData` `Filter.and` case.
    And { filters: Vec<Self> },
    /// Wraps the `TabularData` `Filter.or` case.
    Or { filters: Vec<Self> },
    /// Wraps the `TabularData` `Filter.not` case.
    Not { filter: Box<Self> },
}

impl Filter {
    /// Wraps the `TabularData` `Filter.eq` counterpart.
    #[must_use]
    pub fn eq(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Eq,
            value: value.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.ne` counterpart.
    #[must_use]
    pub fn ne(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Ne,
            value: value.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.gt` counterpart.
    #[must_use]
    pub fn gt(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Gt,
            value: value.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.gte` counterpart.
    #[must_use]
    pub fn gte(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Gte,
            value: value.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.lt` counterpart.
    #[must_use]
    pub fn lt(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Lt,
            value: value.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.lte` counterpart.
    #[must_use]
    pub fn lte(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Compare {
            column: column.into(),
            op: ComparisonOp::Lte,
            value: value.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.between` counterpart.
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

    /// Wraps the `TabularData` `Filter.oneOf` counterpart.
    #[must_use]
    pub fn one_of(column: impl Into<String>, values: Vec<AnyValue>) -> Self {
        Self::In {
            column: column.into(),
            values,
        }
    }

    /// Wraps the `TabularData` `Filter.contains` counterpart.
    #[must_use]
    pub fn contains(column: impl Into<String>, value: impl Into<AnyValue>) -> Self {
        Self::Contains {
            column: column.into(),
            value: value.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.isNull` counterpart.
    #[must_use]
    pub fn is_null(column: impl Into<String>) -> Self {
        Self::IsNull {
            column: column.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.isNotNull` counterpart.
    #[must_use]
    pub fn is_not_null(column: impl Into<String>) -> Self {
        Self::IsNotNull {
            column: column.into(),
        }
    }

    /// Wraps the `TabularData` `Filter.and` counterpart.
    #[must_use]
    pub fn and(filters: Vec<Self>) -> Self {
        Self::And { filters }
    }

    /// Wraps the `TabularData` `Filter.or` counterpart.
    #[must_use]
    pub fn or(filters: Vec<Self>) -> Self {
        Self::Or { filters }
    }

    /// Wraps the `TabularData` `Filter.negate` counterpart.
    #[must_use]
    pub fn negate(filter: Self) -> Self {
        Self::Not {
            filter: Box::new(filter),
        }
    }
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.filtered` counterpart.
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
