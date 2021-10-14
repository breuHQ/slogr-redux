use criterion::{ criterion_group, criterion_main, Criterion};
mod commands;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Sentinel Serve", |b| b.iter(|| commands::serve()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);