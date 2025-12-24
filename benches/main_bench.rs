use aoc_24_rust::real_main;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_my_code(c: &mut Criterion) {
    c.bench_function("aoc_main", |b| b.iter(|| real_main()));
}

criterion_group!(benches, bench_my_code);
criterion_main!(benches);
