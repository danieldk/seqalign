//! Edit operations.

use std::cmp;
use std::fmt::Debug;
use std::hash::Hash;

use crate::{Measure, SeqPair};

pub mod archetype;

/// Trait for sequence edit operations.
pub trait Operation<T>: Clone + Debug + Eq + Hash {
    /// Return the cell after backtracking from the given cell with this operation.
    ///
    /// Must return `None` if backtracking is not possible (e.g. would lead
    /// to an invalid cell). This method is used for the construction of
    /// traces and edit scripts.
    fn backtrack(
        &self,
        seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)>;

    /// Compute the cost after applying the operation.
    ///
    /// Returns `None` if the operation cannot be applied. Otherwise, it
    /// returns the cost for the alignment at `source_idx`, `target_idx`
    /// using this operation.
    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &[Vec<usize>],
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq;
}

///An indexed edit operation.
///
/// Indexed edit operations are a pairing of an edit operation and the
/// sequence positions when/where the operation was applied. The indexes
/// can be used to simplify external use of the operations. For example,
/// if we are interested in which elements were aligned, then a
/// subsequence of matches
///
/// * *match 1 2*
/// * *match 3 7*
/// * *match 4 10*
///
/// Tells us that indices 1/2, 3/7, and 4/10 of the source/target sequence
/// were aligned.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IndexedOperation<O>
where
    O: Debug,
{
    operation: O,
    source_idx: usize,
    target_idx: usize,
}

impl<O> IndexedOperation<O>
where
    O: Debug,
{
    pub fn new(operation: O, source_idx: usize, target_idx: usize) -> Self {
        IndexedOperation {
            operation,
            source_idx,
            target_idx,
        }
    }

    pub fn operation(&self) -> &O {
        &self.operation
    }

    pub fn source_idx(&self) -> usize {
        self.source_idx
    }

    pub fn target_idx(&self) -> usize {
        self.target_idx
    }
}

pub(crate) trait Backtrack<T> {
    type Operation: Operation<T>;

    fn backtrack(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &[Vec<usize>],
        source_idx: usize,
        target_idx: usize,
    ) -> Option<Self::Operation>
    where
        T: Eq;

    fn backtracks(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &[Vec<usize>],
        source_idx: usize,
        target_idx: usize,
    ) -> Vec<Self::Operation>
    where
        T: Eq;
}

impl<M, T> Backtrack<T> for M
where
    M: Measure<T>,
{
    type Operation = M::Operation;

    /// Give the operation that was used to construct the cost matrix cell
    /// at (`source_idx`, `target_idx`).
    fn backtrack(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &[Vec<usize>],
        source_idx: usize,
        target_idx: usize,
    ) -> Option<Self::Operation>
    where
        T: Eq,
    {
        for op in self.operations() {
            if let Some(cost) = op.cost(seq_pair, cost_matrix, source_idx, target_idx) {
                if cost == cost_matrix[source_idx][target_idx] {
                    return Some(op.clone());
                }
            }
        }

        None
    }

    /// Give the operations that were used to construct the cost matrix cell
    /// at (`source_idx`, `taget_idx`).
    fn backtracks(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &[Vec<usize>],
        source_idx: usize,
        target_idx: usize,
    ) -> Vec<Self::Operation>
    where
        T: Eq,
    {
        let mut ops = Vec::new();

        for op in self.operations() {
            if let Some(cost) = op.cost(seq_pair, cost_matrix, source_idx, target_idx) {
                if cost == cost_matrix[source_idx][target_idx] {
                    ops.push(op.clone());
                }
            }
        }

        ops
    }
}

pub(crate) trait BestCost<T> {
    type Operation: Operation<T>;

    fn best_cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &[Vec<usize>],
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq;
}

impl<M, T> BestCost<T> for M
where
    M: Measure<T>,
{
    type Operation = M::Operation;

    /// Compute the cost of the best operation.
    ///
    /// Returns `None` if the operation cannot be applied. Otherwise, it
    /// returns the cost for the alignment at `source_idx`, `target_idx`
    /// using this operation.
    fn best_cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &[Vec<usize>],
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq,
    {
        // Used filter_map + min_by_key before, but the compiler is not able
        // to optimize to a better loop.
        let mut best = None;
        for op in self.operations() {
            if let Some(cost) = op.cost(seq_pair, cost_matrix, source_idx, target_idx) {
                best = best
                    .map(|best_cost| cmp::min(cost, best_cost))
                    .or(Some(cost))
            }
        }

        best
    }
}
