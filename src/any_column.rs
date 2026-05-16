use std::cmp::Ordering;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::column_slice::ColumnSlice;
use crate::dataframe::DataFrame;
use crate::error::{from_swift, TabularDataError};
use crate::ffi;
use crate::private::to_cstring;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum AnyValue {
    #[default]
    Null,
    String(String),
    Int(i64),
    Double(f64),
    Bool(bool),
    Date(f64),
    Data(String),
    Array(Vec<Self>),
    Object(BTreeMap<String, Self>),
}

impl AnyValue {
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) | Self::Data(value) => Some(value),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Int(value) => value.to_string().parse::<f64>().ok(),
            Self::Double(value) | Self::Date(value) => Some(*value),
            _ => None,
        }
    }

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

    #[must_use]
    pub fn stable_key(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| format!("{self:?}"))
    }

    #[must_use]
    pub fn equals(&self, other: &Self) -> bool {
        match (self.as_f64(), other.as_f64()) {
            (Some(left), Some(right)) => left.total_cmp(&right) == Ordering::Equal,
            _ => self == other,
        }
    }

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnyColumn {
    pub name: String,
    pub type_name: String,
    pub missing_count: usize,
    pub values: Vec<AnyValue>,
}

impl AnyColumn {
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

    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[must_use]
    pub fn value(&self, index: usize) -> Option<&AnyValue> {
        self.values.get(index)
    }

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

    #[must_use]
    pub fn summary(&self) -> crate::summary::ColumnSummary {
        crate::summary::summarize_values(&self.values)
    }
}

impl DataFrame {
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

    pub fn any_columns(&self) -> Result<Vec<AnyColumn>, TabularDataError> {
        self.column_names()?
            .into_iter()
            .map(|name| self.any_column(&name))
            .collect()
    }
}
