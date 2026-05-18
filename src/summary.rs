use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::any_column::AnyValue;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::encode_json_cstring;

/// Wraps numeric summaries produced by `TabularData` `ColumnSummary` counterparts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NumericSummary {
    /// Wraps the `TabularData` `NumericSummary.someCount` counterpart.
    pub some_count: usize,
    /// Wraps the `TabularData` `NumericSummary.noneCount` counterpart.
    pub none_count: usize,
    /// Wraps the `TabularData` `NumericSummary.totalCount` counterpart.
    pub total_count: usize,
    /// Wraps the `TabularData` `NumericSummary.mean` counterpart.
    pub mean: Option<f64>,
    /// Wraps the `TabularData` `NumericSummary.standardDeviation` counterpart.
    pub standard_deviation: Option<f64>,
    /// Wraps the `TabularData` `NumericSummary.min` counterpart.
    pub min: Option<f64>,
    /// Wraps the `TabularData` `NumericSummary.max` counterpart.
    pub max: Option<f64>,
    /// Wraps the `TabularData` `NumericSummary.median` counterpart.
    pub median: Option<f64>,
    /// Wraps the `TabularData` `NumericSummary.firstQuartile` counterpart.
    pub first_quartile: Option<f64>,
    /// Wraps the `TabularData` `NumericSummary.thirdQuartile` counterpart.
    pub third_quartile: Option<f64>,
}

/// Wraps categorical summaries produced by `TabularData` `ColumnSummary` counterparts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoricalSummary {
    /// Wraps the `TabularData` `CategoricalSummary.someCount` counterpart.
    pub some_count: usize,
    /// Wraps the `TabularData` `CategoricalSummary.noneCount` counterpart.
    pub none_count: usize,
    /// Wraps the `TabularData` `CategoricalSummary.totalCount` counterpart.
    pub total_count: usize,
    /// Wraps the `TabularData` `CategoricalSummary.uniqueCount` counterpart.
    pub unique_count: usize,
    /// Wraps the `TabularData` `CategoricalSummary.mode` counterpart.
    pub mode: Vec<AnyValue>,
}

/// Wraps `TabularData` `ColumnSummary` counterparts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ColumnSummary {
    /// Wraps the `TabularData` `ColumnSummary.numeric` case.
    Numeric(NumericSummary),
    /// Wraps the `TabularData` `ColumnSummary.categorical` case.
    Categorical(CategoricalSummary),
}

#[allow(clippy::cast_precision_loss)]
pub(crate) fn summarize_values(values: &[AnyValue]) -> ColumnSummary {
    let some_values: Vec<&AnyValue> = values.iter().filter(|value| !value.is_null()).collect();
    let none_count = values.len().saturating_sub(some_values.len());

    if some_values.iter().all(|value| value.as_f64().is_some()) {
        let mut numeric_values: Vec<f64> = some_values
            .iter()
            .filter_map(|value| value.as_f64())
            .collect();
        numeric_values.sort_by(f64::total_cmp);
        let count = numeric_values.len();
        let mean = if count == 0 {
            None
        } else {
            Some(numeric_values.iter().sum::<f64>() / count as f64)
        };
        let standard_deviation = mean.and_then(|mean| {
            if count < 2 {
                None
            } else {
                let variance = numeric_values
                    .iter()
                    .map(|value| {
                        let delta = value - mean;
                        delta * delta
                    })
                    .sum::<f64>()
                    / (count as f64 - 1.0);
                Some(variance.sqrt())
            }
        });
        return ColumnSummary::Numeric(NumericSummary {
            some_count: count,
            none_count,
            total_count: values.len(),
            mean,
            standard_deviation,
            min: numeric_values.first().copied(),
            max: numeric_values.last().copied(),
            median: percentile(&numeric_values, 0.5),
            first_quartile: percentile(&numeric_values, 0.25),
            third_quartile: percentile(&numeric_values, 0.75),
        });
    }

    let mut counts: BTreeMap<String, (AnyValue, usize)> = BTreeMap::new();
    for value in some_values {
        let key = value.stable_key();
        counts
            .entry(key)
            .and_modify(|(_, count)| *count += 1)
            .or_insert_with(|| ((*value).clone(), 1));
    }
    let max_count = counts.values().map(|(_, count)| *count).max().unwrap_or(0);
    let mode = counts
        .values()
        .filter(|(_, count)| *count == max_count)
        .map(|(value, _)| value.clone())
        .collect();
    ColumnSummary::Categorical(CategoricalSummary {
        some_count: values.len() - none_count,
        none_count,
        total_count: values.len(),
        unique_count: counts.len(),
        mode,
    })
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::suboptimal_flops
)]
fn percentile(sorted: &[f64], proportion: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let last_index = (sorted.len() - 1) as f64;
    let position = last_index * proportion;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    if lower == upper {
        Some(sorted[lower])
    } else {
        let weight = position - lower as f64;
        Some(sorted[lower] + (sorted[upper] - sorted[lower]) * weight)
    }
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.summaryFrame` counterpart.
    pub fn summary_frame(&self) -> Result<Self, TabularDataError> {
        self.summary()
    }

    /// Wraps the `TabularData` `DataFrame.summaryColumns` counterpart.
    pub fn summary_columns<S: AsRef<str>>(&self, columns: &[S]) -> Result<Self, TabularDataError> {
        let columns: Vec<String> = columns
            .iter()
            .map(|column| column.as_ref().to_string())
            .collect();
        let columns = encode_json_cstring(&columns, "summary columns")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_summary_columns(self.as_raw(), columns.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.summaryIndices` counterpart.
    pub fn summary_indices(&self, indices: &[usize]) -> Result<Self, TabularDataError> {
        let indices = encode_json_cstring(&indices, "summary indices")?;
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::td_dataframe_summary_indices(self.as_raw(), indices.as_ptr(), &mut raw, &mut error)
        };
        if status == ffi::status::OK {
            Ok(Self::from_raw(raw))
        } else {
            Err(from_swift(status, error))
        }
    }

    /// Wraps the `TabularData` `DataFrame.columnSummary` counterpart.
    pub fn column_summary(&self, name: &str) -> Result<ColumnSummary, TabularDataError> {
        Ok(self.any_column(name)?.summary())
    }
}
