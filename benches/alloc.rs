use criterion::{criterion_group, criterion_main, Criterion};

//use rustfft::num_complex::Complex;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Clone)]
pub struct Complex<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
}


impl<T: Clone> Complex<T> {
    /// Create a new Complex
    #[inline]
    pub const fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("nested vec Complex<f32>", |b| b.iter(|| vec![vec![Complex::new(0.0,0.0); 8192]; 300] ));
    c.bench_function("flat vec Complex<f32>", |b| b.iter(|| vec![Complex::new(0.0,0.0); 300*8192] ));
    c.bench_function("nested vec f64", |b| b.iter(|| vec![vec![0.0f64; 8192]; 300] ));
    c.bench_function("flat vec f64", |b| b.iter(|| vec![0.0f64; 300*8192] ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);