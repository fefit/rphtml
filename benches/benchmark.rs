use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rxhtml::parser::*;
fn parse_doc() {
	let mut doc = Doc::new();
	doc.parse_file("./cases/full.html");
}
fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("parse", |b| b.iter(|| parse_doc()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
