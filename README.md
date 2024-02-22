# fcwt-rs

`fcwt-rs` is a pure Rust library for fast continuous wavelet transform (CWT) computations using the FFT algorithm. It provides a flexible and efficient way to perform CWT on signals, particularly useful in signal processing and time-frequency analysis.

This is a port of the fCWT C++ library [[https://github.com/fastlib/fCWT]]. Test vectors have been compared between the outputs of fCWT and fcwt-rs.

## Features

- Supports custom wavelet and scale definitions using traits.
- Utilizes rustfft for fast computation.

## TODO

- Implement parallel computations
- Currently only LinFreqs scales are implemented

## Installation

Add `fcwt-rs` to your `Cargo.toml`:

```toml
[dependencies]
fcwt-rs = "0.1.0"
```

## Usage
```rust
use fcwt_rs::{FastCwt, wavelet::MorletWavelet, scales::LinFreqs};

// Create a Morlet Wavelet with sigma=1.0
let wavelet = MorletWavelet::new(1.0);

// Create a frequency scale from 10-20Hz at 1kHz sample rate, with a size of 200
let scales = LinFreqs::new(&wavelet, 1000, 10.0, 20.0, 200);

// Creat a FastCwt instance using the specified Wavelet and Scales
let mut cwt = FastCwt::new(wavelet, scales, false);

// Input signal length must be a power of two
let mut signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

// Compute the fCWT
let result = cwt.cwt(&mut signal);

println!("{:?}", result);
```
