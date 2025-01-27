use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day25::{part1, part1_functional};

fn benchmark(c: &mut Criterion) {
    let keys = vec![[1, 2, 3, 4, 5], [0, 1, 2, 3, 4]];
    let locks = vec![[4, 3, 2, 1, 0], [5, 4, 3, 2, 1]];

    c.bench_function("part1", |b| {
        b.iter(|| part1(black_box(&keys), black_box(&locks)))
    });
    c.bench_function("part1_functional", |b| {
        b.iter(|| part1_functional(black_box(&keys), black_box(&locks)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
