type Float = super::Float;

pub trait Scales {
    fn len(&self) -> usize;
    fn sample_rate(&self) -> usize;
    fn scale(&self, index: usize) -> Float;
    fn freq(&self, index: usize) -> Float;
}

#[derive(Debug, Clone)]
pub struct LinFreqs {
    scales: Vec<Float>,
    sample_rate: usize,
}

impl LinFreqs {
    pub fn new(sample_rate: usize, start_freq: Float, end_freq: Float, size: usize) -> Self {
        assert!(
            start_freq < end_freq,
            "start frequency must be lower than the end frequency"
        );
        // Ensure end freq is below Nyquist frequency (sample rate/2)
        assert!(end_freq <= (sample_rate / 2) as Float);

        let mut scales: Vec<Float> = vec![0.0; size];

        // frequency delta
        let df = end_freq - start_freq;

        for i in 0..size {
            scales[size - i - 1] =
                (sample_rate as Float) / (start_freq + (df / size as Float) * i as Float);
        }

        Self {
            scales,
            sample_rate,
        }
    }
}

impl Scales for LinFreqs {
    #[inline(always)]
    fn len(&self) -> usize {
        self.scales.len()
    }

    #[inline(always)]
    fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    #[inline(always)]
    fn scale(&self, index: usize) -> Float {
        self.scales[index]
    }

    #[inline(always)]
    fn freq(&self, index: usize) -> Float {
        assert!(
            index < self.scales.len(),
            "Frequency Index must be in bounds"
        );
        self.sample_rate as Float / self.scales[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: Float = 1e-5;

    #[test]
    fn test_lin_freqs_new() {
        let sample_rate = 100;
        let start_freq = 10.0;
        let end_freq = 20.0;
        let size = 5;
        let lin_freqs = LinFreqs::new(sample_rate, start_freq, end_freq, size);

        assert_eq!(lin_freqs.scales.len(), size);
        assert_eq!(lin_freqs.sample_rate, sample_rate);

        // Check if scales are calculated correctly
        let df = end_freq - start_freq;
        for i in 0..size {
            let expected_scale =
                (sample_rate as Float) / (start_freq + (df / size as Float) * i as Float);
            assert_eq!(lin_freqs.scales[size - i - 1], expected_scale);
        }
    }

    #[test]
    #[should_panic]
    fn test_lin_freqs_new_end_freq_above_nyquist() {
        let sample_rate = 100;
        let start_freq = 10.0;
        let end_freq = 60.0; // Above Nyquist frequency (sample_rate / 2)
        let size = 5;
        let _ = LinFreqs::new(sample_rate, start_freq, end_freq, size);
    }

    #[test]
    fn test_values() {
        let scales = LinFreqs::new(1000, 1.0, 20.0, 1000);
        assert_eq!(scales.len(), 1000);

        // Compare with values from python bindings to fCWT
        assert!(scales.freq(0) - 19.980999 < EPS);
        assert!(scales.freq(1) - 19.962 < EPS);
        assert_eq!(scales.freq(scales.len() - 1), 1.0);
    }
}
