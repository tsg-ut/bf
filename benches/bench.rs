use bf::compile;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

pub fn bench_cat(c: &mut Criterion) {
    let code = fs::read_to_string("examples/cat.bf").unwrap();
    c.bench_function("cat", |b| {
        b.iter(|| {
            let program = compile(black_box(&code)).unwrap();
            let input = b"meow";
            let _output = program.run(input).unwrap();
        });
    });
}

pub fn bench_elvm_hello(c: &mut Criterion) {
    let code = fs::read_to_string("examples/elvm-hello.bf").unwrap();
    c.bench_function("elvm_hello", |b| {
        b.iter(|| {
            let program = compile(black_box(&code)).unwrap();
            let input = b"";
            let _output = program.run(input).unwrap();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_cat, bench_elvm_hello
}
criterion_main!(benches);
