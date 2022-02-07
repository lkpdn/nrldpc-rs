pub mod default;

use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;

pub trait NRLDPCEncoder {
    fn encode(&self, bg: ArrayView2<i16>, z: usize, msg: ArrayView1<u8>) -> Array1<u8>;
}
