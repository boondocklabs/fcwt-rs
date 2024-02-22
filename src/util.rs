
/// Generate a linear chirp
pub fn chirp(fs: f32, n: usize, f0: f32, f1: f32) -> Vec<f32> {
    let mut signal = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f32 / fs;
        let k = (f1 - f0) / n as f32;
        signal.push((2.0 * std::f32::consts::PI * (f0 + k * t * fs) * t).sin());
    }

    signal
}