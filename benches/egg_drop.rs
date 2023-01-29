use egg_drop::{egg_drop, par_simple_dp, simple_dp};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Egg Drop");
    for parameter in [100, 200, 300, 400, 500].iter() {
        group.throughput(Throughput::Elements(*parameter as u64));
        group.bench_with_input(
            BenchmarkId::new("Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(simple_dp, *par, 500)),
        );
        group.bench_with_input(
            BenchmarkId::new("Parallel Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(par_simple_dp, *par, 500)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
