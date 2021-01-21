use criterion::{criterion_group, criterion_main, Criterion};
use rphtml::config::ParseOptions;
use rphtml::parser::*;
fn parse_doc() {
	let _ = Doc::parse_file("./cases/full.html", Default::default());
}
fn parse_doc_decode_entity() {
	let _ = Doc::parse_file(
		"./cases/full.html",
		ParseOptions {
			..Default::default()
		},
	);
}
fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("parse-default", |b| b.iter(parse_doc));
	c.bench_function("parse-with-decode", |b| b.iter(parse_doc_decode_entity));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
