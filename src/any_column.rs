#![allow(clippy::cast_precision_loss)]

use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::column_slice::ColumnSlice;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::to_cstring;

/// Wraps values carried by `TabularData` `AnyColumn` and `DataFrame.Row` counterparts.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum AnyValue {
    /// Wraps the `TabularData` `AnyValue.null` case.
    #[default]
    Null,
    /// Wraps the `TabularData` `AnyValue.string` case.
    String(String),
    /// Wraps the `TabularData` `AnyValue.int` case.
    Int(i64),
    /// Wraps the `TabularData` `AnyValue.double` case.
    Double(f64),
    /// Wraps the `TabularData` `AnyValue.bool` case.
    Bool(bool),
    /// Wraps the `TabularData` `AnyValue.date` case.
    Date(f64),
    /// Wraps the `TabularData` `AnyValue.data` case.
    Data(String),
    /// Wraps the `TabularData` `AnyValue.array` case.
    Array(Vec<Self>),
    /// Wraps the `TabularData` `AnyValue.object` case.
    Object(BTreeMap<String, Self>),
}

impl AnyValue {
    /// Wraps the `TabularData` `AnyValue.isNull` counterpart.
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Wraps the `TabularData` `AnyValue.asStr` counterpart.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) | Self::Data(value) => Some(value),
            _ => None,
        }
    }

    /// Wraps the `TabularData` `AnyValue.asBool` counterpart.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Wraps the `TabularData` `AnyValue.asI64` counterpart.
    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Wraps the `TabularData` `AnyValue.asF64` counterpart.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Int(value) => value.to_string().parse::<f64>().ok(),
            Self::Double(value) | Self::Date(value) => Some(*value),
            _ => None,
        }
    }

    /// Wraps the `TabularData` `AnyValue.typeName` counterpart.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::String(_) => "string",
            Self::Int(_) => "int",
            Self::Double(_) => "double",
            Self::Bool(_) => "bool",
            Self::Date(_) => "date",
            Self::Data(_) => "data",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }

    /// Wraps the `TabularData` `AnyValue.stableKey` counterpart.
    #[must_use]
    pub fn stable_key(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| format!("{self:?}"))
    }

    /// Wraps the `TabularData` `AnyValue.equals` counterpart.
    #[must_use]
    pub fn equals(&self, other: &Self) -> bool {
        match (self.as_f64(), other.as_f64()) {
            (Some(left), Some(right)) => left.total_cmp(&right) == Ordering::Equal,
            _ => self == other,
        }
    }

    /// Wraps the `TabularData` `AnyValue.partialCmp` counterpart.
    #[must_use]
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::String(left), Self::String(right)) | (Self::Data(left), Self::Data(right)) => {
                Some(left.cmp(right))
            }
            (Self::Bool(left), Self::Bool(right)) => Some(left.cmp(right)),
            (Self::Date(left), Self::Date(right)) => left.partial_cmp(right),
            _ => match (self.as_f64(), other.as_f64()) {
                (Some(left), Some(right)) => left.partial_cmp(&right),
                _ => None,
            },
        }
    }

    /// Wraps the `TabularData` `AnyValue.contains` counterpart.
    #[must_use]
    pub fn contains(&self, needle: &Self) -> bool {
        match self {
            Self::String(haystack) => needle
                .as_str()
                .is_some_and(|value| haystack.contains(value)),
            Self::Array(values) => values.iter().any(|value| value.equals(needle)),
            Self::Object(map) => needle.as_str().is_some_and(|key| map.contains_key(key)),
            _ => false,
        }
    }
}

impl From<&str> for AnyValue {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for AnyValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for AnyValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for AnyValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<i32> for AnyValue {
    fn from(value: i32) -> Self {
        Self::Int(i64::from(value))
    }
}

impl From<usize> for AnyValue {
    fn from(value: usize) -> Self {
        Self::Int(i64::try_from(value).unwrap_or(i64::MAX))
    }
}

impl From<f64> for AnyValue {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<f32> for AnyValue {
    fn from(value: f32) -> Self {
        Self::Double(f64::from(value))
    }
}

/// Wraps the `TabularData` `AnyColumn` counterpart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnyColumn {
    /// Wraps the `TabularData` `AnyColumn.name` counterpart.
    pub name: String,
    /// Wraps the `TabularData` `AnyColumn.typeName` counterpart.
    pub type_name: String,
    /// Wraps the `TabularData` `AnyColumn.missingCount` counterpart.
    pub missing_count: usize,
    /// Wraps the `TabularData` `AnyColumn.values` counterpart.
    pub values: Vec<AnyValue>,
}

impl AnyColumn {
    /// Wraps the `TabularData` `AnyColumn.init` counterpart.
    #[must_use]
    pub fn new(name: impl Into<String>, values: Vec<AnyValue>) -> Self {
        let missing_count = values.iter().filter(|value| value.is_null()).count();
        let type_name = values
            .iter()
            .find(|value| !value.is_null())
            .map_or_else(|| "null".into(), |value| value.type_name().into());
        Self {
            name: name.into(),
            type_name,
            missing_count,
            values,
        }
    }

    /// Wraps the `TabularData` `AnyColumn.len` counterpart.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Wraps the `TabularData` `AnyColumn.isEmpty` counterpart.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Wraps the `TabularData` `AnyColumn.value` counterpart.
    #[must_use]
    pub fn value(&self, index: usize) -> Option<&AnyValue> {
        self.values.get(index)
    }

