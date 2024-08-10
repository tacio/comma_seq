use criterion::{black_box, criterion_group, criterion_main, Criterion};
use comma_seq::FirstDigitCalculator;

fn benchmark_first_digit(c: &mut Criterion) {
    let mut calculator = FirstDigitCalculator::new();
    
    c.bench_function("first_digit sequential", |b| {
        b.iter(|| {
            for i in 1..=1000 {
                black_box(calculator.first_digit(black_box(i)));
            }
        })
    });

    c.bench_function("first_digit large numbers", |b| {
        b.iter(|| {
            for i in 1_000_000_000..1_000_001_000 {
                black_box(calculator.first_digit(black_box(i)));
            }
        })
    });
}

criterion_group!(benches, benchmark_first_digit);
criterion_main!(benches);