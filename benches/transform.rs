/// Run a batch of transforms. The built binary can be used in Intel VTune to analyze performance

use mimalloc::MiMalloc;
use fcwt::{MorletWavelet,LinFreqs,FastCwt};
use criterion::{criterion_group, criterion_main, Criterion};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const SCALES: usize = 10;
const SAMPLES: usize = 2048;
const SAMPLE_RATE: usize = 10000;

fn transform() {
    let w = MorletWavelet::new(2.0);
    let scales = LinFreqs::new(&w, SAMPLE_RATE, 0.1, 40.0, SCALES);
    let mut fcwt = FastCwt::new(w, scales, true);

    let mut input = fcwt::util::chirp(SAMPLE_RATE as f32, SAMPLES, 0.1, 20.0);

    let out = fcwt.cwt(&mut input);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fcwt 2048x10", |b| b.iter(|| transform() ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);