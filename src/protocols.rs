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

/// Aliases the `TabularData` `DataFrame` slice-style counterpart used by this crate.
pub type DataFrameSlice = DataFrame;
/// Aliases the `TabularData` `DataFrame.Row` counterpart used by this crate.
pub type DataFrameRow = AnyRow;
/// Aliases `TabularData` `DataFrame.Row` collections used by this crate.
pub type DataFrameRows = Vec<AnyRow>;
/// Aliases the filled-column wrapper used by `TabularData` typed-column counterparts.
pub type FilledColumn = Column;
/// Aliases the `TabularData` `AnyColumn` slice-style counterpart used by this crate.
pub type AnyColumnSlice = ColumnSlice;
/// Aliases the discontiguous column-slice counterpart used by this crate.
pub type DiscontiguousColumnSlice = ColumnSlice;
/// Aliases summary column identifiers used by `TabularData` `DataFrame` counterparts.
pub type SummaryColumnIds = Vec<usize>;

/// Wraps column prototypes used by `TabularData` typed-column counterparts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnPrototype {
    /// Wraps the `TabularData` `ColumnPrototype.name` counterpart.
    pub name: String,
    /// Wraps the `TabularData` `ColumnPrototype.typeName` counterpart.
    pub type_name: String,
}

impl ColumnPrototype {
    /// Wraps the `TabularData` `ColumnPrototype.init` counterpart.
    #[must_use]
    pub fn new(name: impl Into<String>, type_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
        }
    }
}

/// Mirrors `TabularData` typed-column prototype requirements.
pub trait AnyColumnPrototype {
    /// Matches the `TabularData` `AnyColumnPrototype.name` requirement.
    fn name(&self) -> &str;
    /// Matches the `TabularData` `AnyColumnPrototype.wrappedElementType` requirement.
    fn wrapped_element_type(&self) -> &str;
    /// Matches the `TabularData` `AnyColumnPrototype.makeColumn` requirement.
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

/// Mirrors `TabularData` typed-column protocol requirements.
pub trait AnyColumnProtocol {
    /// Matches the `TabularData` `AnyColumnProtocol.name` requirement.
    fn name(&self) -> &str;
    /// Matches the `TabularData` `AnyColumnProtocol.count` requirement.
    fn count(&self) -> usize;
    /// Matches the `TabularData` `AnyColumnProtocol.wrappedElementType` requirement.
    fn wrapped_element_type(&self) -> &str;
    /// Matches the `TabularData` `AnyColumnProtocol.value` requirement.
    fn value(&self, index: usize) -> Option<AnyValue>;
    /// Matches the `TabularData` `AnyColumnProtocol.sliceValues` requirement.
    fn slice_values(&self, range: Range<usize>) -> ColumnSlice;
    /// Matches the `TabularData` `AnyColumnProtocol.prototype` requirement.
    fn prototype(&self) -> ColumnPrototype {
        ColumnPrototype::new(self.name(), self.wrapped_element_type())
    }

    /// Matches the `TabularData` `AnyColumnProtocol.isNil` requirement.
    fn is_nil(&self, index: usize) -> bool {
        self.value(index).map_or(true, |value| value.is_null())
    }
}

/// Mirrors `TabularData` optional-column protocol requirements.
pub trait OptionalColumnProtocol: AnyColumnProtocol {
    /// Matches the `TabularData` `OptionalColumnProtocol.missingCount` requirement.
    fn missing_count(&self) -> usize;
}

/// Mirrors `TabularData` filled-column protocol requirements.
pub trait ColumnProtocol: OptionalColumnProtocol {
    /// Matches the `TabularData` `ColumnProtocol.distinctValues` requirement.
    fn distinct_values(&self) -> ColumnSlice;
    /// Matches the `TabularData` `ColumnProtocol.summarize` requirement.
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

/// Wraps typed column identifiers used with `TabularData` `DataFrame` counterparts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnId<T> {
    name: String,
    marker: PhantomData<fn() -> T>,
}

impl<T> ColumnId<T> {
    /// Wraps the `TabularData` `ColumnId.init` counterpart.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            marker: PhantomData,
        }
    }

    /// Wraps the `TabularData` `ColumnId.name` counterpart.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Wraps the `TabularData` `ColumnId.description` counterpart.
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

/// Mirrors shared `DataFrame` protocol conveniences built on `TabularData` counterparts.
pub trait DataFrameProtocol {
    /// Matches the `TabularData` `DataFrameProtocol.base` requirement.
    fn base(&self) -> Result<DataFrame, TabularDataError>;
    /// Matches the `TabularData` `DataFrameProtocol.rowsVec` requirement.
    fn rows_vec(&self) -> Result<DataFrameRows, TabularDataError>;
    /// Matches the `TabularData` `DataFrameProtocol.columnsVec` requirement.
    fn columns_vec(&self) -> Result<Vec<AnyColumn>, TabularDataError>;
    /// Matches the `TabularData` `DataFrameProtocol.shape` requirement.
    fn shape(&self) -> (usize, usize);

    /// Matches the `TabularData` `DataFrameProtocol.isEmpty` requirement.
    fn is_empty(&self) -> bool {
        self.shape().0 == 0 || self.shape().1 == 0
    }

    /// Matches the `TabularData` `DataFrameProtocol.range` requirement.
    fn range(&self, range: Range<usize>) -> Result<DataFrameSlice, TabularDataError>;

    /// Matches the `TabularData` `DataFrameProtocol.columnById` requirement.
    fn column_by_id<T>(&self, id: &ColumnId<T>) -> Result<Column, TabularDataError>;
    /// Matches the `TabularData` `DataFrameProtocol.containsColumnId` requirement.
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
