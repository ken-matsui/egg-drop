use egg_drop::{egg_drop, fast_dp, par_fast_dp, rayon_par_fast_dp, simple_loop_dp};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Egg Dropping");
    for parameter in [1000, 2000, 3000, 4000, 5000].iter() {
        group.throughput(Throughput::Elements(*parameter as u64));
        group.bench_with_input(
            BenchmarkId::new("Simple Loop DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(simple_loop_dp, *par, 5000)),
        );
        group.bench_with_input(
            BenchmarkId::new("Fast DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(fast_dp, *par, 5000)),
        );
        group.bench_with_input(
            BenchmarkId::new("Rayon Parallel Fast DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(rayon_par_fast_dp, *par, 5000)),
        );
        group.bench_with_input(
            BenchmarkId::new("Parallel Fast DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(par_fast_dp, *par, 5000)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
