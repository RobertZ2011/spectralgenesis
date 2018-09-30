extern crate rustfft;

use self::rustfft::num_complex::Complex32;

pub mod cpu_fft;

pub trait Fft {
    fn width(&self) -> u32;
    fn transform(&self, input: &mut [Complex32], output: &mut [Complex32]);
    fn inverse(&self, input: &mut [Complex32], output: &mut [Complex32]);
}