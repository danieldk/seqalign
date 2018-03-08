//! Sequence distance measures.
//!
//! This module provides some predefined sequence distance measures.

use {Measure, SeqPair};
use op::Operation;
use op::archetype;

/// Levenshtein distance.
///
/// Levenshtein distance uses the following operations:
///
/// * Insert
/// * Delete
/// * Substitute
/// * Match
#[derive(Clone, Debug)]
pub struct Levenshtein {
    ops: [LevenshteinOp; 4],
}

/// Construct a Levenshtein measure with the associated insertion, deletion,
/// and substitution cost.
impl Levenshtein {
    pub fn new(insert_cost: usize, delete_cost: usize, substitute_cost: usize) -> Self {
        use self::LevenshteinOp::*;

        Levenshtein {
            ops: [
                Insert(insert_cost),
                Delete(delete_cost),
                Match,
                Substitute(substitute_cost),
            ],
        }
    }
}

impl<T> Measure<T> for Levenshtein
where
    T: Eq,
{
    type Operation = LevenshteinOp;

    fn operations(&self) -> &[Self::Operation] {
        &self.ops
    }
}

/// Levenshtein operation with associated cost.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LevenshteinOp {
    Insert(usize),
    Delete(usize),
    Match,
    Substitute(usize),
}

impl<T> Operation<T> for LevenshteinOp
where
    T: Eq,
{
    fn backtrack(
        &self,
        seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        use self::LevenshteinOp::*;

        match *self {
            Delete(cost) => archetype::Delete(cost).backtrack(seq_pair, source_idx, target_idx),
            Insert(cost) => archetype::Insert(cost).backtrack(seq_pair, source_idx, target_idx),
            Match => archetype::Match.backtrack(seq_pair, source_idx, target_idx),
            Substitute(cost) => {
                archetype::Substitute(cost).backtrack(seq_pair, source_idx, target_idx)
            }
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &Vec<Vec<usize>>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize> {
        use self::LevenshteinOp::*;

        match *self {
            Delete(cost) => {
                archetype::Delete(cost).cost(seq_pair, cost_matrix, source_idx, target_idx)
            }
            Insert(cost) => {
                archetype::Insert(cost).cost(seq_pair, cost_matrix, source_idx, target_idx)
            }
            Match => archetype::Match.cost(seq_pair, cost_matrix, source_idx, target_idx),
            Substitute(cost) => {
                archetype::Substitute(cost).cost(seq_pair, cost_matrix, source_idx, target_idx)
            }
        }
    }
}
