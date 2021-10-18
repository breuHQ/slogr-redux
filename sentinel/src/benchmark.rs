use criterion::{criterion_group, criterion_main, Criterion};
use synthetic::io::server::Server;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("Sentinel Serve", |b| b.iter(|| Server::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
