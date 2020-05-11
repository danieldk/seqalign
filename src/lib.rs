//! Sequence alignment
//!
//! This crate implements commonly-used sequence alignment methods based on
//! edit operations. There are multiple crates available to compute edit
//! distances. However, to my knowledge there was no crate that supports
//! all of the following seqalign features:
//!
//! * Works on slices of any type.
//! * Can return both the edit distance and the edit script/alignment.
//! * Can be extended with new measures.
//!
//! # Example
//!
//! ```
//! use seqalign::Align;
//! use seqalign::measures::LevenshteinDamerau;
//!
//! let incorrect = &['t', 'p', 'y', 'o'];
//! let correct = &['t', 'y', 'p', 'o', 's'];
//!
//! let measure = LevenshteinDamerau::new(1, 1, 1, 1);
//! let alignment = measure.align(incorrect, correct);
//!
//! // Get the edit distance
//! assert_eq!(2, alignment.distance());
//!
//! // Get the edit script.
//! use seqalign::measures::LevenshteinDamerauOp;
//! use seqalign::op::IndexedOperation;
//!
//! assert_eq!(vec![
//!   	IndexedOperation::new(LevenshteinDamerauOp::Match, 0, 0),
//!   	IndexedOperation::new(LevenshteinDamerauOp::Transpose(1), 1, 1),
//!   	IndexedOperation::new(LevenshteinDamerauOp::Match, 3, 3),
//!   	IndexedOperation::new(LevenshteinDamerauOp::Insert(1), 4, 4)
//!   ], alignment.edit_script());
//! ```

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod dynprog;
pub use crate::dynprog::{Align, Alignment};

pub mod measures;

pub mod op;

/// Trait for edit distance measures.
pub trait Measure<T> {
    /// The edit operations associated with the measure.
    type Operation: op::Operation<T>;

    /// Get a slice with the measure's operations. Typically, this contains
    /// all the enum variants of the associated type `Operation`.
    fn operations(&self) -> &[Self::Operation];
}

/// A pairing of two sequences.
pub struct SeqPair<'a, T>
where
    T: 'a,
{
    pub source: &'a [T],
    pub target: &'a [T],
}
