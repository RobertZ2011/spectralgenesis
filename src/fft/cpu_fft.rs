extern crate rustfft;

use self::rustfft::num_complex::Complex32;
use self::rustfft::FFTplanner;
use self::rustfft::FFT;
use fft::Fft;
use std::sync::Arc;

pub struct CpuFft {
    forward: Arc<FFT<f32>>,
    inverse: Arc<FFT<f32>>
}

impl CpuFft {
    pub fn new(size: usize) -> CpuFft {
        let mut forward_planner = FFTplanner::new(false);
        let mut inverse_planner = FFTplanner::new(true);

        CpuFft {
            forward: forward_planner.plan_fft(size),
            inverse: inverse_planner.plan_fft(size)
        }
    }
}

impl Fft for CpuFft {
    fn width(&self) -> u32 {
        return self.forward.len() as u32;
    }

    fn transform(&self, input: &mut [Complex32], output: &mut [Complex32]) {
        self.forward.process_multi(input, output);
    }

    fn inverse(&self, input: &mut [Complex32], output: &mut [Complex32]) {
        self.inverse.process_multi(input, output);
    }
}