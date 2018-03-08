mod dynprog;
pub use dynprog::CostMatrix;

pub mod measures;

mod op;
pub use op::{IndexedOperation, Operation};

/// Trait for edit distance measures.
pub trait Measure<T> {
    /// The edit operations associated with the measure.
    type Operation: Operation<T>;

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
