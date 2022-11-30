use egg_drop::{egg_drop, fast_dp, par_fast_dp, rayon_par_fast_dp, simple_dp};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

struct Input {
    pub(crate) n: i32,
    pub(crate) h: i32,
}

impl Input {
    pub(crate) fn new(n: i32, h: i32) -> Input {
        Input { n, h }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(n: {}, h: {})", self.n, self.h)
    }
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Egg Dropping");
    for parameter in [
        Input::new(2, 1),
        Input::new(1, 2),
        Input::new(2, 6),
        Input::new(3, 14),
        Input::new(4, 30),
        Input::new(5, 62),
        Input::new(6, 126),
        Input::new(50, 500),
        Input::new(500, 1000),
    ]
    .iter()
    {
        group.throughput(Throughput::Elements(parameter.n as u64));
        group.bench_with_input(
            BenchmarkId::new("Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(simple_dp, par.n, par.h)),
        );
        group.bench_with_input(
            BenchmarkId::new("Fast DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(fast_dp, par.n, par.h)),
        );
        group.bench_with_input(
            BenchmarkId::new("Rayon Parallel Fast DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(rayon_par_fast_dp, par.n, par.h)),
        );
        group.bench_with_input(
            BenchmarkId::new("Parallel Fast DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(par_fast_dp, par.n, par.h)),
        );
    }
    group.finish();
}

fn bench2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Egg Dropping");
    for parameter in [1000, 2000, 3000, 4000, 5000].iter() {
        group.throughput(Throughput::Elements(*parameter as u64));
        group.bench_with_input(
            BenchmarkId::new("Simple DP", parameter),
            parameter,
            |b, par| b.iter(|| egg_drop(simple_dp, *par, 5000)),
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

criterion_group!(benches, bench2);
criterion_main!(benches);
