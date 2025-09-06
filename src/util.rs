/// Generate a linear chirp
pub fn chirp(fs: f32, n_samples: usize, start_freq: f32, end_freq: f32) -> Vec<f32> {
    let mut signal = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let t = i as f32 / fs;
        let k = (end_freq - start_freq) / n_samples as f32;
        signal.push((2.0 * std::f32::consts::PI * (start_freq + k * i as f32) * t).sin());
    }

    signal
}
