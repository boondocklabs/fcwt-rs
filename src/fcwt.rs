use crate::fft::FftBackend;
use crate::CwtResult;
use crate::{scales::Scales, wavelet::Wavelet};

type Float = super::Float;
type Complex = super::Complex;
pub struct FastCwt<W: Wavelet, S: Scales> {
    wavelet: W,
    scales: S,
    normalize: bool,
}

impl <W: Wavelet, S: Scales> FastCwt<W, S> {
    pub fn new(wavelet: W, scales: S, normalize: bool) -> Self {

        Self {
            wavelet,
            scales,
            normalize,
        }
    }

    pub fn wavelet(&self) -> &W {
        &self.wavelet
    }

    pub fn scales(&self) -> &S {
        &self.scales
    }

    pub fn cwt(&mut self, input: &mut Vec<Float>) -> CwtResult<Float> {

        #[cfg(feature="profile")]
        puffin::profile_function!();

        #[cfg(feature="profile")]
        puffin::profile_scope!("cwt");

        let mut fft = {
            #[cfg(feature="profile")]
            puffin::profile_scope!("plan");

            #[cfg(feature = "fftw")]
            let fft = crate::fft::FftwBackend::<Float>::new(input.len());

            #[cfg(not(feature = "fftw"))]
            let fft = crate::fft::RustFftBackend::<Float>::new(input.len());

            fft
        };

        /*
        if input.len().is_power_of_two() == false {
            let npot = input.len().next_power_of_two();
            let npot_delta = npot - input.len();
            input.extend_from_slice(&vec![0f32; npot_delta]);
        }
        */

        assert!(input.len().is_power_of_two());

        let mut output = {
            #[cfg(feature="profile")]
            puffin::profile_scope!("alloc");
            CwtResult::new(self.scales.len(), input.len())
        };

        let mut buffer = {
            #[cfg(feature="profile")]
            puffin::profile_scope!("alloc_input");
            vec![Complex::new(0.0, 0.0); input.len()]
        };

        let input_fft = {
            #[cfg(feature="profile")]
            puffin::profile_scope!("fft");
            fft.forward(input)
        };

        {
            #[cfg(feature="profile")]
            puffin::profile_scope!("mother");
            self.wavelet.generate_mother(input.len());
        }

        for i in 0..self.scales.len() {
            #[cfg(feature="profile")]
            puffin::profile_scope!("scale", i.to_string());

            let row = self.convolve(&mut fft, &input_fft.to_vec(), &mut buffer, self.scales.scale(i));

            {
                #[cfg(feature="profile")]
                puffin::profile_scope!("push", i.to_string());
                output.push_row(row);
            }
        }

        if self.normalize {
            #[cfg(feature="profile")]
            puffin::profile_scope!("normalize");
            output.normalize();
        }

        #[cfg(feature="profile")]
        puffin::GlobalProfiler::lock().new_frame();

        output
    }

    fn convolve(&mut self, fft: &mut dyn FftBackend<Float>, input: &Vec<Complex>, buffer: &mut Vec<Complex>, scale: Float) -> Vec<Complex> {
        #[cfg(feature="profile")]
        puffin::profile_function!();

        self.daughter_wavelet_multiply(input, buffer, scale, false, false);

        let output = fft.inverse(buffer);

        /*
        if self.normalize == true {
            self.normalize_inplace(&mut output);
        }
        */

       output 
    }

    /*
    #[inline]
    fn normalize_inplace(&self, data: &mut Vec<Complex>) {
        #[cfg(feature="profile")]
        puffin::profile_function!();

        let size = data.len();

        for i in 0..size {
            data[i] /= size as Float;
        }
    }
    */

    fn daughter_wavelet_multiply(
        &mut self,
        input: &Vec<Complex>,
        output: &mut Vec<Complex>,
        scale: f32,
        imaginary: bool,
        doublesided: bool,
    ) {
        #[cfg(feature="profile")]
        puffin::profile_function!();

        let size = input.len();
        let step = scale / 2.0;
        let endpoint = ((size as f32) / 2.0).min((size as f32) * 2.0 / scale) as usize;

        let mother = self.wavelet.mother();
        assert!(mother.len() > 0);

        for i in 0..endpoint {
            let mother_index = ((size - 1) as f32).min(step * i as f32);

            output[i].re = input[i].re * mother[mother_index as usize];
            output[i].im = input[i].im * mother[mother_index as usize] * (1.0 - 2.0 * (imaginary as i32 as f32));
        }

        if doublesided {
            for i in 0..endpoint {
                let mother_index = (size - 1).min((step * i as f32) as usize);
                output[size - 1 - i].re = input[size - 1 - i].re * mother[mother_index] * (1.0 - 2.0 * (imaginary as i32 as f32));
                output[size - 1 - i].im = input[size - 1 - i].im * mother[mother_index];
            }
        }
    }

}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wavelet::MorletWavelet;
    use crate::scales::LinFreqs;

    #[test]
    fn test_fast_cwt_new() {
        let w = MorletWavelet::new(1.0);
        let s = LinFreqs::new(&w, 100, 10.0, 20.0, 5);
        let mut fast_cwt = FastCwt::new(w, s, false);
        // Check if the FastCwt instance is created successfully
        assert_eq!(fast_cwt.cwt(&mut vec![0.0; 8]).len(), 5);
    }

    #[test]
    fn test_fast_cwt_cwt_power_of_two() {
        let w = MorletWavelet::new(1.0);
        let s = LinFreqs::new(&w, 100, 10.0, 20.0, 5);
        let slen = s.len();
        let mut fast_cwt = FastCwt::new(w, s, false);
        let mut input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let output = fast_cwt.cwt(&mut input);

        // Check if the output has the correct dimensions
        assert_eq!(output.len(), slen);
        assert_eq!(output[0].len(), input.len());
    }

    #[test]
    #[should_panic]
    fn test_fast_cwt_cwt_non_power_of_two() {
        let w = MorletWavelet::new(1.0);
        let s = LinFreqs::new(&w, 100, 10.0, 20.0, 5);
        let mut fast_cwt = FastCwt::new(w, s, false);
        let mut input = vec![1.0, 2.0, 3.0, 4.0, 5.0]; // Not a power of two
        let _ = fast_cwt.cwt(&mut input);
    }

    #[test]
    fn test_daughter_wavelet_multiply() {
        let w = MorletWavelet::new(1.0);
        let s = LinFreqs::new(&w, 100, 10.0, 20.0, 5);
        let mut fast_cwt = FastCwt::new(w, s, false);
        fast_cwt.wavelet.generate_mother(1024);

        // Define the input data and parameters
        let input = vec![
            Complex::new(1.0, 2.0),
            Complex::new(3.0, 4.0),
            Complex::new(5.0, 6.0),
            Complex::new(7.0, 8.0),
        ];
        let scale = 2.0;
        let imaginary = false;
        let doublesided = false;

        let _expected_output = vec![
            Complex::new(0.0, 0.0), // Replace with the expected values
            Complex::new(0.0, 0.0), // Replace with the expected values
            Complex::new(0.0, 0.0), // Replace with the expected values
            Complex::new(0.0, 0.0), // Replace with the expected values
        ];

        let mut buffer = vec![Complex::new(0.0, 0.0); 1024];

        // Call the function
        fast_cwt.daughter_wavelet_multiply(&input, &mut buffer, scale, imaginary, doublesided);

        // Assert that the output is as expected
        //assert_eq!(output, expected_output);
    }
}
*/