#![feature(test)]
extern crate non_dominated_sort;
extern crate test;

#[path = "../tests/common/mod.rs"]
mod common;

use common::test_fronts;
use test::Bencher;

#[bench]
fn bench_non_dominated_sort_1000_10(b: &mut Bencher) {
    b.iter(|| test_fronts(1_000, 10));
}
