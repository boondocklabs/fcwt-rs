use rustfft::num_complex::Complex;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const COMPLEX_ZERO: Complex<f32> = Complex::<f32> {
    re: 0.0,
    im: 0.0,
};

fn main() {
    //const INIT_CHUNK: [Complex<f32>;8192] = [COMPLEX_ZERO; 8192];

    let mut total: usize = 0;

    for _i in 0..10000u64 {
        let mut x = vec![vec![COMPLEX_ZERO; 8192]; 300];
        //let mut x = vec![INIT_CHUNK; 300];
        x[1][0] = x[0][0];
        total += x[0].len() + x.len();
    }

    println!("Total {}", total);
}