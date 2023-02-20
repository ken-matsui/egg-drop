use egg_drop::{egg_drop, par_simple_dp, simple_dp};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Egg Drop");
    group.sample_size(10); // 10 is minimum required; default is 100
    for parameter in [20].iter() {
        group.throughput(Throughput::Elements(*parameter as u64));
        group.bench_with_input(
            BenchmarkId::new("Serial Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(simple_dp, 1000, 1000, *par)),
        );
        group.bench_with_input(
            BenchmarkId::new("Parallel Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(par_simple_dp, 1000, 1000, *par)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
