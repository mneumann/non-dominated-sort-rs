extern crate non_dominated_sort;
mod common;

use common::{create_solutions_with_n_fronts, Tuple, TupleDominationOrd};
use non_dominated_sort::non_dominated_sort;

fn get_solutions() -> Vec<Tuple> {
    vec![
        Tuple(1, 2),
        Tuple(1, 2),
        Tuple(2, 1),
        Tuple(1, 3),
        Tuple(0, 2),
    ]
}

#[test]
fn test_non_dominated_sort() {
    let solutions = get_solutions();

    let f0 = non_dominated_sort(&solutions, &TupleDominationOrd);
    assert_eq!(0, f0.rank());
    assert_eq!(&[2, 4], f0.current_front_indices());

    let f1 = f0.next_front();
    assert_eq!(1, f1.rank());
    assert_eq!(&[0, 1], f1.current_front_indices());

    let f2 = f1.next_front();
    assert_eq!(2, f2.rank());
    assert_eq!(&[3], f2.current_front_indices());

    let f3 = f2.next_front();
    assert_eq!(3, f3.rank());
    assert_eq!(true, f3.is_empty());
}

fn test_fronts(n: usize, n_fronts: usize) {
    let (solutions, expected_fronts) = create_solutions_with_n_fronts(n, n_fronts);

    let mut f = non_dominated_sort(&solutions, &TupleDominationOrd);
    for (expected_rank, expected_front) in expected_fronts.iter().enumerate() {
        assert_eq!(expected_rank, f.rank());
        assert_eq!(&expected_front[..], f.current_front_indices());
        f = f.next_front();
    }
    assert_eq!(true, f.is_empty());
}

#[test]
fn test_non_dominated_sort_1000_5() {
    test_fronts(1_000, 5);
}
