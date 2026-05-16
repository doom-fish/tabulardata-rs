use serde::{Deserialize, Serialize};

use crate::any_column::AnyValue;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;
use crate::sort::SortOrder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    WeekOfYear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupValueType {
    String,
    Int,
    Double,
    Bool,
    Date,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct GroupBySpec {
    columns: Vec<String>,
    time_unit: Option<TimeUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum GroupAggregation {
    Counts {
        order: Option<SortOrder>,
    },
    Sum {
        column: String,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    },
    Mean {
        column: String,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    },
    Quantile {
        column: String,
        quantile: f64,
        order: Option<SortOrder>,
    },
    Minimum {
        column: String,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    },
    Maximum {
        column: String,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    },
}

pub struct GroupBy<'a> {
    frame: &'a DataFrame,
    spec: GroupBySpec,
}

impl DataFrame {
    #[must_use]
    pub fn group_by<S: AsRef<str>>(&self, columns: &[S]) -> GroupBy<'_> {
        GroupBy {
            frame: self,
            spec: GroupBySpec {
                columns: columns
                    .iter()
                    .map(|column| column.as_ref().to_string())
                    .collect(),
                time_unit: None,
            },
        }
    }

    #[must_use]
    pub fn group_by_time(&self, column: &str, time_unit: TimeUnit) -> GroupBy<'_> {
        GroupBy {
            frame: self,
            spec: GroupBySpec {
                columns: vec![column.into()],
                time_unit: Some(time_unit),
            },
        }
    }
}

impl GroupBy<'_> {
    pub fn counts(&self, order: Option<SortOrder>) -> Result<DataFrame, TabularDataError> {
        self.aggregate(&GroupAggregation::Counts { order })
    }

    pub fn sums(
        &self,
        column: &str,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    ) -> Result<DataFrame, TabularDataError> {
        self.aggregate(&GroupAggregation::Sum {
            column: column.into(),
            value_type,
            order,
        })
    }

    pub fn means(
        &self,
        column: &str,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    ) -> Result<DataFrame, TabularDataError> {
        self.aggregate(&GroupAggregation::Mean {
            column: column.into(),
            value_type,
            order,
        })
    }

    pub fn quantiles(
        &self,
        column: &str,
        quantile: f64,
        order: Option<SortOrder>,
    ) -> Result<DataFrame, TabularDataError> {
        self.aggregate(&GroupAggregation::Quantile {
            column: column.into(),
            quantile,
            order,
        })
    }

    pub fn minimums(
        &self,
        column: &str,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    ) -> Result<DataFrame, TabularDataError> {
        self.aggregate(&GroupAggregation::Minimum {
            column: column.into(),
            value_type,
            order,
        })
    }

    pub fn maximums(
        &self,
        column: &str,
        value_type: GroupValueType,
        order: Option<SortOrder>,
    ) -> Result<DataFrame, TabularDataError> {
        self.aggregate(&GroupAggregation::Maximum {
            column: column.into(),
            value_type,
            order,
        })
    }

    pub fn group(&self, keys: &[AnyValue]) -> Result<Option<DataFrame>, TabularDataError> {
        let group = encode_json_cstring(&self.spec, "grouping spec")?;
        let keys = encode_json_cstring(&keys, "group keys")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_group_slice_json(
                self.frame.as_raw(),
                group.as_ptr(),
                keys.as_ptr(),
                &mut raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            if raw.is_null() {
                Ok(None)
            } else {
                Ok(Some(DataFrame::from_raw(raw)))
            }
        } else {
            Err(from_swift(status, error))
        }
    }

    fn aggregate(&self, aggregate: &GroupAggregation) -> Result<DataFrame, TabularDataError> {
        let group = encode_json_cstring(&self.spec, "grouping spec")?;
        let aggregate = encode_json_cstring(aggregate, "group aggregate")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_group_aggregate_json(
                self.frame.as_raw(),
                group.as_ptr(),
                aggregate.as_ptr(),
                &mut raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(DataFrame::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }
}
