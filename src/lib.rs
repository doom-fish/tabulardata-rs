#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [`TabularData`](https://developer.apple.com/documentation/tabulardata)
//! framework.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

pub mod any_column;
pub mod any_row;
pub mod column;
pub mod column_encoder;
pub mod column_slice;
pub mod csv_reader;
pub mod csv_writer;
pub mod dataframe;
pub mod error;
pub mod ffi;
pub mod filter;
pub mod formatting;
pub mod groupby;
pub mod join;
pub mod json;
pub mod mutation;
mod private;
pub mod protocols;
pub mod sframe;
pub mod shaped_data;
pub mod slicing;
pub mod sort;
pub mod split;
pub mod summary;

pub use any_column::{AnyColumn, AnyValue};
pub use any_row::AnyRow;
pub use column::{Column, ColumnData};
pub use column_encoder::{ColumnCodec, ColumnElementType};
pub use column_slice::ColumnSlice;
pub use csv_reader::{CSVReadRequest, CSVType, DateParseStrategy};
pub use csv_writer::DateWriteStrategy;
pub use dataframe::{CSVReadingOptions, CSVWritingOptions, DataFrame, JoinKind};
pub use error::TabularDataError;
pub use filter::{ComparisonOp, Filter};
pub use formatting::FormattingOptions;
pub use groupby::{GroupBy, GroupSummaries, GroupValueType, TimeUnit};
pub use join::JoinColumns;
pub use json::{
    JSONReadRequest, JSONReadingError, JSONReadingOptions, JSONType, JSONWritingOptions,
};
pub use protocols::{
    AnyColumnProtocol, AnyColumnPrototype, AnyColumnSlice, ColumnId, ColumnProtocol,
    ColumnPrototype, DataFrameProtocol, DataFrameRow, DataFrameRows, DataFrameSlice,
    DiscontiguousColumnSlice, FilledColumn, OptionalColumnProtocol, SummaryColumnIds,
};
pub use sframe::{SFrameReadRequest, SFrameReadingError};
pub use shaped_data::ShapedData;
pub use sort::{SortKey, SortOrder};
pub use summary::{CategoricalSummary, ColumnSummary, NumericSummary};

/// Common imports.
pub mod prelude {
    pub use crate::any_column::{AnyColumn, AnyValue};
    pub use crate::any_row::AnyRow;
    pub use crate::column::{Column, ColumnData};
    pub use crate::column_encoder::{ColumnCodec, ColumnElementType};
    pub use crate::column_slice::ColumnSlice;
    pub use crate::csv_reader::{CSVReadRequest, CSVType, DateParseStrategy};
    pub use crate::csv_writer::DateWriteStrategy;
    pub use crate::dataframe::{CSVReadingOptions, CSVWritingOptions, DataFrame, JoinKind};
    pub use crate::error::TabularDataError;
    pub use crate::filter::{ComparisonOp, Filter};
    pub use crate::formatting::FormattingOptions;
    pub use crate::groupby::{GroupBy, GroupSummaries, GroupValueType, TimeUnit};
    pub use crate::join::JoinColumns;
    pub use crate::json::{
        JSONReadRequest, JSONReadingError, JSONReadingOptions, JSONType, JSONWritingOptions,
    };
    pub use crate::protocols::{
        AnyColumnProtocol, AnyColumnPrototype, AnyColumnSlice, ColumnId, ColumnProtocol,
        ColumnPrototype, DataFrameProtocol, DataFrameRow, DataFrameRows, DataFrameSlice,
        DiscontiguousColumnSlice, FilledColumn, OptionalColumnProtocol, SummaryColumnIds,
    };
    pub use crate::sframe::{SFrameReadRequest, SFrameReadingError};
    pub use crate::shaped_data::ShapedData;
    pub use crate::sort::{SortKey, SortOrder};
    pub use crate::summary::{CategoricalSummary, ColumnSummary, NumericSummary};
}
