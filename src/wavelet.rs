use std::f32::consts::PI;

pub type Float = super::Float;
pub type Complex = super::Complex;

pub trait Wavelet {
    fn generate_mother(&mut self, size: usize) -> Vec<Float>;
    fn generate(&self, size: usize, scale: Float) -> Vec<Complex>;
    //fn get(&self, size: usize, scale: Float) -> Vec<Complex>;
    fn bandwidth(&self) -> Float;
    fn four_wavelen(&self) -> Float;
    fn imag_frequency(&self) -> bool;
    fn is_double_sided(&self) -> bool;
    fn mother(&self) -> &[Float];

    fn get_support(&self, scale: Float) -> isize {
        (self.bandwidth() * scale * 3.0) as isize
    }
}

pub struct MorletWavelet {
    four_wavelen: Float,
    imag_frequency: bool,
    double_sided: bool,
    mother: Vec<Float>,

    fb: Float,
    ifb: Float,
    fb2: Float,
}

impl MorletWavelet {
    pub fn new(bandwidth: Float) -> Self {
        Self {
            four_wavelen: 0.9876,
            fb: bandwidth,
            fb2: 2.0 * bandwidth * bandwidth,
            ifb: 1.0 / bandwidth,
            imag_frequency: false,
            double_sided: false,
            mother: vec![],
        }
    }
}

impl Wavelet for MorletWavelet {
    fn generate_mother(&mut self, size: usize) -> Vec<Float> {
        let mut mother = Vec::with_capacity(size);

        let torad = (2.0 * PI) / size as Float;
        let norm = (2.0 * PI).sqrt() * super::IPI4;

        for i in 0..size {
            //let mut tmp = 2.0 * (i as Float).to_radians() * self.fb - 2.0 * PI * self.fb;
            let mut tmp = 2.0 * (i as Float * torad) * self.fb - 2.0 * PI * self.fb;
            tmp = -(tmp * tmp) / 2.0;

            mother.push(norm * tmp.exp());
        }

        self.mother = mother.clone();

        mother
    }

    fn generate(&self, size: usize, scale: Float) -> Vec<Complex> {
        let width = self.get_support(scale);
        let norm = size as Float * self.ifb * super::IPI4;

        let mut output: Vec<Complex> = Vec::with_capacity((width * 2 + 1) as usize);

        for i in 0..width * 2 + 1 {
            let tmp1 = (i - width) as Float / scale;
            let tmp2 = (-(tmp1 * tmp1) / self.fb2).exp();

            let real = norm * tmp2 * (tmp1 * 2.0 * PI).cos() / scale;
            let imag = norm * tmp2 * (tmp1 * 2.0 * PI).sin() / scale;

            output.push(Complex::new(real, imag));
        }

        output
    }

    #[inline(always)]
    fn bandwidth(&self) -> Float {
        self.fb
    }

    #[inline(always)]
    fn four_wavelen(&self) -> Float {
        self.four_wavelen
    }

    #[inline(always)]
    fn imag_frequency(&self) -> bool {
        self.imag_frequency
    }

    #[inline(always)]
    fn is_double_sided(&self) -> bool {
        self.double_sided
    }

    #[inline(always)]
    fn mother(&self) -> &[Float] {
        self.mother.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Epsilon for float comparison
    const EPS: Float = 1e-5;

    #[test]
    fn test_morlet_wavelet_new() {
        let morlet = MorletWavelet::new(1.0);
        assert_eq!(morlet.fb, 1.0);
        assert_eq!(morlet.fb2, 2.0);
        assert_eq!(morlet.ifb, 1.0);
        assert_eq!(morlet.four_wavelen, 0.9876);
        assert!(!morlet.imag_frequency);
        assert!(!morlet.double_sided);
    }

    #[test]
    fn test_morlet_wavelet_generate_mother() {
        let mut morlet = MorletWavelet::new(1.0);
        morlet.mother = morlet.generate_mother(4);
        assert_eq!(morlet.mother.len(), 4);
    }

    #[test]
    fn test_morlet_wavelet_generate_time() {
        let morlet = MorletWavelet::new(2.0);
        let size = 25;
        let scale = 2.0;
        let result = morlet.generate(size, scale);

        assert_eq!(
            result.len(),
            (morlet.get_support(scale) as Float * 2.0 + 1.0) as usize
        );

        // Check if values we computed are within epsilon of values
        // produced using python bindings to fCWT C++ library
        assert!(result[0].re - 0.05215157 < EPS);
        assert!(result[0].im - -4.9752100e-09 < EPS);
        assert!(result[1].re - -0.10700808 < EPS);
        assert!(result[1].im - 2.8149339e-07 < EPS);

        for (i, complex) in result.iter().enumerate() {
            let tmp1 = (i as isize - morlet.get_support(scale)) as f32 / scale;
            let tmp2 = (-(tmp1 * tmp1) / morlet.fb2).exp();
            let expected_real =
                size as Float * morlet.ifb * crate::IPI4 * tmp2 * (tmp1 * 2.0 * PI).cos() / scale;
            let expected_imag =
                size as Float * morlet.ifb * crate::IPI4 * tmp2 * (tmp1 * 2.0 * PI).sin() / scale;

            // Check if within epsilon
            assert!((complex.re - expected_real).abs() < EPS);
            assert!((complex.im - expected_imag).abs() < EPS);
        }
    }

    #[test]
    fn test_morlet_wavelet_bandwidth() {
        let morlet = MorletWavelet::new(2.5);
        assert_eq!(morlet.bandwidth(), 2.5);
    }

    #[test]
    fn test_morlet_wavelet_get_support() {
        let morlet = MorletWavelet::new(1.0);
        assert_eq!(morlet.get_support(1.0), 3);
    }
}
