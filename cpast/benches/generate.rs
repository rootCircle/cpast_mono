use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate bench", |b| {
        b.iter(|| clex_gen::generator("N[1,1000] N{\\1}".to_owned()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
