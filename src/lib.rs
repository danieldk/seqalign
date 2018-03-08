#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod dynprog;
pub use dynprog::Alignment;

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