    /// Wraps the `TabularData` `AnyColumn.isNil` counterpart.
    #[must_use]
    pub fn is_nil(&self, index: usize) -> bool {
        self.value(index).map_or(true, AnyValue::is_null)
    }

    /// Wraps the `TabularData` `AnyColumn.slice` counterpart.
    #[must_use]
    pub fn slice(&self, range: std::ops::Range<usize>) -> ColumnSlice {
        let start = range.start.min(self.values.len());
        let end = range.end.min(self.values.len());
        let values = self.values[start..end].to_vec();
        ColumnSlice::new(
            self.name.clone(),
            self.type_name.clone(),
            values,
            true,
            (start..end).collect(),
        )
    }

    /// Wraps the `TabularData` `AnyColumn.mask` counterpart.
    #[must_use]
    pub fn mask(&self, mask: &[bool]) -> ColumnSlice {
        let mut values = Vec::new();
        let mut indices = Vec::new();
        for (index, (value, is_selected)) in self.values.iter().zip(mask.iter()).enumerate() {
            if *is_selected {
                values.push(value.clone());
                indices.push(index);
            }
        }
        ColumnSlice::new(
            self.name.clone(),
            self.type_name.clone(),
            values,
            false,
            indices,
        )
    }

    /// Wraps the `TabularData` `AnyColumn.distinct` counterpart.
    #[must_use]
    pub fn distinct(&self) -> ColumnSlice {
        let mut seen = std::collections::BTreeSet::new();
        let mut values = Vec::new();
        let mut indices = Vec::new();
        for (index, value) in self.values.iter().enumerate() {
            let key = value.stable_key();
            if seen.insert(key) {
                values.push(value.clone());
                indices.push(index);
            }
        }
        ColumnSlice::new(
            self.name.clone(),
            self.type_name.clone(),
            values,
            false,
            indices,
        )
    }

    /// Wraps the `TabularData` `AnyColumn.summary` counterpart.
    #[must_use]
    pub fn summary(&self) -> crate::summary::ColumnSummary {
        crate::summary::summarize_values(&self.values)
    }

    /// Wraps the `TabularData` `AnyColumn.toColumn` counterpart.
    pub fn to_column(&self) -> Result<crate::column::Column, TabularDataError> {
        crate::column::Column::from_any_values(self.name.clone(), &self.type_name, &self.values)
    }

    /// Wraps the `TabularData` `AnyColumn.min` counterpart.
    #[must_use]
    pub fn min(&self) -> Option<AnyValue> {
        self.values
            .iter()
            .filter(|value| !value.is_null())
            .cloned()
            .min_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal))
    }

    /// Wraps the `TabularData` `AnyColumn.max` counterpart.
    #[must_use]
    pub fn max(&self) -> Option<AnyValue> {
        self.values
            .iter()
            .filter(|value| !value.is_null())
            .cloned()
            .max_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal))
    }

    /// Wraps the `TabularData` `AnyColumn.argmin` counterpart.
    #[must_use]
    pub fn argmin(&self) -> Option<usize> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, value)| !value.is_null())
            .min_by(|(_, left), (_, right)| left.partial_cmp(right).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
    }

    /// Wraps the `TabularData` `AnyColumn.argmax` counterpart.
    #[must_use]
    pub fn argmax(&self) -> Option<usize> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, value)| !value.is_null())
            .max_by(|(_, left), (_, right)| left.partial_cmp(right).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
    }

    /// Wraps the `TabularData` `AnyColumn.sum` counterpart.
    #[must_use]
    pub fn sum(&self) -> Option<f64> {
        let values: Vec<f64> = self.values.iter().filter_map(AnyValue::as_f64).collect();
        (!values.is_empty()).then(|| values.iter().sum())
    }

    /// Wraps the `TabularData` `AnyColumn.mean` counterpart.
    #[must_use]
    pub fn mean(&self) -> Option<f64> {
        let values: Vec<f64> = self.values.iter().filter_map(AnyValue::as_f64).collect();
        (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
    }

    /// Wraps the `TabularData` `AnyColumn.standardDeviation` counterpart.
    #[must_use]
    pub fn standard_deviation(&self) -> Option<f64> {
        let values: Vec<f64> = self.values.iter().filter_map(AnyValue::as_f64).collect();
        if values.len() < 2 {
            return None;
        }
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values
            .iter()
            .map(|value| {
                let delta = value - mean;
                delta * delta
            })
            .sum::<f64>()
            / (values.len() as f64 - 1.0);
        Some(variance.sqrt())
    }

    /// Wraps the `TabularData` `AnyColumn.description` counterpart.
    #[must_use]
    pub fn description(&self) -> String {
        format!(
            "AnyColumn(name={}, type={}, len={}, missing={})",
            self.name,
            self.type_name,
            self.len(),
            self.missing_count
        )
    }
}

impl std::fmt::Display for AnyColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description())
    }
}

impl DataFrame {
    /// Wraps the `TabularData` `DataFrame.anyColumn` counterpart.
    pub fn any_column(&self, name: &str) -> Result<AnyColumn, TabularDataError> {
        let name = to_cstring(name)?;
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::td_dataframe_any_column_json(self.as_raw(), name.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            crate::private::decode_json(payload)
        }
    }

    /// Wraps the `TabularData` `DataFrame.anyColumns` counterpart.
    pub fn any_columns(&self) -> Result<Vec<AnyColumn>, TabularDataError> {
        self.column_names()?
            .into_iter()
            .map(|name| self.any_column(&name))
            .collect()
    }
}
