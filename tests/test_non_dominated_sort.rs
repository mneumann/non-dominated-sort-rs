extern crate non_dominated_sort;
mod common;

use common::{assert_front_eq, create_solutions_with_n_fronts, Tuple, TupleDominationOrd};
use non_dominated_sort::{non_dominated_sort, NonDominatedSort};

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
    let fronts = NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts();

    assert_eq!(3, fronts.len());
    assert_front_eq(0, &[2, 4], &fronts[0]);
    assert_front_eq(1, &[0, 1], &fronts[1]);
    assert_front_eq(2, &[3], &fronts[2]);
}

#[test]
fn test_non_dominated_sort_stop_at() {
    let solutions = get_solutions();

    {
        let fronts =
            NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts_stop_at(2);
        assert_eq!(1, fronts.len());
        assert_front_eq(0, &[2, 4], &fronts[0]);
    }
    {
        let fronts =
            NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts_stop_at(3);
        assert_eq!(2, fronts.len());
        assert_front_eq(0, &[2, 4], &fronts[0]);
        assert_front_eq(1, &[0, 1], &fronts[1]);
    }
}

#[test]
fn test_non_dominated_sort_iter() {
    let solutions = get_solutions();
    let mut fronts = NonDominatedSort::new(&solutions, &TupleDominationOrd);

    {
        let f0 = fronts.next();
        assert!(f0.is_some());
        assert_front_eq(0, &[2, 4], &f0.unwrap());
    }
    {
        let f1 = fronts.next();
        assert!(f1.is_some());
        assert_front_eq(1, &[0, 1], &f1.unwrap());
    }
    {
        let f2 = fronts.next();
        assert!(f2.is_some());
        assert_front_eq(2, &[3], &f2.unwrap());
    }
    {
        let f3 = fronts.next();
        assert!(f3.is_none());
    }
}

fn test_fronts(n: usize, n_fronts: usize) {
    let (solutions, expected_fronts) = create_solutions_with_n_fronts(n, n_fronts);

    let fronts = NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts();

    assert_eq!(n_fronts, fronts.len());

    for front in 0..n_fronts {
        assert_front_eq(front, &expected_fronts[front], &fronts[front]);
    }
}

#[test]
fn test_non_dominated_sort2() {
    test_fronts(1_000, 5);
}

#[test]
fn test_non_dominated_sort3() {
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
