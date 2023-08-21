use criterion::black_box;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn is_none(option: Option<i32>) {
    black_box(option.is_none());
}

fn eq_none(option: Option<i32>) {
    black_box(option == None);
}

fn matches_none(option: Option<i32>) {
    black_box(matches!(option, None));
}

pub fn benchmark(criterion: &mut Criterion) {
    criterion.bench_function("is none/some", |b| b.iter(|| is_none(black_box(Some(5)))));
    criterion.bench_function("is none/none", |b| b.iter(|| is_none(black_box(None))));

    criterion.bench_function("eq none/some", |b| b.iter(|| eq_none(black_box(Some(5)))));
    criterion.bench_function("eq none/none", |b| b.iter(|| eq_none(black_box(None))));

    criterion.bench_function("matches none/some", |b| {
        b.iter(|| matches_none(black_box(Some(5))))
    });
    criterion.bench_function("matches none/none", |b| {
        b.iter(|| matches_none(black_box(None)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
