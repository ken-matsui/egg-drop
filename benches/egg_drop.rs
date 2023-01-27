use egg_drop::{egg_drop, simple_dp};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Egg Dropping");
    for parameter in [1000, 2000, 3000, 4000, 5000].iter() {
        group.throughput(Throughput::Elements(*parameter as u64));
        group.bench_with_input(
            BenchmarkId::new("Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(simple_dp, *par, 5000)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
