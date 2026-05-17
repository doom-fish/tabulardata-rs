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

pub struct GroupSummaryEntry {
    pub keys: Vec<AnyValue>,
    pub summary: DataFrame,
}

pub struct GroupSummaries {
    grouping_columns: Vec<String>,
    entries: Vec<GroupSummaryEntry>,
}

impl GroupSummaries {
    #[must_use]
    pub fn grouping_columns(&self) -> &[String] {
        &self.grouping_columns
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[must_use]
    pub fn entries(&self) -> &[GroupSummaryEntry] {
        &self.entries
    }

    #[must_use]
    pub fn group(&self, keys: &[AnyValue]) -> Option<&DataFrame> {
        self.entries
            .iter()
            .find(|entry| entry.keys == keys)
            .map(|entry| &entry.summary)
    }

    #[must_use]
    pub fn description(&self) -> String {
        format!(
            "GroupSummaries(groups={}, columns={:?})",
            self.entries.len(),
            self.grouping_columns
        )
    }

    pub fn format(
        &self,
        options: &crate::formatting::FormattingOptions,
    ) -> Result<String, TabularDataError> {
        let mut rendered = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            let summary = entry.summary.format(options)?;
            rendered.push(format!("keys={:?}\n{}", entry.keys, summary));
        }
        Ok(rendered.join("\n\n"))
    }
}

impl std::fmt::Display for GroupSummaries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description())
    }
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

    pub fn group_count(&self) -> Result<usize, TabularDataError> {
        Ok(self.counts(None)?.row_count())
    }

    pub fn ungrouped(&self) -> Result<DataFrame, TabularDataError> {
        self.frame.try_clone()
    }

    pub fn filter_groups<F>(&self, mut predicate: F) -> Result<DataFrame, TabularDataError>
    where
        F: FnMut(&DataFrame) -> Result<bool, TabularDataError>,
    {
        let mut filtered = self.frame.slice_rows(0..0)?;
        for (_, group) in self.materialized_groups()? {
            if predicate(&group)? {
                filtered.append_rows_of(&group)?;
            }
        }
        Ok(filtered)
    }

    pub fn map_groups<F>(&self, mut transform: F) -> Result<DataFrame, TabularDataError>
    where
        F: FnMut(&DataFrame) -> Result<DataFrame, TabularDataError>,
    {
        let mut mapped: Option<DataFrame> = None;
        for (_, group) in self.materialized_groups()? {
            let transformed = transform(&group)?;
            match &mut mapped {
                Some(frame) => frame.append_rows_of(&transformed)?,
                None => mapped = Some(transformed),
            }
        }
        mapped.map_or_else(|| self.frame.slice_rows(0..0), Ok)
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    pub fn random_split(
        &self,
        proportion: f64,
        seed: Option<u64>,
    ) -> Result<(DataFrame, DataFrame), TabularDataError> {
        if !(0.0..=1.0).contains(&proportion) {
            return Err(TabularDataError::InvalidArgument(
                "split proportion must be between 0 and 1".into(),
            ));
        }
        let mut groups = self.materialized_groups()?;
        shuffle_groups(&mut groups, seed.unwrap_or(0x9E37_79B9_7F4A_7C15));
        let split_at = ((groups.len() as f64) * proportion).round() as usize;
        let (left_groups, right_groups) = groups.split_at(split_at.min(groups.len()));
        let mut left = self.frame.slice_rows(0..0)?;
        let mut right = self.frame.slice_rows(0..0)?;
        for (_, group) in left_groups {
            left.append_rows_of(group)?;
        }
        for (_, group) in right_groups {
            right.append_rows_of(group)?;
        }
        Ok((left, right))
    }

    pub fn summary(&self) -> Result<GroupSummaries, TabularDataError> {
        self.build_summaries(None)
    }

    pub fn summary_of<S: AsRef<str>>(
        &self,
        columns: &[S],
    ) -> Result<GroupSummaries, TabularDataError> {
        let columns: Vec<String> = columns
            .iter()
            .map(|column| column.as_ref().to_string())
            .collect();
        self.build_summaries(Some(&columns))
    }

    fn build_summaries(
        &self,
        columns: Option<&[String]>,
    ) -> Result<GroupSummaries, TabularDataError> {
        let mut entries = Vec::new();
        for (keys, group) in self.materialized_groups()? {
            let summary = if let Some(columns) = columns {
                group.summary_columns(columns)?
            } else {
                group.summary_frame()?
            };
            entries.push(GroupSummaryEntry { keys, summary });
        }
        Ok(GroupSummaries {
            grouping_columns: self.spec.columns.clone(),
            entries,
        })
    }

    fn materialized_groups(&self) -> Result<Vec<(Vec<AnyValue>, DataFrame)>, TabularDataError> {
        let counts = self.counts(None)?;
        let mut groups = Vec::new();
        for row in counts.rows()? {
            let keys: Vec<AnyValue> = self
                .spec
                .columns
                .iter()
                .map(|column| row.get(column).cloned().unwrap_or_default())
                .collect();
            if let Some(group) = self.group(&keys)? {
                groups.push((keys, group));
            }
        }
        Ok(groups)
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

fn shuffle_groups(groups: &mut [(Vec<AnyValue>, DataFrame)], seed: u64) {
    if groups.len() < 2 {
        return;
    }
    let mut state = seed.max(1);
    for index in (1..groups.len()).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let modulus = u64::try_from(index + 1).unwrap_or(u64::MAX);
        let swap_with = usize::try_from(state % modulus).unwrap_or(0);
        groups.swap(index, swap_with);
    }
}
