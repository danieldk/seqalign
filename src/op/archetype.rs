//! Archetypal edit operations.
//!
//! This module provides archetypal edit operations. These operations are
//! not meant to be used directly, but can be used in the implementation
//! of new measures.

use op::Operation;
use SeqPair;

/// Delete operation with associated cost.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delete(pub usize);

impl<T> Operation<T> for Delete {
    fn backtrack(
        &self,
        _seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        if source_idx > 0 {
            Some((source_idx - 1, target_idx))
        } else {
            None
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &Vec<Vec<usize>>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq,
    {
        let (from_source_idx, from_target_idx) = self.backtrack(seq_pair, source_idx, target_idx)?;
        let orig_cost = cost_matrix[from_source_idx][from_target_idx];
        Some(orig_cost + self.0)
    }
}

/// Insert operation with associated cost.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Insert(pub usize);

impl<T> Operation<T> for Insert {
    fn backtrack(
        &self,
        _seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        if target_idx > 0 {
            Some((source_idx, target_idx - 1))
        } else {
            None
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &Vec<Vec<usize>>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq,
    {
        let (from_source_idx, from_target_idx) = self.backtrack(seq_pair, source_idx, target_idx)?;
        let orig_cost = cost_matrix[from_source_idx][from_target_idx];
        Some(orig_cost + self.0)
    }
}

/// Match operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Match;

impl<T> Operation<T> for Match {
    fn backtrack(
        &self,
        _seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        if source_idx > 0 && target_idx > 0 {
            Some((source_idx - 1, target_idx - 1))
        } else {
            None
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &Vec<Vec<usize>>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq,
    {
        let (from_source_idx, from_target_idx) = self.backtrack(seq_pair, source_idx, target_idx)?;
        let orig_cost = cost_matrix[from_source_idx][from_target_idx];

        if seq_pair.source[from_source_idx] == seq_pair.target[from_target_idx] {
            Some(orig_cost)
        } else {
            None
        }
    }
}

/// Substitute operation with associated cost.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Substitute(pub usize);

impl<T> Operation<T> for Substitute {
    fn backtrack(
        &self,
        _seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        if source_idx > 0 && target_idx > 0 {
            Some((source_idx - 1, target_idx - 1))
        } else {
            None
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &Vec<Vec<usize>>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq,
    {
        let (from_source_idx, from_target_idx) = self.backtrack(seq_pair, source_idx, target_idx)?;
        let orig_cost = cost_matrix[from_source_idx][from_target_idx];
        Some(orig_cost + self.0)
    }
}

/// Transpose operation with associated cost.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transpose(pub usize);

impl<T> Operation<T> for Transpose {
    fn backtrack(
        &self,
        _seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        if source_idx >= 2 && target_idx >= 2 {
            Some((source_idx - 2, target_idx - 2))
        } else {
            None
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: &Vec<Vec<usize>>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize>
    where
        T: Eq,
    {
        let (from_source_idx, from_target_idx) = self.backtrack(seq_pair, source_idx, target_idx)?;
        let orig_cost = cost_matrix[from_source_idx][from_target_idx];

        if seq_pair.source[from_source_idx] == seq_pair.target[from_target_idx + 1]
            && seq_pair.source[from_source_idx + 1] == seq_pair.target[from_target_idx]
        {
            Some(orig_cost + self.0)
        } else {
            None
        }
    }
}
