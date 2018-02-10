extern crate non_dominated_sort;
mod common;

use std::cmp::Ordering;
use common::{Tuple, TupleDominationOrd};
use non_dominated_sort::DominationOrd;

#[test]
fn test_non_domination() {
    let a = &Tuple(1, 2);
    let b = &Tuple(2, 1);

    // Non-domination due to reflexitivity
    assert_eq!(Ordering::Equal, TupleDominationOrd.domination_ord(a, a));
    assert_eq!(Ordering::Equal, TupleDominationOrd.domination_ord(b, b));

    // Non-domination
    assert_eq!(Ordering::Equal, TupleDominationOrd.domination_ord(a, b));
    assert_eq!(Ordering::Equal, TupleDominationOrd.domination_ord(b, a));
}

#[test]
fn test_domination() {
    let a = &Tuple(1, 2);
    let b = &Tuple(1, 3);
    let c = &Tuple(0, 2);

    // a < b
    assert_eq!(Ordering::Less, TupleDominationOrd.domination_ord(a, b));
    // c < a
    assert_eq!(Ordering::Less, TupleDominationOrd.domination_ord(c, a));
    // transitivity => c < b
    assert_eq!(Ordering::Less, TupleDominationOrd.domination_ord(c, b));

    // Just reverse the relation: forall a, b: a < b => b > a

    // b > a
    assert_eq!(Ordering::Greater, TupleDominationOrd.domination_ord(b, a));
    // a > c
    assert_eq!(Ordering::Greater, TupleDominationOrd.domination_ord(a, c));
    // transitivity => b > c
    assert_eq!(Ordering::Greater, TupleDominationOrd.domination_ord(b, c));
}
