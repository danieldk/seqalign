//! Sequence distance measures.
//!
//! This module provides some predefined sequence distance measures.

use ndarray::ArrayView2;

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
        cost_matrix: ArrayView2<usize>,
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

/// Levenshtein-Damerau distance.
///
/// Levenshtein-Damerau distance uses the following operations:
///
/// * Insert
/// * Delete
/// * Substitute
/// * Match
/// * Transpose (*xy* -> *yx*)
#[derive(Clone, Debug)]
pub struct LevenshteinDamerau {
    ops: [LevenshteinDamerauOp; 5],
}

/// Construct a Levenshtein-Damerau measure with the associated insertion,
/// deletion, substitution, and transposition cost.
impl LevenshteinDamerau {
    pub fn new(
        insert_cost: usize,
        delete_cost: usize,
        substitute_cost: usize,
        transpose_cost: usize,
    ) -> Self {
        use self::LevenshteinDamerauOp::*;

        LevenshteinDamerau {
            ops: [
                Insert(insert_cost),
                Delete(delete_cost),
                Match,
                Substitute(substitute_cost),
                Transpose(transpose_cost),
            ],
        }
    }
}

impl<T> Measure<T> for LevenshteinDamerau
where
    T: Eq,
{
    type Operation = LevenshteinDamerauOp;

    fn operations(&self) -> &[Self::Operation] {
        &self.ops
    }
}

/// Levenshtein operation with associated cost.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LevenshteinDamerauOp {
    Insert(usize),
    Delete(usize),
    Match,
    Substitute(usize),
    Transpose(usize),
}

impl<T> Operation<T> for LevenshteinDamerauOp
where
    T: Eq,
{
    fn backtrack(
        &self,
        seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        use self::LevenshteinDamerauOp::*;

        match *self {
            Delete(cost) => archetype::Delete(cost).backtrack(seq_pair, source_idx, target_idx),
            Insert(cost) => archetype::Insert(cost).backtrack(seq_pair, source_idx, target_idx),
            Match => archetype::Match.backtrack(seq_pair, source_idx, target_idx),
            Substitute(cost) => {
                archetype::Substitute(cost).backtrack(seq_pair, source_idx, target_idx)
            }
            Transpose(cost) => {
                archetype::Transpose(cost).backtrack(seq_pair, source_idx, target_idx)
            }
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: ArrayView2<usize>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize> {
        use self::LevenshteinDamerauOp::*;

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
            Transpose(cost) => {
                archetype::Transpose(cost).cost(seq_pair, cost_matrix, source_idx, target_idx)
            }
        }
    }
}

/// Longest common subsequence (LCS) alignment.
///
/// This measure uses the following edit operations:
///
/// * Insert
/// * Delete
/// * Match
///
/// The matches in edit script for this measure give a longest common
/// subsequence. The cost is the number of insertions/deletions after
/// aligning the LCSes.
#[derive(Clone, Debug)]
pub struct LCS {
    ops: [LCSOp; 3],
}

/// Construct LCS measure with the associated insertion and deletion
/// cost.
impl LCS {
    pub fn new(insert_cost: usize, delete_cost: usize) -> Self {
        use self::LCSOp::*;

        LCS {
            ops: [Insert(insert_cost), Delete(delete_cost), Match],
        }
    }
}

impl<T> Measure<T> for LCS
where
    T: Eq,
{
    type Operation = LCSOp;

    fn operations(&self) -> &[Self::Operation] {
        &self.ops
    }
}

/// Levenshtein operation with associated cost.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LCSOp {
    Insert(usize),
    Delete(usize),
    Match,
}

impl<T> Operation<T> for LCSOp
where
    T: Eq,
{
    fn backtrack(
        &self,
        seq_pair: &SeqPair<T>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<(usize, usize)> {
        use self::LCSOp::*;

        match *self {
            Delete(cost) => archetype::Delete(cost).backtrack(seq_pair, source_idx, target_idx),
            Insert(cost) => archetype::Insert(cost).backtrack(seq_pair, source_idx, target_idx),
            Match => archetype::Match.backtrack(seq_pair, source_idx, target_idx),
        }
    }

    fn cost(
        &self,
        seq_pair: &SeqPair<T>,
        cost_matrix: ArrayView2<usize>,
        source_idx: usize,
        target_idx: usize,
    ) -> Option<usize> {
        use self::LCSOp::*;

        match *self {
            Delete(cost) => {
                archetype::Delete(cost).cost(seq_pair, cost_matrix, source_idx, target_idx)
            }
            Insert(cost) => {
                archetype::Insert(cost).cost(seq_pair, cost_matrix, source_idx, target_idx)
            }
            Match => archetype::Match.cost(seq_pair, cost_matrix, source_idx, target_idx),
        }
    }
}

#[cfg(test)]
mod tests {
    use Measure;
    use measures::{Levenshtein, LevenshteinDamerau, LCS};

    use Align;

    struct TestCase {
        source: &'static str,
        target: &'static str,
        levenshtein_dist: usize,
        levenshtein_damerau_dist: usize,
        lcs_dist: usize,
    }

    impl TestCase {
        fn new(
            source: &'static str,
            target: &'static str,
            levenshtein_dist: usize,
            levenshtein_damerau_dist: usize,
            lcs_dist: usize,
        ) -> Self {
            TestCase {
                source,
                target,
                levenshtein_dist,
                levenshtein_damerau_dist,
                lcs_dist,
            }
        }
    }

    lazy_static! {
        static ref TESTCASES: Vec<TestCase> = vec![
            TestCase::new("pineapple", "", 9, 9, 9),
            TestCase::new("", "pineapple", 9, 9, 9),
            TestCase::new("pineapple", "pen", 7, 7, 8),
            TestCase::new("pen", "pineapple", 7, 7, 8),
            TestCase::new("pineapple", "applet", 5, 5, 5),
            TestCase::new("applet", "pen", 4, 4, 5),
            TestCase::new("tpyo", "typo", 2, 1, 2),
        ];
    }

    #[test]
    pub fn test_lcs() {
        run_testcases(|| LCS::new(1, 1), |testcase| testcase.lcs_dist);
    }

    #[test]
    pub fn test_levenshtein() {
        run_testcases(
            || Levenshtein::new(1, 1, 1),
            |testcase| testcase.levenshtein_dist,
        );
    }

    #[test]
    pub fn test_levenshtein_damerau() {
        run_testcases(
            || LevenshteinDamerau::new(1, 1, 1, 1),
            |testcase| testcase.levenshtein_damerau_dist,
        );
    }

    fn run_testcases<MF, M, DF>(measure: MF, distance: DF)
    where
        MF: Fn() -> M,
        M: Measure<char>,
        DF: Fn(&TestCase) -> usize,
    {
        for testcase in TESTCASES.iter() {
            let source: Vec<char> = testcase.source.chars().collect();
            let target: Vec<char> = testcase.target.chars().collect();
            let measure = measure();
            assert_eq!(
                distance(testcase),
                measure.align(&source, &target).distance()
            )
        }
    }

}
