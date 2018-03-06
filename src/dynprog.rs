use {IndexedOperation, Measure, Operation, SeqPair};
use op::{Backtrack, BestOperation};

/// Edit distance cost matrix.
pub struct CostMatrix<'a, M, T>
where
    M: Measure<T>,
    T: Eq + 'a,
{
    measure: M,
    pair: SeqPair<'a, T>,
    cost_matrix: Vec<Vec<usize>>,
}

impl<'a, M, T> CostMatrix<'a, M, T>
where
    M: Measure<T>,
    T: Eq,
{
    /// Align two sequences.
    ///
    /// This function aligns two sequences and returns the cost matrix.
    pub fn align(measure: M, source: &'a [T], target: &'a [T]) -> Self {
        let pair = SeqPair {
            source: source.as_ref(),
            target: target.as_ref(),
        };

        let source_len = pair.source.len() + 1;
        let target_len = pair.target.len() + 1;

        let mut cost_matrix = vec![vec![0; target_len]; source_len];

        // Fill first row. This is separated from the rest of the matrix fill
        // because we do not want to fill cell [0][0].
        for target_idx in 1..target_len {
            cost_matrix[0][target_idx] = measure
                .best_operation(&pair, &cost_matrix, 0, target_idx)
                .expect("No applicable operation")
                .1;
        }

        // Fill the matrix
        for source_idx in 1..source_len {
            for target_idx in 0..target_len {
                cost_matrix[source_idx][target_idx] = measure
                    .best_operation(&pair, &cost_matrix, source_idx, target_idx)
                    .expect("No applicable operation")
                    .1;
            }
        }

        CostMatrix {
            measure,
            pair,
            cost_matrix,
        }
    }

    /// Get the edit distance.
    pub fn distance(&self) -> usize {
        self.cost_matrix[self.cost_matrix.len() - 1][self.cost_matrix[0].len() - 1]
    }

    /// Return the script of edit operations to rewrite the source sequence
    /// to the target sequence.
    pub fn edit_script(&self) -> Vec<IndexedOperation<M::Operation>> {
        let mut source_idx = self.pair.source.len();
        let mut target_idx = self.pair.target.len();
        let mut script = Vec::new();

        while let Some(op) =
            self.measure
                .backtrack(&self.pair, &self.cost_matrix, source_idx, target_idx)
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
    pub fn cost_matrix(&self) -> &Vec<Vec<usize>> {
        &self.cost_matrix
    }

    /// Get the sequence pair associated with this cost matrix.
    pub fn seq_pair(&self) -> &SeqPair<T> {
        &self.pair
    }
}

#[cfg(test)]
mod tests {
    use IndexedOperation;
    use measures::Levenshtein;
    use measures::LevenshteinOp::*;

    use super::CostMatrix;

    #[test]
    fn distance_test() {
        let applet: Vec<char> = "applet".chars().collect();
        let pineapple: Vec<char> = "pineapple".chars().collect();
        let pen: Vec<char> = "pen".chars().collect();

        assert_eq!(
            CostMatrix::align(Levenshtein::new(1, 1, 1), &pineapple, &pen).distance(),
            7
        );
        assert_eq!(
            CostMatrix::align(Levenshtein::new(1, 1, 1), &pen, &pineapple).distance(),
            7
        );
        assert_eq!(
            CostMatrix::align(Levenshtein::new(1, 1, 1), &pineapple, &applet).distance(),
            5
        );
        assert_eq!(
            CostMatrix::align(Levenshtein::new(1, 1, 1), &applet, &pen).distance(),
            4
        );
    }

    #[test]
    fn edit_script_test() {
        let applet: Vec<char> = "applet".chars().collect();
        let pineapple: Vec<char> = "pineapple".chars().collect();
        let pen: Vec<char> = "pen".chars().collect();

        let ops = Levenshtein::new(1, 1, 1);

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
            CostMatrix::align(ops.clone(), &pineapple, &pen).edit_script()
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
            CostMatrix::align(ops.clone(), &pen, &pineapple).edit_script()
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
            CostMatrix::align(ops.clone(), &pineapple, &applet).edit_script()
        );
    }

    #[test]
    fn align_empty_test() {
        let empty: &[char] = &[];
        let non_empty: Vec<char> = "hello".chars().collect();

        assert_eq!(
            CostMatrix::align(Levenshtein::new(1, 1, 1), empty, empty).distance(),
            0
        );
        assert_eq!(
            CostMatrix::align(Levenshtein::new(1, 1, 1), non_empty.as_slice(), empty).distance(),
            5
        );
        assert_eq!(
            CostMatrix::align(Levenshtein::new(1, 1, 1), empty, non_empty.as_slice()).distance(),
            5
        );
    }
}
