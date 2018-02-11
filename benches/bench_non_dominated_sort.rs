#![feature(test)]
extern crate non_dominated_sort;
extern crate test;

#[path = "../tests/common/mod.rs"]
mod common;

use common::{create_solutions_with_n_fronts, TupleDominationOrd};
use test::Bencher;
use non_dominated_sort::non_dominated_sort;

#[bench]
fn bench_non_dominated_sort_1000_10_usize(b: &mut Bencher) {
    let (solutions, _expected_fronts) = create_solutions_with_n_fronts(1_000, 10);

    b.iter(|| {
        let mut f = non_dominated_sort::<_, _, usize>(&solutions, &TupleDominationOrd);
    });
}

#[bench]
fn bench_non_dominated_sort_1000_10_u32(b: &mut Bencher) {
    let (solutions, _expected_fronts) = create_solutions_with_n_fronts(1_000, 10);

    b.iter(|| {
        let mut f = non_dominated_sort::<_, _, u32>(&solutions, &TupleDominationOrd);
    });
}
