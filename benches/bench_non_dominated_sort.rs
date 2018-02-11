#![feature(test)]
extern crate non_dominated_sort;
extern crate test;

#[path = "../tests/common/mod.rs"]
mod common;

use common::{assert_front_eq, create_solutions_with_n_fronts, TupleDominationOrd};
use non_dominated_sort::NonDominatedSort;
use test::Bencher;

fn test_fronts(n: usize, n_fronts: usize) {
    let (solutions, expected_fronts) = create_solutions_with_n_fronts(n, n_fronts);

    let fronts = NonDominatedSort::new(&solutions, &TupleDominationOrd).pareto_fronts();

    assert_eq!(n_fronts, fronts.len());

    /*
    for front in 0..n_fronts {
        assert_front_eq(front, &expected_fronts[front], &fronts[front]);
    }
    */
}

#[bench]
fn bench_non_dominated_sort_1000_10(b: &mut Bencher) {
    b.iter(|| test_fronts(1_000, 10));
}
