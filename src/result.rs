use rustfft::num_complex::Complex;
use core::ops::{Index, IndexMut};
use rayon::prelude::*;


pub struct CwtResult<T> {
    scales: usize,
    samples: usize,
    data: Vec<Vec<Complex<T>>>,
}

impl CwtResult<f32> {
    #[inline]
    pub fn new(scales: usize, samples: usize) -> Self {
        let data = Vec::with_capacity(scales*samples);
        Self {
            scales,
            samples,
            data
        }
    }

    /// Get the number of scales
    #[inline]
    pub fn num_scales(&self) -> usize {
        self.scales
    }

    /// Get the number of samples per scale
    #[inline]
    pub fn num_samples(&self) -> usize {
        self.samples
    }

    #[inline]
    pub fn rows(&self) -> &Vec<Vec<Complex<f32>>> {
        &self.data
    }

    #[inline]
    pub fn push_row(&mut self, value: Vec<Complex<f32>>) {
        self.data.push(value);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn normalize(&mut self) {
        self.data.par_iter_mut().for_each(|row| {
            let size = row.len() as f32;

            row.par_iter_mut().for_each(|field| {
                field.unscale(size as f32);
            });
        });
    }

}

impl Index<usize> for CwtResult<f32> {
    type Output = [Complex<f32>];
    
    fn index(&self, y: usize) -> &Self::Output {
        &self.data[y]
    }
}

impl IndexMut<usize> for CwtResult<f32> {    
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        &mut self.data[y]
    }
}
