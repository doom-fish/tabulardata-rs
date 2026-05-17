#![allow(clippy::use_self)]

use core::marker::PhantomData;
use std::fmt;
use std::ops::Range;

use crate::any_column::{AnyColumn, AnyValue};
use crate::any_row::AnyRow;
use crate::column::Column;
use crate::column_slice::ColumnSlice;
use crate::dataframe::DataFrame;
use crate::error::TabularDataError;
use crate::summary::ColumnSummary;

pub type DataFrameSlice = DataFrame;
pub type DataFrameRow = AnyRow;
pub type DataFrameRows = Vec<AnyRow>;
pub type FilledColumn = Column;
pub type AnyColumnSlice = ColumnSlice;
pub type DiscontiguousColumnSlice = ColumnSlice;
pub type SummaryColumnIds = Vec<usize>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnPrototype {
    pub name: String,
    pub type_name: String,
}

impl ColumnPrototype {
    #[must_use]
    pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
        }
    }
}

pub trait AnyColumnPrototype {
    fn name(&self) -> &str;
    fn wrapped_element_type(&self) -> &str;
    fn make_column(&self, capacity: usize) -> Column;
}

impl AnyColumnPrototype for ColumnPrototype {
    fn name(&self) -> &str {
        &self.name
    }

    fn wrapped_element_type(&self) -> &str {
        &self.type_name
    }

    fn make_column(&self, capacity: usize) -> Column {
        Column::with_capacity(self.name.clone(), &self.type_name, capacity)
    }
}

pub trait AnyColumnProtocol {
    fn name(&self) -> &str;
    fn count(&self) -> usize;
    fn wrapped_element_type(&self) -> &str;
    fn value(&self, index: usize) -> Option<AnyValue>;
    fn slice_values(&self, range: Range<usize>) -> ColumnSlice;
    fn prototype(&self) -> ColumnPrototype {
        ColumnPrototype::new(self.name(), self.wrapped_element_type())
    }

    fn is_nil(&self, index: usize) -> bool {
        self.value(index).map_or(true, |value| value.is_null())
    }
}

pub trait OptionalColumnProtocol: AnyColumnProtocol {
    fn missing_count(&self) -> usize;
}

pub trait ColumnProtocol: OptionalColumnProtocol {
    fn distinct_values(&self) -> ColumnSlice;
    fn summarize(&self) -> ColumnSummary;
}

impl AnyColumnProtocol for Column {
    fn name(&self) -> &str {
        &self.name
    }

    fn count(&self) -> usize {
        self.len()
    }

    fn wrapped_element_type(&self) -> &str {
        self.type_name()
    }

    fn value(&self, index: usize) -> Option<AnyValue> {
        self.value(index)
    }

    fn slice_values(&self, range: Range<usize>) -> ColumnSlice {
        self.slice(range)
    }
}

impl OptionalColumnProtocol for Column {
    fn missing_count(&self) -> usize {
        self.missing_count()
    }
}

impl ColumnProtocol for Column {
    fn distinct_values(&self) -> ColumnSlice {
        self.distinct()
    }

    fn summarize(&self) -> ColumnSummary {
        self.summary()
    }
}

impl AnyColumnProtocol for AnyColumn {
    fn name(&self) -> &str {
        &self.name
    }

    fn count(&self) -> usize {
        self.len()
    }

    fn wrapped_element_type(&self) -> &str {
        &self.type_name
    }

    fn value(&self, index: usize) -> Option<AnyValue> {
        self.value(index).cloned()
    }

    fn slice_values(&self, range: Range<usize>) -> ColumnSlice {
        self.slice(range)
    }
}

impl OptionalColumnProtocol for AnyColumn {
    fn missing_count(&self) -> usize {
        self.missing_count
    }
}

impl ColumnProtocol for AnyColumn {
    fn distinct_values(&self) -> ColumnSlice {
        self.distinct()
    }

    fn summarize(&self) -> ColumnSummary {
        self.summary()
    }
}

impl AnyColumnProtocol for ColumnSlice {
    fn name(&self) -> &str {
        &self.name
    }

    fn count(&self) -> usize {
        self.len()
    }

    fn wrapped_element_type(&self) -> &str {
        &self.type_name
    }

    fn value(&self, index: usize) -> Option<AnyValue> {
        self.value(index).cloned()
    }

    fn slice_values(&self, range: Range<usize>) -> ColumnSlice {
        self.range(range)
    }
}

impl OptionalColumnProtocol for ColumnSlice {
    fn missing_count(&self) -> usize {
        self.missing_count
    }
}

impl ColumnProtocol for ColumnSlice {
    fn distinct_values(&self) -> ColumnSlice {
        self.distinct()
    }

    fn summarize(&self) -> ColumnSummary {
        self.summary()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnId<T> {
    name: String,
    marker: PhantomData<fn() -> T>,
}

impl<T> ColumnId<T> {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            marker: PhantomData,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> String {
        format!("{}:{}", self.name, core::any::type_name::<T>())
    }
}

impl<T> fmt::Display for ColumnId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}

pub trait DataFrameProtocol {
    fn base(&self) -> Result<DataFrame, TabularDataError>;
    fn rows_vec(&self) -> Result<DataFrameRows, TabularDataError>;
    fn columns_vec(&self) -> Result<Vec<AnyColumn>, TabularDataError>;
    fn shape(&self) -> (usize, usize);

    fn is_empty(&self) -> bool {
        self.shape().0 == 0 || self.shape().1 == 0
    }

    fn range(&self, range: Range<usize>) -> Result<DataFrameSlice, TabularDataError>;

    fn column_by_id<T>(&self, id: &ColumnId<T>) -> Result<Column, TabularDataError>;
    fn contains_column_id<T>(&self, id: &ColumnId<T>) -> Result<bool, TabularDataError>;
}

impl DataFrameProtocol for DataFrame {
    fn base(&self) -> Result<DataFrame, TabularDataError> {
        self.try_clone()
    }

    fn rows_vec(&self) -> Result<DataFrameRows, TabularDataError> {
        self.rows()
    }

    fn columns_vec(&self) -> Result<Vec<AnyColumn>, TabularDataError> {
        self.any_columns()
    }

    fn shape(&self) -> (usize, usize) {
        DataFrame::shape(self)
    }

    fn range(&self, range: Range<usize>) -> Result<DataFrameSlice, TabularDataError> {
        self.slice_rows(range)
    }

    fn column_by_id<T>(&self, id: &ColumnId<T>) -> Result<Column, TabularDataError> {
        self.column(id.name())
    }

    fn contains_column_id<T>(&self, id: &ColumnId<T>) -> Result<bool, TabularDataError> {
        self.contains_column(id.name())
    }
}
