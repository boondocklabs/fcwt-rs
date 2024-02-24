/// Run a batch of transforms. The built binary can be used in Intel VTune to analyze performance

use mimalloc::MiMalloc;
use fcwt::{MorletWavelet,LinFreqs,FastCwt};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let w = MorletWavelet::new(2.0);
    let scales = LinFreqs::new(&w, 1000, 0.1, 40.0, 300);
    let mut fcwt = FastCwt::new(w, scales, true);

    let mut input = fcwt::util::chirp(1000.0, 65536, 0.1, 20.0);

    for i in 0..100 {
        fcwt.cwt(&mut input);
        println!("Iter {}", i);
    }
}