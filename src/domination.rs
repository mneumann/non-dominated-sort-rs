use std::cmp::Ordering;

/// The dominance relation between two solutions. There are three possible situations:
///
/// - Either solution `a` dominates solution `b` ("a < b")
/// - Or solution `b` dominates solution `a` ("a > b")
/// - Or neither solution `a` nor `b` dominates each other ("a == b")
///
pub trait DominationOrd {
    /// The solution value type on which the dominance relation is defined.
    type Solution;

    /// Returns the domination order.
    fn domination_ord(&self, a: &Self::Solution, b: &Self::Solution) -> Ordering {
        if self.dominates(a, b) {
            Ordering::Less
        } else if self.dominates(b, a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Returns true if `a` dominates `b` ("a < b").
    fn dominates(&self, a: &Self::Solution, b: &Self::Solution) -> bool {
        match self.domination_ord(a, b) {
            Ordering::Less => true,
            _ => false,
        }
    }
}
