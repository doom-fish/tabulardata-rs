use serde::Deserialize;
use serde_json::{Number, Value};

use crate::error::TabularDataError;
use crate::private::decode_json;

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnData {
    Strings(Vec<Option<String>>),
    Ints(Vec<Option<i64>>),
    Doubles(Vec<Option<f64>>),
    Bools(Vec<Option<bool>>),
}

impl ColumnData {
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Strings(values) => values.len(),
            Self::Ints(values) => values.len(),
            Self::Doubles(values) => values.len(),
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
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn missing_count(&self) -> usize {
        match &self.data {
            ColumnData::Strings(values) => values.iter().filter(|value| value.is_none()).count(),
            ColumnData::Ints(values) => values.iter().filter(|value| value.is_none()).count(),
            ColumnData::Doubles(values) => values.iter().filter(|value| value.is_none()).count(),
            ColumnData::Bools(values) => values.iter().filter(|value| value.is_none()).count(),
        }
    }

    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match &self.data {
            ColumnData::Strings(_) => "String",
            ColumnData::Ints(_) => "Int",
            ColumnData::Doubles(_) => "Double",
            ColumnData::Bools(_) => "Bool",
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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
        ColumnData::Doubles(values) => values
            .iter()
            .map(|value| {
                value.as_ref().map_or(Ok(Value::Null), |value| {
                    Number::from_f64(*value).map(Value::Number).ok_or_else(|| {
                        TabularDataError::InvalidArgument(
                            "double columns must not contain NaN or infinite values".into(),
                        )
                    })
                })
            })
            .collect::<Result<Vec<_>, _>>()?,
        ColumnData::Bools(values) => values
            .iter()
            .map(|value| value.map_or(Value::Null, Value::Bool))
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
    let payload = decode_json::<ColumnPayload>(ptr)?;
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
        other => Err(TabularDataError::FrameworkError(format!(
            "unsupported column kind: {other}"
        ))),
    }
}
