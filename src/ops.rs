//! Edit operations.

use std::fmt;
use std::usize;

use Matrix;

/// Trait for sequence edit operations.
pub trait EditOperation<T>: fmt::Debug + fmt::Display {
    /// Compute the cost after applying the operation.
    ///
    /// Returns `None` if the operation cannot be applied. Otherwise, it
    /// returns the cost for the alignment at `source_idx`, `target_idx`
    /// using this operation.
    fn apply(&self, matrix: &Matrix<T>, source_idx: usize, target_idx: usize) -> Option<usize>;

    /// Return the matrix cell after backtracking from this operation.
    ///
    /// Must return `None` if backtracking is not possible (e.g. would lead
    /// to an invalid cell). This method is used for the construction of
    /// traces and edit scripts.
    fn backtrack(&self, source_idx: usize, target_idx: usize) -> Option<(usize, usize)>;
}

/// Delete operation.
#[derive(Debug)]
pub struct Delete(pub usize);

impl<T> EditOperation<T> for Delete {
    fn apply(&self, matrix: &Matrix<T>, source_idx: usize, target_idx: usize) -> Option<usize> {
        if source_idx > 0 {
            Some(matrix.matrix()[source_idx - 1][target_idx] + self.0)
        } else {
            None
        }
    }

    fn backtrack(&self, source_idx: usize, target_idx: usize) -> Option<(usize, usize)> {
        if source_idx > 0 {
            Some((source_idx - 1, target_idx))
        } else {
            None
        }
    }
}

impl fmt::Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "delete")
    }
}

/// Insert operation.
#[derive(Debug)]
pub struct Insert(pub usize);

impl<T> EditOperation<T> for Insert {
    fn apply(&self, matrix: &Matrix<T>, source_idx: usize, target_idx: usize) -> Option<usize> {
        if target_idx > 0 {
            Some(matrix.matrix()[source_idx][target_idx - 1] + self.0)
        } else {
            None
        }
    }

    fn backtrack(&self, source_idx: usize, target_idx: usize) -> Option<(usize, usize)> {
        if target_idx > 0 {
            Some((source_idx, target_idx - 1))
        } else {
            None
        }
    }
}

impl fmt::Display for Insert {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "insert")
    }
}

/// Match operation.
#[derive(Debug)]
pub struct Match;

impl<T> EditOperation<T> for Match
where
    T: Eq,
{
    fn apply(&self, matrix: &Matrix<T>, source_idx: usize, target_idx: usize) -> Option<usize> {
        if source_idx > 0 && target_idx > 0 {
            if matrix.seq_pair().source[source_idx - 1] == matrix.seq_pair().target[target_idx - 1]
            {
                return Some(matrix.matrix()[source_idx - 1][target_idx - 1]);
            }
        }

        None
    }

    fn backtrack(&self, source_idx: usize, target_idx: usize) -> Option<(usize, usize)> {
        if source_idx > 0 && target_idx > 0 {
            Some((source_idx - 1, target_idx - 1))
        } else {
            None
        }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "match")
    }
}

/// Substitution operation.
#[derive(Debug)]
pub struct Substitute(pub usize);

impl<T> EditOperation<T> for Substitute {
    fn apply(&self, matrix: &Matrix<T>, source_idx: usize, target_idx: usize) -> Option<usize> {
        if source_idx > 0 && target_idx > 0 {
            Some(matrix.matrix()[source_idx - 1][target_idx - 1] + self.0)
        } else {
            None
        }
    }

    fn backtrack(&self, source_idx: usize, target_idx: usize) -> Option<(usize, usize)> {
        if source_idx > 0 && target_idx > 0 {
            Some((source_idx - 1, target_idx - 1))
        } else {
            None
        }
    }
}

impl fmt::Display for Substitute {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "substitute")
    }
}

/// A list of edit operations.
///
/// This is a wrapper around a `Vec` of `EditOperation`s that provides some
/// convenience methods.
#[derive(Debug)]
pub struct EditOperations<T>(pub Vec<Box<EditOperation<T>>>);

impl<T> EditOperations<T> {
    /// Apply the best edit operation for the given cell.
    pub fn apply(&self, matrix: &Matrix<T>, source_idx: usize, target_idx: usize) -> Option<usize> {
        self.0
            .iter()
            .filter_map(|op| op.apply(matrix, source_idx, target_idx))
            .min()
    }

    /// Return the edit operation that can be used to backtrack from the
    /// given cell.
    pub fn backtrack(
        &self,
        matrix: &Matrix<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<&EditOperation<T>> {
        for op in &self.0 {
            if let Some(cost) = op.apply(matrix, source_idx, target_idx) {
                if cost == matrix.matrix()[source_idx][target_idx] {
                    return Some(op.as_ref());
                }
            }
        }

        None
    }
}
