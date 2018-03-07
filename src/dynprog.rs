use SeqPair;
use ops::{EditOperation, EditOperations};

/// Edit distance cost cost_matrix.
pub struct CostMatrix<'a, T>
where
    T: 'a,
{
    pair: SeqPair<'a, T>,
    ops: &'a EditOperations<T>,
    cost_matrix: Vec<Vec<usize>>,
}

impl<'a, T> CostMatrix<'a, T> {
    /// Align two sequences.
    ///
    /// This function aligns two sequences and returns the cost cost_matrix.
    pub fn align(ops: &'a EditOperations<T>, source: &'a [T], target: &'a [T]) -> CostMatrix<'a, T> {
        let pair = SeqPair {
            source: source.as_ref(),
            target: target.as_ref(),
        };

        let source_len = pair.source.len() + 1;
        let target_len = pair.target.len() + 1;

        let mut cost_matrix = CostMatrix {
            pair,
            ops,
            cost_matrix: vec![vec![0; target_len]; source_len],
        };

        // Fill first row. This is separated from the rest of the cost_matrix fill
        // because we do not want to fill cell [0][0].
        for target_idx in 1..target_len {
            cost_matrix.cost_matrix[0][target_idx] = ops.apply(&cost_matrix, 0, target_idx)
                .expect("No applicable operation");
        }

        // Fill the cost_matrix
        for source_idx in 1..source_len {
            for target_idx in 0..target_len {
                cost_matrix.cost_matrix[source_idx][target_idx] = ops.apply(&cost_matrix, source_idx, target_idx)
                    .expect("No applicable operation");
            }
        }

        cost_matrix
    }

    /// Get the edit distance.
    pub fn distance(&self) -> usize {
        self.cost_matrix[self.cost_matrix.len() - 1][self.cost_matrix[0].len() - 1]
    }

    pub fn edit_script(&self) -> Option<Vec<&'a EditOperation<T>>> {
        let mut source_idx = self.pair.source.len();
        let mut target_idx = self.pair.target.len();
        let mut script = Vec::new();

        while let Some(op) = self.ops.backtrack(self, source_idx, target_idx) {
            let (new_source_idx, new_target_idx) = op.backtrack(source_idx, target_idx)?;
            source_idx = new_source_idx;
            target_idx = new_target_idx;
            script.push(op);

            if source_idx == 0 && target_idx == 0 {
                break;
            }
        }

        assert_eq!(source_idx, 0, "Cannot backtrack to cell 0, 0");
        assert_eq!(target_idx, 0, "Cannot backtrack to cell 0, 0");

        script.reverse();

        Some(script)
    }

    /// Get the cost cost_matrix.
    pub fn cost_matrix(&self) -> &Vec<Vec<usize>> {
        &self.cost_matrix
    }

    /// Get the sequence pair associated with this cost cost_matrix.
    pub fn seq_pair(&self) -> &SeqPair<T> {
        &self.pair
    }
}

#[cfg(test)]
mod tests {
    use ops::EditOperations;
    use measures::levenshtein;

    use super::CostMatrix;

    #[test]
    fn distance_test() {
        let applet: Vec<char> = "applet".chars().collect();
        let pineapple: Vec<char> = "pineapple".chars().collect();
        let pen: Vec<char> = "pen".chars().collect();

        assert_eq!(
            CostMatrix::align(&levenshtein(1, 1, 1), &pineapple, &pen).distance(),
            7
        );
        assert_eq!(
            CostMatrix::align(&levenshtein(1, 1, 1), &pen, &pineapple).distance(),
            7
        );
        assert_eq!(
            CostMatrix::align(&levenshtein(1, 1, 1), &pineapple, &applet).distance(),
            5
        );
        assert_eq!(
            CostMatrix::align(&levenshtein(1, 1, 1), &applet, &pen).distance(),
            4
        );
    }

    #[test]
    fn edit_script_test() {
        let applet: Vec<char> = "applet".chars().collect();
        let pineapple: Vec<char> = "pineapple".chars().collect();
        let pen: Vec<char> = "pen".chars().collect();

        let ops = levenshtein(1, 1, 1);

        assert_eq!(
            edit_script_str(&ops, &pineapple, &pen),
            vec![
                "match",
                "substitute",
                "match",
                "delete",
                "delete",
                "delete",
                "delete",
                "delete",
                "delete",
            ]
        );

        assert_eq!(
            edit_script_str(&ops, &pen, &pineapple),
            vec![
                "match",
                "substitute",
                "match",
                "insert",
                "insert",
                "insert",
                "insert",
                "insert",
                "insert",
            ]
        );

        assert_eq!(
            edit_script_str(&ops, &pineapple, &applet),
            vec![
                "delete", "delete", "delete", "delete", "match", "match", "match", "match",
                "match", "insert",
            ]
        );
    }

    fn edit_script_str<T>(ops: &EditOperations<T>, seq1: &[T], seq2: &[T]) -> Vec<String>
    where
        T: Eq,
    {
        let seq1 = seq1.as_ref();
        let seq2 = seq2.as_ref();

        CostMatrix::align(ops, seq1, seq2)
            .edit_script()
            .unwrap()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    #[test]
    fn align_empty_test() {
        let empty: &[char] = &[];
        let non_empty: Vec<char> = "hello".chars().collect();

        assert_eq!(
            CostMatrix::align(&levenshtein(1, 1, 1), empty, empty).distance(),
            0
        );
        assert_eq!(
            CostMatrix::align(&levenshtein(1, 1, 1), non_empty.as_slice(), empty).distance(),
            5
        );
        assert_eq!(
            CostMatrix::align(&levenshtein(1, 1, 1), empty, non_empty.as_slice()).distance(),
            5
        );
    }
}
