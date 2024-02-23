use std::sync::Arc;
use rustfft::{Fft,FftNum};
use crate::{Complex, Float};

pub trait FftBackend<T> {
    fn forward(&mut self, input: &[T]) -> Vec<Complex>;
    fn inverse(&mut self, input: &mut [Complex]) -> Vec<Complex>;
}

/// RustFFT Backend
pub struct RustFftBackend<T: FftNum> {
    forward_scratch_buffer: Vec<Complex>,
    inverse_scratch_buffer: Vec<Complex>,
    forward_plan: Arc<dyn Fft<T>>,
    inverse_plan: Arc<dyn Fft<T>>,
}

impl <T: FftNum> RustFftBackend<T> {
    pub fn new(size: usize) -> Self {
        let mut planner = rustfft::FftPlanner::<T>::new();

        let forward_plan = planner.plan_fft_forward(size);
        let inverse_plan = planner.plan_fft_inverse(size);

        Self {
            forward_scratch_buffer: vec![Complex::new(0.0,0.0); forward_plan.get_inplace_scratch_len()],
            inverse_scratch_buffer: vec![Complex::new(0.0,0.0); inverse_plan.get_inplace_scratch_len()],
            forward_plan,
            inverse_plan,
        }
    }
}

impl FftBackend<Float> for RustFftBackend<Float> {
    fn forward(&mut self, input: &[Float]) -> Vec<Complex> {
        let mut out: Vec<Complex> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
        self.forward_plan.process_with_scratch(out.as_mut_slice(), self.forward_scratch_buffer.as_mut_slice());
        out
    }

    fn inverse(&mut self, input: &mut [Complex]) -> Vec<Complex> {
        let mut output = input.to_vec();
        self.inverse_plan.process_with_scratch(output.as_mut_slice(), self.inverse_scratch_buffer.as_mut_slice());
        output
    }
}

/// FFTW3 Backend using the fftw crate
#[cfg(feature = "fftw")]
pub use fftw_backend::FftwBackend;

#[cfg(feature = "fftw")]
mod fftw_backend {
    use super::*;

    use fftw::{
        plan::*,
        types::*,
        array::{AlignedAllocable, AlignedVec},
    };

    pub struct FftwBackend<T: FftNum> {

        forward_input_buffer: AlignedVec<T>,
        forward_output_buffer: AlignedVec<Complex>,

        inverse_input_buffer: AlignedVec<Complex>,
        inverse_output_buffer: AlignedVec<Complex>,

        forward_plan: R2CPlan32,
        inverse_plan: C2CPlan32,
    }

    impl <T: FftNum+AlignedAllocable+Default> FftwBackend<T> {
        pub fn new(size: usize) -> Self {

            // Allocate buffers for real to complex forward transform. Output length is size/2+1
            let forward_input_buffer = AlignedVec::new(size);
            let forward_output_buffer = AlignedVec::new((size>>1) + 1);

            let inverse_input_buffer = AlignedVec::new(size);
            let inverse_output_buffer = AlignedVec::new(size);

            // Create plans
            let forward_plan = R2CPlan::aligned(&[size], Flag::ESTIMATE).unwrap();
            let inverse_plan = C2CPlan::aligned(&[size], Sign::Backward, Flag::ESTIMATE).unwrap();

            Self {
                forward_input_buffer,
                forward_output_buffer,
                inverse_input_buffer,
                inverse_output_buffer,
                forward_plan,
                inverse_plan,
            }
        }
    }

    impl FftBackend<f32> for FftwBackend<f32> {
        fn forward(&mut self, input: &[f32]) -> Vec<Complex> {
            self.forward_input_buffer.copy_from_slice(input);
            self.forward_plan.r2c(&mut self.forward_input_buffer, &mut self.forward_output_buffer).unwrap();

            let mut out = vec![Complex::new(0.0,0.0); input.len()];

            out[0..(input.len()>>1) + 1].copy_from_slice(self.forward_output_buffer.as_slice());

            // Make the FFT output symmetrical
            for i in 1..(input.len() >> 1) {
                out[input.len() - i].re = out[i].re;
                out[input.len() - i].im = -out[i].im;
            }

            out
        }

        fn inverse(&mut self, input: &mut [Complex]) -> Vec<Complex> {
            self.inverse_input_buffer.copy_from_slice(input);
            self.inverse_plan.c2c(&mut self.inverse_input_buffer, &mut self.inverse_output_buffer).unwrap();
            self.inverse_output_buffer.to_vec()
        }
    }
}