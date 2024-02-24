pub type Float = f32;
pub type Complex = rustfft::num_complex::Complex<Float>;

const IPI4: Float = 0.75112554446;

pub mod fft;
pub mod wavelet;
pub mod scales;
pub mod fcwt;
pub mod result;

pub mod util;

pub use wavelet::MorletWavelet;
pub use scales::{Scales, LinFreqs};
pub use fcwt::FastCwt;
pub use result::CwtResult;