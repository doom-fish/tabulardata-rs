use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::TabularDataError;

/// Wraps shaped values used with `TabularData` typed-column counterparts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapedData<T> {
    /// Wraps the `TabularData` `ShapedData.shape` counterpart.
    pub shape: Vec<usize>,
    /// Wraps the `TabularData` `ShapedData.strides` counterpart.
    pub strides: Vec<usize>,
    /// Wraps the `TabularData` `ShapedData.contents` counterpart.
    pub contents: Vec<T>,
}

impl<T> ShapedData<T> {
    /// Wraps the `TabularData` `ShapedData.init` counterpart.
    pub fn new(
        shape: Vec<usize>,
        strides: Vec<usize>,
        contents: Vec<T>,
    ) -> Result<Self, TabularDataError> {
        if shape.is_empty() {
            return Err(TabularDataError::InvalidArgument(
                "shape must contain at least one dimension".into(),
            ));
        }
        if shape.len() != strides.len() {
            return Err(TabularDataError::InvalidArgument(
                "shape and strides must have the same rank".into(),
            ));
        }
        let expected = shape.iter().product::<usize>();
        if expected != contents.len() {
            return Err(TabularDataError::InvalidArgument(format!(
                "shape {shape:?} requires {expected} values, got {}",
                contents.len()
            )));
        }
        Ok(Self {
            shape,
            strides,
            contents,
        })
    }

    /// Wraps the `TabularData` `ShapedData.rank` counterpart.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// Wraps the `TabularData` `ShapedData.linearIndex` counterpart.
    #[must_use]
    pub fn linear_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }
        let mut offset = 0usize;
        for ((index, bound), stride) in indices.iter().zip(&self.shape).zip(&self.strides) {
            if *index >= *bound {
                return None;
            }
            offset = offset.checked_add(index.checked_mul(*stride)?)?;
        }
        (offset < self.contents.len()).then_some(offset)
    }

    /// Wraps the `TabularData` `ShapedData.get` counterpart.
    #[must_use]
    pub fn get(&self, indices: &[usize]) -> Option<&T> {
        self.linear_index(indices)
            .and_then(|index| self.contents.get(index))
    }

    /// Wraps the `TabularData` `ShapedData.at` counterpart.
    pub fn at(&self, indices: &[usize]) -> Result<&T, TabularDataError> {
        self.get(indices).ok_or_else(|| {
            TabularDataError::InvalidArgument(format!(
                "indices {indices:?} are out of bounds for shape {:?}",
                self.shape
            ))
        })
    }
}

impl<T: fmt::Debug> fmt::Display for ShapedData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShapedData(shape={:?}, strides={:?}, contents={:?})",
            self.shape, self.strides, self.contents
        )
    }
}
