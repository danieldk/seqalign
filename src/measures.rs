//! Sequence distance measures.
//!
//! This module provides some predefined sequence distance measures.

use {Measure, Operation, SeqPair};

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
        _seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        use self::LevenshteinOp::*;

        match *self {
            Delete(_) => if source_idx > 0 {
                Some((source_idx - 1, target_idx))
            } else {
                None
            },
            Insert(_) => if target_idx > 0 {
                Some((source_idx, target_idx - 1))
            } else {
                None
            },
            Match => if source_idx > 0 && target_idx > 0 {
                Some((source_idx - 1, target_idx - 1))
            } else {
                None
            },
            Substitute(_) => if source_idx > 0 && target_idx > 0 {
                Some((source_idx - 1, target_idx - 1))
            } else {
                None
            },
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

        let (from_source_idx, from_target_idx) = self.backtrack(seq_pair, source_idx, target_idx)?;
        let orig_cost = cost_matrix[from_source_idx][from_target_idx];

        match *self {
            Delete(cost) => Some(orig_cost + cost),
            Insert(cost) => Some(orig_cost + cost),
            Match => {
                if seq_pair.source[from_source_idx] == seq_pair.target[from_target_idx] {
                    Some(orig_cost)
                } else {
                    None
                }
            }
            Substitute(cost) => Some(orig_cost + cost),
        }
    }
}
