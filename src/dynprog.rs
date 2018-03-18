use ndarray::{Array2, ArrayView2};

use {Measure, SeqPair};
use op::{Backtrack, BestCost, IndexedOperation, Operation};

/// Trait enabling alignment of all `Measure`s.
///
/// This trait is used to implement alignment using dynamic programming
/// for every type that implements the `Measure` trait.
pub trait Align<'a, M, T>
where
    M: Measure<T>,
    T: Eq,
{
    /// Align two sequences.
    ///
    /// This function aligns two sequences and returns the alignment.
    fn align(&'a self, source: &'a [T], target: &'a [T]) -> Alignment<'a, M, T>;
}

impl<'a, M, T> Align<'a, M, T> for M
where
    M: Measure<T>,
    T: Eq,
{
    fn align(&'a self, source: &'a [T], target: &'a [T]) -> Alignment<'a, M, T> {
        let pair = SeqPair {
            source: source.as_ref(),
            target: target.as_ref(),
        };

        let source_len = pair.source.len() + 1;
        let target_len = pair.target.len() + 1;

        let mut cost_matrix = Array2::zeros((source_len, target_len));

        // Fill first row. This is separated from the rest of the matrix fill
        // because we do not want to fill cell [0][0].
        for target_idx in 1..target_len {
            cost_matrix[(0, target_idx)] = self.best_cost(&pair, cost_matrix.view(), 0, target_idx)
                .expect("No applicable operation");
        }

        // Fill the matrix
        for source_idx in 1..source_len {
            for target_idx in 0..target_len {
                cost_matrix[(source_idx, target_idx)] =
                    self.best_cost(&pair, cost_matrix.view(), source_idx, target_idx)
                        .expect("No applicable operation");
            }
        }

        Alignment {
            measure: self,
            pair,
            cost_matrix,
        }
    }
}

/// Edit distance cost matrix.
pub struct Alignment<'a, M, T>
where
    M: 'a + Measure<T>,
    T: 'a + Eq,
{
    measure: &'a M,
    pair: SeqPair<'a, T>,
    cost_matrix: Array2<usize>,
}

impl<'a, M, T> Alignment<'a, M, T>
where
    M: Measure<T>,
    T: Eq,
{
    /// Get the edit distance.
    pub fn distance(&self) -> usize {
        let shape = self.cost_matrix.shape();
        self.cost_matrix[(shape[0] - 1, shape[1] - 1)]
    }

    /// Return the script of edit operations to rewrite the source sequence
    /// to the target sequence.
    pub fn edit_script(&self) -> Vec<IndexedOperation<M::Operation>> {
        let mut source_idx = self.pair.source.len();
        let mut target_idx = self.pair.target.len();
        let mut script = Vec::new();

        while let Some(op) =
            self.measure
                .backtrack(&self.pair, self.cost_matrix.view(), source_idx, target_idx)
        {
            let (new_source_idx, new_target_idx) = op.backtrack(&self.pair, source_idx, target_idx)
                .expect("Cannot backtrack");
            source_idx = new_source_idx;
            target_idx = new_target_idx;

            script.push(IndexedOperation::new(op, source_idx, target_idx));

            if source_idx == 0 && target_idx == 0 {
                break;
            }
        }

        assert_eq!(source_idx, 0, "Cannot backtrack to cell 0, 0");
        assert_eq!(target_idx, 0, "Cannot backtrack to cell 0, 0");

        script.reverse();

        script
    }

    /// Get the cost matrix.
    pub fn cost_matrix(&self) -> ArrayView2<usize> {
        self.cost_matrix.view()
    }

    /// Get the sequence pair associated with this cost matrix.
    pub fn seq_pair(&self) -> &SeqPair<T> {
        &self.pair
    }
}

#[cfg(test)]
mod tests {
    use op::IndexedOperation;
    use measures::Levenshtein;
    use measures::LevenshteinOp::*;

    use super::Align;

    #[test]
    fn distance_test() {
        let applet: Vec<char> = "applet".chars().collect();
        let pineapple: Vec<char> = "pineapple".chars().collect();
        let pen: Vec<char> = "pen".chars().collect();

        let levenshtein = Levenshtein::new(1, 1, 1);

        assert_eq!(levenshtein.align(&pineapple, &pen).distance(), 7);
        assert_eq!(levenshtein.align(&pen, &pineapple).distance(), 7);
        assert_eq!(levenshtein.align(&pineapple, &applet).distance(), 5);
        assert_eq!(levenshtein.align(&applet, &pen).distance(), 4);
    }

    #[test]
    fn edit_script_test() {
        let applet: Vec<char> = "applet".chars().collect();
        let pineapple: Vec<char> = "pineapple".chars().collect();
        let pen: Vec<char> = "pen".chars().collect();

        let levenshtein = Levenshtein::new(1, 1, 1);

        assert_eq!(
            vec![
                IndexedOperation::new(Match, 0, 0),
                IndexedOperation::new(Substitute(1), 1, 1),
                IndexedOperation::new(Match, 2, 2),
                IndexedOperation::new(Delete(1), 3, 3),
                IndexedOperation::new(Delete(1), 4, 3),
                IndexedOperation::new(Delete(1), 5, 3),
                IndexedOperation::new(Delete(1), 6, 3),
                IndexedOperation::new(Delete(1), 7, 3),
                IndexedOperation::new(Delete(1), 8, 3),
            ],
            levenshtein.align(&pineapple, &pen).edit_script()
        );

        assert_eq!(
            vec![
                IndexedOperation::new(Match, 0, 0),
                IndexedOperation::new(Substitute(1), 1, 1),
                IndexedOperation::new(Match, 2, 2),
                IndexedOperation::new(Insert(1), 3, 3),
                IndexedOperation::new(Insert(1), 3, 4),
                IndexedOperation::new(Insert(1), 3, 5),
                IndexedOperation::new(Insert(1), 3, 6),
                IndexedOperation::new(Insert(1), 3, 7),
                IndexedOperation::new(Insert(1), 3, 8),
            ],
            levenshtein.align(&pen, &pineapple).edit_script()
        );

        assert_eq!(
            vec![
                IndexedOperation::new(Delete(1), 0, 0),
                IndexedOperation::new(Delete(1), 1, 0),
                IndexedOperation::new(Delete(1), 2, 0),
                IndexedOperation::new(Delete(1), 3, 0),
                IndexedOperation::new(Match, 4, 0),
                IndexedOperation::new(Match, 5, 1),
                IndexedOperation::new(Match, 6, 2),
                IndexedOperation::new(Match, 7, 3),
                IndexedOperation::new(Match, 8, 4),
                IndexedOperation::new(Insert(1), 9, 5),
            ],
            levenshtein.align(&pineapple, &applet).edit_script()
        );
    }

    #[test]
    fn align_empty_test() {
        let empty: &[char] = &[];
        let non_empty: Vec<char> = "hello".chars().collect();

        let levenshtein = Levenshtein::new(1, 1, 1);

        assert_eq!(levenshtein.align(empty, empty).distance(), 0);
        assert_eq!(levenshtein.align(non_empty.as_slice(), empty).distance(), 5);
        assert_eq!(levenshtein.align(empty, non_empty.as_slice()).distance(), 5);
    }
}
