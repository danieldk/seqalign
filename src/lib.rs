/// A pairing of two sequences.
pub struct SeqPair<'a, T>
where
    T: 'a,
{
    pub source: &'a [T],
    pub target: &'a [T],
}
mod dynprog;
pub use dynprog::CostMatrix;

pub mod measures;

pub mod ops;
