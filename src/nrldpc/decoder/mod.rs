mod aminstar;
mod dc;
mod rn;

use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;

pub trait NRLDPCDecoder {
    fn decode(
        &self,
        bg: ArrayView2<i16>,
        code_rate: f32,
        z: usize,
        channel_llr: ArrayView1<f64>,
        max_iter: usize,
        early_termination: bool,
    ) -> Array1<u8>;
}
