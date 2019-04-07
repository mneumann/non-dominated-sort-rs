#[path = "../tests/common/mod.rs"]
mod common;

use common::{create_solutions_with_n_fronts, TupleDominanceOrd};
use criterion::{criterion_group, criterion_main, Criterion};
use non_dominated_sort::non_dominated_sort;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "non_dominated_sort",
        |b: &mut criterion::Bencher, &n: &usize| {
            let (solutions, expected_fronts) = create_solutions_with_n_fronts(n, 10);
            b.iter(|| {
                let mut f = non_dominated_sort(&solutions, &TupleDominanceOrd);
                for (expected_rank, expected_front) in expected_fronts.iter().enumerate() {
                    assert_eq!(expected_rank, f.rank());
                    assert_eq!(&expected_front[..], f.current_front_indices());
                    f = f.next_front();
                }
                assert_eq!(true, f.is_empty());
            })
        },
        vec![100, 200],
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
