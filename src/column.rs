#![allow(clippy::cast_precision_loss, clippy::too_many_lines)]

use std::fmt;

use serde::Deserialize;
use serde_json::{Number, Value};

use crate::any_column::AnyValue;
use crate::column_slice::ColumnSlice;
use crate::error::TabularDataError;
use crate::summary::ColumnSummary;

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnData {
    Strings(Vec<Option<String>>),
    Ints(Vec<Option<i64>>),
    Doubles(Vec<Option<f64>>),
    Bools(Vec<Option<bool>>),
    Dates(Vec<Option<f64>>),
    Data(Vec<Option<String>>),
}

impl ColumnData {
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Strings(values) | Self::Data(values) => values.len(),
            Self::Ints(values) => values.len(),
            Self::Doubles(values) | Self::Dates(values) => values.len(),
            Self::Bools(values) => values.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Strings(_) => "string",
            Self::Ints(_) => "int",
            Self::Doubles(_) => "double",
            Self::Bools(_) => "bool",
            Self::Dates(_) => "date",
            Self::Data(_) => "data",
        }
    }

    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Strings(_) => "String",
            Self::Ints(_) => "Int",
            Self::Doubles(_) => "Double",
            Self::Bools(_) => "Bool",
            Self::Dates(_) => "Date",
            Self::Data(_) => "Data",
        }
    }

    #[must_use]
    pub fn missing_count(&self) -> usize {
        match self {
            Self::Strings(values) => values.iter().filter(|value| value.is_none()).count(),
            Self::Ints(values) => values.iter().filter(|value| value.is_none()).count(),
            Self::Doubles(values) => values.iter().filter(|value| value.is_none()).count(),
            Self::Bools(values) => values.iter().filter(|value| value.is_none()).count(),
            Self::Dates(values) => values.iter().filter(|value| value.is_none()).count(),
            Self::Data(values) => values.iter().filter(|value| value.is_none()).count(),
        }
    }

    #[must_use]
    pub fn cleared(&self) -> Self {
        match self {
            Self::Strings(_) => Self::Strings(Vec::new()),
            Self::Ints(_) => Self::Ints(Vec::new()),
            Self::Doubles(_) => Self::Doubles(Vec::new()),
            Self::Bools(_) => Self::Bools(Vec::new()),
            Self::Dates(_) => Self::Dates(Vec::new()),
            Self::Data(_) => Self::Data(Vec::new()),
        }
    }

    #[must_use]
    pub fn with_capacity(type_name: &str, capacity: usize) -> Self {
        match normalize_type_name(type_name).as_str() {
            "int" | "integer" => Self::Ints(Vec::with_capacity(capacity)),
            "double" | "float" => Self::Doubles(Vec::with_capacity(capacity)),
            "bool" | "boolean" => Self::Bools(Vec::with_capacity(capacity)),
            "date" => Self::Dates(Vec::with_capacity(capacity)),
            "data" | "binary" => Self::Data(Vec::with_capacity(capacity)),
            _ => Self::Strings(Vec::with_capacity(capacity)),
        }
    }

    #[must_use]
    pub fn values(&self) -> Vec<AnyValue> {
        match self {
            Self::Strings(values) => values
                .iter()
                .map(|value| value.clone().map_or(AnyValue::Null, AnyValue::String))
                .collect(),
            Self::Ints(values) => values
                .iter()
                .map(|value| value.map_or(AnyValue::Null, AnyValue::Int))
                .collect(),
            Self::Doubles(values) => values
                .iter()
                .map(|value| value.map_or(AnyValue::Null, AnyValue::Double))
                .collect(),
            Self::Bools(values) => values
                .iter()
                .map(|value| value.map_or(AnyValue::Null, AnyValue::Bool))
                .collect(),
            Self::Dates(values) => values
                .iter()
                .map(|value| value.map_or(AnyValue::Null, AnyValue::Date))
                .collect(),
            Self::Data(values) => values
                .iter()
                .map(|value| value.clone().map_or(AnyValue::Null, AnyValue::Data))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub data: ColumnData,
}

impl Column {
    #[must_use]
    pub fn with_capacity(name: impl Into<String>, type_name: &str, capacity: usize) -> Self {
        Self {
            name: name.into(),
            data: ColumnData::with_capacity(type_name, capacity),
        }
    }

    #[must_use]
    pub fn strings(name: impl Into<String>, values: Vec<Option<String>>) -> Self {
        Self {
            name: name.into(),
            data: ColumnData::Strings(values),
        }
    }

    #[must_use]
    pub fn ints(name: impl Into<String>, values: Vec<Option<i64>>) -> Self {
        Self {
            name: name.into(),
            data: ColumnData::Ints(values),
        }
    }

    #[must_use]
    pub fn doubles(name: impl Into<String>, values: Vec<Option<f64>>) -> Self {
        Self {
            name: name.into(),
            data: ColumnData::Doubles(values),
        }
    }

    #[must_use]
    pub fn bools(name: impl Into<String>, values: Vec<Option<bool>>) -> Self {
        Self {
            name: name.into(),
            data: ColumnData::Bools(values),
        }
    }

    #[must_use]
    pub fn dates(name: impl Into<String>, values: Vec<Option<f64>>) -> Self {
        Self {
            name: name.into(),
            data: ColumnData::Dates(values),
        }
    }

    #[must_use]
    pub fn binary(name: impl Into<String>, values: Vec<Option<String>>) -> Self {
        Self {
            name: name.into(),
            data: ColumnData::Data(values),
        }
    }

    pub fn from_any_values(
        name: impl Into<String>,
        type_name: &str,
        values: &[AnyValue],
    ) -> Result<Self, TabularDataError> {
        let name = name.into();
        let inferred_type = values.iter().find(|value| !value.is_null()).map_or_else(
            || type_name.to_string(),
            |value| value.type_name().to_string(),
        );
        let type_name = if normalize_type_name(type_name) == "null" {
            inferred_type
        } else {
            type_name.to_string()
        };
        match normalize_type_name(&type_name).as_str() {
            "string" => Ok(Self::strings(
                name,
                values
                    .iter()
                    .map(|value| match value {
                        AnyValue::Null => Ok(None),
                        AnyValue::String(value) => Ok(Some(value.clone())),
                        other => Err(type_mismatch("string", other)),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            "int" | "integer" => Ok(Self::ints(
                name,
                values
                    .iter()
                    .map(|value| match value {
                        AnyValue::Null => Ok(None),
                        AnyValue::Int(value) => Ok(Some(*value)),
                        other => Err(type_mismatch("int", other)),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            "double" | "float" => Ok(Self::doubles(
                name,
                values
                    .iter()
                    .map(|value| match value {
                        AnyValue::Null => Ok(None),
                        AnyValue::Int(value) => Ok(Some(*value as f64)),
                        AnyValue::Double(value) => Ok(Some(*value)),
                        other => Err(type_mismatch("double", other)),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            "bool" | "boolean" => Ok(Self::bools(
                name,
                values
                    .iter()
                    .map(|value| match value {
                        AnyValue::Null => Ok(None),
                        AnyValue::Bool(value) => Ok(Some(*value)),
                        other => Err(type_mismatch("bool", other)),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            "date" => Ok(Self::dates(
                name,
                values
                    .iter()
                    .map(|value| match value {
                        AnyValue::Null => Ok(None),
                        AnyValue::Int(value) => Ok(Some(*value as f64)),
                        AnyValue::Double(value) | AnyValue::Date(value) => Ok(Some(*value)),
                        other => Err(type_mismatch("date", other)),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            "data" | "binary" => Ok(Self::binary(
                name,
                values
                    .iter()
                    .map(|value| match value {
                        AnyValue::Null => Ok(None),
                        AnyValue::Data(value) | AnyValue::String(value) => Ok(Some(value.clone())),
                        other => Err(type_mismatch("data", other)),
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            other => Err(TabularDataError::InvalidArgument(format!(
                "unsupported column type '{other}'"
            ))),
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    #[must_use]
    pub fn cleared(&self) -> Self {
        Self {
            name: self.name.clone(),
            data: self.data.cleared(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn missing_count(&self) -> usize {
        self.data.missing_count()
    }

    #[must_use]
    pub fn type_name(&self) -> &'static str {
        self.data.type_name()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[must_use]
    pub fn value(&self, index: usize) -> Option<AnyValue> {
        self.values().get(index).cloned()
    }

    #[must_use]
    pub fn values(&self) -> Vec<AnyValue> {
        self.data.values()
    }

    #[must_use]
    pub fn slice(&self, range: std::ops::Range<usize>) -> ColumnSlice {
        let values = self.values();
        let start = range.start.min(values.len());
        let end = range.end.min(values.len());
        ColumnSlice::new(
            self.name.clone(),
            self.type_name().to_string(),
            values[start..end].to_vec(),
            true,
            (start..end).collect(),
        )
    }

    #[must_use]
    pub fn distinct(&self) -> ColumnSlice {
        self.slice(0..self.len()).distinct()
    }

    #[must_use]
    pub fn summary(&self) -> ColumnSummary {
        crate::summary::summarize_values(&self.values())
    }

    #[must_use]
    pub fn min(&self) -> Option<AnyValue> {
        extremum(&self.values(), true)
    }

    #[must_use]
    pub fn max(&self) -> Option<AnyValue> {
        extremum(&self.values(), false)
    }

    #[must_use]
    pub fn argmin(&self) -> Option<usize> {
        extremum_index(&self.values(), true)
    }

    #[must_use]
    pub fn argmax(&self) -> Option<usize> {
        extremum_index(&self.values(), false)
    }

    #[must_use]
    pub fn sum(&self) -> Option<f64> {
        let values = numeric_values(&self.values());
        (!values.is_empty()).then(|| values.iter().sum())
    }

    #[must_use]
    pub fn mean(&self) -> Option<f64> {
        let values = numeric_values(&self.values());
        (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
    }

    #[must_use]
    pub fn standard_deviation(&self) -> Option<f64> {
        standard_deviation(&numeric_values(&self.values()))
    }

    #[must_use]
    pub fn description(&self) -> String {
        format!(
            "Column(name={}, type={}, len={}, missing={})",
            self.name,
            self.type_name(),
            self.len(),
            self.missing_count()
        )
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}

#[derive(Debug, Deserialize)]
struct ColumnPayload {
    name: String,
    kind: String,
    values: Vec<Value>,
}

pub(crate) fn encode_column_json(column: &Column) -> Result<String, TabularDataError> {
    let values = match &column.data {
        ColumnData::Strings(values) => values
            .iter()
            .map(|value| {
                value
                    .as_ref()
                    .map_or(Value::Null, |value| Value::String(value.clone()))
            })
            .collect(),
        ColumnData::Ints(values) => values
            .iter()
            .map(|value| value.map_or(Value::Null, |value| Value::Number(value.into())))
            .collect(),
        ColumnData::Doubles(values) | ColumnData::Dates(values) => values
            .iter()
            .map(|value| {
                value.as_ref().map_or(Ok(Value::Null), |value| {
                    Number::from_f64(*value).map(Value::Number).ok_or_else(|| {
                        TabularDataError::InvalidArgument(format!(
                            "{} columns must not contain NaN or infinite values",
                            column.data.kind()
                        ))
                    })
                })
            })
            .collect::<Result<Vec<_>, _>>()?,
        ColumnData::Bools(values) => values
            .iter()
            .map(|value| value.map_or(Value::Null, Value::Bool))
            .collect(),
        ColumnData::Data(values) => values
            .iter()
            .map(|value| {
                value
                    .as_ref()
                    .map_or(Value::Null, |value| Value::String(value.clone()))
            })
            .collect(),
    };

    let payload = serde_json::json!({
        "name": column.name,
        "kind": column.data.kind(),
        "values": values,
    });
    serde_json::to_string(&payload).map_err(|error| {
        TabularDataError::FrameworkError(format!("failed to encode column payload: {error}"))
    })
}

pub(crate) fn decode_column_json(ptr: *mut core::ffi::c_char) -> Result<Column, TabularDataError> {
    let payload = crate::private::decode_json::<ColumnPayload>(ptr)?;
    match payload.kind.as_str() {
        "string" => Ok(Column::strings(
            payload.name,
            payload
                .values
                .into_iter()
                .map(|value| match value {
                    Value::Null => Ok(None),
                    Value::String(value) => Ok(Some(value)),
                    other => Err(TabularDataError::FrameworkError(format!(
                        "expected string column value, got {other}"
                    ))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        "int" => Ok(Column::ints(
            payload.name,
            payload
                .values
                .into_iter()
                .map(|value| match value {
                    Value::Null => Ok(None),
                    Value::Number(number) => number.as_i64().map(Some).ok_or_else(|| {
                        TabularDataError::FrameworkError(
                            "integer column values must fit in i64".into(),
                        )
                    }),
                    other => Err(TabularDataError::FrameworkError(format!(
                        "expected int column value, got {other}"
                    ))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        "double" => Ok(Column::doubles(
            payload.name,
            payload
                .values
                .into_iter()
                .map(|value| match value {
                    Value::Null => Ok(None),
                    Value::Number(number) => number.as_f64().map(Some).ok_or_else(|| {
                        TabularDataError::FrameworkError(
                            "double column values must be finite numbers".into(),
                        )
                    }),
                    other => Err(TabularDataError::FrameworkError(format!(
                        "expected double column value, got {other}"
                    ))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        "bool" => Ok(Column::bools(
            payload.name,
            payload
                .values
                .into_iter()
                .map(|value| match value {
                    Value::Null => Ok(None),
                    Value::Bool(value) => Ok(Some(value)),
                    other => Err(TabularDataError::FrameworkError(format!(
                        "expected bool column value, got {other}"
                    ))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        "date" => Ok(Column::dates(
            payload.name,
            payload
                .values
                .into_iter()
                .map(|value| match value {
                    Value::Null => Ok(None),
                    Value::Number(number) => number.as_f64().map(Some).ok_or_else(|| {
                        TabularDataError::FrameworkError(
                            "date column values must be numeric timestamps".into(),
                        )
                    }),
                    other => Err(TabularDataError::FrameworkError(format!(
                        "expected date column value, got {other}"
                    ))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        "data" => Ok(Column::binary(
            payload.name,
            payload
                .values
                .into_iter()
                .map(|value| match value {
                    Value::Null => Ok(None),
                    Value::String(value) => Ok(Some(value)),
                    other => Err(TabularDataError::FrameworkError(format!(
                        "expected data column value, got {other}"
                    ))),
                })
                .collect::<Result<Vec<_>, _>>()?,
        )),
        other => Err(TabularDataError::FrameworkError(format!(
            "unsupported column kind: {other}"
        ))),
    }
}

fn normalize_type_name(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .replace("Swift.", "")
        .to_lowercase()
}

fn type_mismatch(expected: &str, value: &AnyValue) -> TabularDataError {
    TabularDataError::InvalidArgument(format!(
        "expected {expected} value, got {}",
        value.type_name()
    ))
}

fn numeric_values(values: &[AnyValue]) -> Vec<f64> {
    values.iter().filter_map(AnyValue::as_f64).collect()
}

fn standard_deviation(values: &[f64]) -> Option<f64> {
    let mean = (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)?;
    (values.len() >= 2).then(|| {
        let variance = values
            .iter()
            .map(|value| {
                let delta = value - mean;
                delta * delta
            })
            .sum::<f64>()
            / (values.len() as f64 - 1.0);
        variance.sqrt()
    })
}

fn extremum(values: &[AnyValue], min: bool) -> Option<AnyValue> {
    let mut best: Option<&AnyValue> = None;
    for value in values.iter().filter(|value| !value.is_null()) {
        let replace = best.map_or(true, |current| {
            value.partial_cmp(current).is_some_and(|ordering| {
                if min {
                    ordering.is_lt()
                } else {
                    ordering.is_gt()
                }
            })
        });
        if replace {
            best = Some(value);
        }
    }
    best.cloned()
}

fn extremum_index(values: &[AnyValue], min: bool) -> Option<usize> {
    let mut best: Option<(usize, &AnyValue)> = None;
    for (index, value) in values
        .iter()
        .enumerate()
        .filter(|(_, value)| !value.is_null())
    {
        let replace = best.map_or(true, |(_, current)| {
            value.partial_cmp(current).is_some_and(|ordering| {
                if min {
                    ordering.is_lt()
                } else {
                    ordering.is_gt()
                }
            })
        });
        if replace {
            best = Some((index, value));
        }
    }
    best.map(|(index, _)| index)
}
