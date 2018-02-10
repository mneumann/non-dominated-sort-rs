extern crate non_dominated_sort;
mod common;

use common::{Tuple, TupleDominationOrd};
use non_dominated_sort::{Front, NonDominatedSort};

fn get_solutions() -> Vec<Tuple> {
    vec![
        Tuple(1, 2),
        Tuple(1, 2),
        Tuple(2, 1),
        Tuple(1, 3),
        Tuple(0, 2),
    ]
}

fn assert_front_eq(expected_rank: usize, expected_indices: &[usize], front: &Front<Tuple>) {
    assert_eq!(expected_rank, front.rank);
    assert_eq!(expected_indices.to_owned(), front.solutions_indices_only());
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
