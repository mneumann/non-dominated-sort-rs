// Here we define code commonly used by tests

use non_dominated_sort::{DominationOrd, Front};
use std::cmp::Ordering;

// Our multi-variate fitness/solution value
pub struct Tuple(pub usize, pub usize);

// We can have multiple dominance relations defined on a single
// type, without having to wrap the "Tuple" itself.
pub struct TupleDominationOrd;

impl DominationOrd for TupleDominationOrd {
    type Solution = Tuple;

    fn domination_ord(&self, a: &Self::Solution, b: &Self::Solution) -> Ordering {
        if a.0 < b.0 && a.1 <= b.1 {
            Ordering::Less
        } else if a.0 <= b.0 && a.1 < b.1 {
            Ordering::Less
        } else if a.0 > b.0 && a.1 >= b.1 {
            Ordering::Greater
        } else if a.0 >= b.0 && a.1 > b.1 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

// Create `n_fronts` with each having `n` solutions in it.
pub fn create_solutions_with_n_fronts(n: usize, n_fronts: usize) -> (Vec<Tuple>, Vec<Vec<usize>>) {
    let mut solutions = Vec::with_capacity(n * n_fronts);
    let mut expected_fronts = Vec::with_capacity(n_fronts);

    for front in 0..n_fronts {
        let mut current_front = Vec::with_capacity(n);
        for i in 0..n {
            solutions.push(Tuple(front + i, front + n - i));
            current_front.push(front * n + i);
        }
        expected_fronts.push(current_front);
    }

    return (solutions, expected_fronts);
}

pub fn assert_front_eq(expected_rank: usize, expected_indices: &[usize], front: &Front) {
    assert_eq!(expected_rank, front.rank);
    assert_eq!(expected_indices.len(), front.solutions.len());
    let mut front_indices = front.solutions_indices_only();
    front_indices.sort();
    assert_eq!(expected_indices, &front_indices[..]);
}
