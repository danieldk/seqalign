//! String distance measures.
//!
//! This module provides some predefined string distance measures.

use ops::{Delete, EditOperations, Insert, Match, Substitute};

/// Levenshtein distance.
///
/// Levenshtein distance uses the following operations:
///
/// * Insert
/// * Delete
/// * Substitute
/// * Match
pub fn levensthein<T>(
    insert_cost: usize,
    delete_cost: usize,
    substitute_cost: usize,
) -> EditOperations<T>
where
    T: Eq,
{
    EditOperations(vec![
        Box::new(Match),
        Box::new(Insert(insert_cost)),
        Box::new(Delete(delete_cost)),
        Box::new(Substitute(substitute_cost)),
    ])
}
