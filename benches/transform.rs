use criterion::{criterion_group, criterion_main, Criterion};
use fcwt::{CwtResult, FastCwt, LinFreqs, MorletWavelet};
/// Run a batch of transforms. The built binary can be used in Intel VTune to analyze performance
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SCALES: usize = 10;
const SAMPLES: usize = 2048;
const SAMPLE_RATE: usize = 10000;

fn transform_normalized() -> CwtResult<f32> {
    let w = MorletWavelet::new(2.0);
    let scales = LinFreqs::new(SAMPLE_RATE, 0.1, 40.0, SCALES);
    let mut fcwt = FastCwt::new(w, scales, true);

    let input = fcwt::util::chirp(SAMPLE_RATE as f32, SAMPLES, 0.1, 20.0);

    fcwt.cwt(&input)
}

fn transform() -> CwtResult<f32> {
    let w = MorletWavelet::new(2.0);
    let scales = LinFreqs::new(SAMPLE_RATE, 0.1, 40.0, SCALES);
    let mut fcwt = FastCwt::new(w, scales, false);

    let input = fcwt::util::chirp(SAMPLE_RATE as f32, SAMPLES, 0.1, 20.0);

    fcwt.cwt(&input)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fcwt 2048x10", |b| b.iter(|| transform()));
    c.bench_function("fcwt 2048x10 norm", |b| b.iter(|| transform_normalized()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
