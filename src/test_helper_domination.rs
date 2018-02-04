// Here we define code commonly used by modules `domination` and `non_dominated_sort`.

use domination::DominationOrd;
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
