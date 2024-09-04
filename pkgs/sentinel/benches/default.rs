use criterion::{criterion_group, criterion_main, Criterion};
use synthetic::io::server::Server;

fn criterion_benchmark(criteria: &mut Criterion) {
  criteria.bench_function("Sentinel Serve", |bench| bench.iter(Server::run));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
