use criterion::black_box;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn is_none(option: Option<i32>) {
    black_box(option.is_none());
}

fn eq_none(option: Option<i32>) {
    black_box(option == None);
}

pub fn benchmark(criterion: &mut Criterion) {
    criterion.bench_function("is none", |b| b.iter(|| is_none(black_box(Some(5)))));
    criterion.bench_function("eq none", |b| b.iter(|| eq_none(black_box(Some(5)))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
