use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;

pub trait NRLDPCDecoder {
    fn decode(
        &self,
        bg: ArrayView2<i16>,
        code_rate: f32,
        z: usize,
        input: ArrayView1<i16>,
        max_iter: usize,
        early_termination: bool,
    ) -> Array1<u8>;
}
