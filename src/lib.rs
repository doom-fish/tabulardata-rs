#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [TabularData](https://developer.apple.com/documentation/tabulardata)
//! framework.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

pub mod column;
pub mod dataframe;
pub mod error;
pub mod ffi;
mod private;

pub use column::{Column, ColumnData};
pub use dataframe::{CSVReadingOptions, CSVWritingOptions, DataFrame, JoinKind};
pub use error::TabularDataError;

/// Common imports.
pub mod prelude {
    pub use crate::column::{Column, ColumnData};
    pub use crate::dataframe::{CSVReadingOptions, CSVWritingOptions, DataFrame, JoinKind};
    pub use crate::error::TabularDataError;
}
