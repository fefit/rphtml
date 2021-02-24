use criterion::{criterion_group, criterion_main, Criterion};
use rphtml::config::ParseOptions;
use rphtml::parser::*;
fn parse_doc() {
	let _ = Doc::parse_file("./cases/full.html", Default::default());
}
fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("parse-default", |b| b.iter(parse_doc));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
