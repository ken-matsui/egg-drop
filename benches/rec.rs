use egg_drop::{egg_drop, rec};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_rec(c: &mut Criterion) {
    c.bench_function("rec", |b| {
        b.iter(|| egg_drop(rec, black_box(3), black_box(14)))
    });
}

criterion_group!(benches, bench_rec);
criterion_main!(benches);
