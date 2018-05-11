#![feature(test)]
extern crate non_dominated_sort;
extern crate test;

#[path = "../tests/common/mod.rs"]
mod common;

use common::{create_solutions_with_n_fronts, TupleDominationOrd};
use non_dominated_sort::non_dominated_sort;
use test::Bencher;

#[bench]
fn bench_non_dominated_sort_1000_10(b: &mut Bencher) {
    let (solutions, expected_fronts) = create_solutions_with_n_fronts(1000, 10);
    b.iter(|| {
        let mut f = non_dominated_sort(&solutions, &TupleDominationOrd);
        for (expected_rank, expected_front) in expected_fronts.iter().enumerate() {
            assert_eq!(expected_rank, f.rank());
            assert_eq!(&expected_front[..], f.current_front_indices());
            f = f.next_front();
        }
        assert_eq!(true, f.is_empty());
    });
}
