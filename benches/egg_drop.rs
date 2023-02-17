use egg_drop::{egg_drop_old, par_simple_dp, simple_dp};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Egg Drop");
    group.sample_size(10); // 10 is minimum required; default is 100
    for parameter in [1000].iter() {
        group.throughput(Throughput::Elements(*parameter as u64));
        group.bench_with_input(
            BenchmarkId::new("Serial Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop_old(simple_dp, *par, 1000)),
        );
        group.bench_with_input(
            BenchmarkId::new("Parallel Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop_old(par_simple_dp, *par, 1000)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
