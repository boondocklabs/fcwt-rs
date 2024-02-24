# fCWT Rust Library

`fcwt` is a pure Rust library for fast continuous wavelet transform (CWT) computations using the FFT algorithm. It provides a flexible and efficient way to perform CWT on signals, particularly useful in signal processing and time-frequency analysis.

This is a port of the fCWT C++ library [[https://github.com/fastlib/fCWT]]. Test vectors have been compared between the outputs of fCWT and fcwt-rs.

## Features

- Supports custom wavelet and scale definitions using traits.
- Utilizes rustfft for fast computations in pure Rust
- fftw3 can optionally be enabled with the `fftw` feature flag

## TODO

- Implement parallel computations
- Currently only LinFreqs scales are implemented

## Installation

Add `fcwt` to your `Cargo.toml`:

```toml
[dependencies]
fcwt = "0.1.2"
```

## Usage
```rust
use fcwt::{FastCwt, wavelet::MorletWavelet, scales::LinFreqs};

// Create a Morlet Wavelet with sigma=1.0
let wavelet = MorletWavelet::new(2.0);

// Create a frequency scale 
let scales = LinFreqs::new(&wavelet, 1000, 0.1, 40.0, 300);

// Create a FastCwt instance using the specified Wavelet and Scales
let mut fcwt = FastCwt::new(w, scales, true);

// Create a chirp test signal
let mut signal = fcwt::util::chirp(1000.0, 65536, 0.1, 20.0);

// Compute the fCWT and return a CwtResult<Float>
let result = cwt.cwt(&mut signal);
```
