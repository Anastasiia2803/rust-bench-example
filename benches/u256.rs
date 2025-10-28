use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn bench_addition(c: &mut Criterion) {
    c.bench_function("u128_add", |b| {
        b.iter(|| {
            let mut x = 0u128;
            for i in 0..1000 {
                x = x.wrapping_add(i);
            }
            black_box(x)
        })
    });
}

criterion_group!(benches, bench_addition);
criterion_main!(benches);
