use libm::{exp2, log, log2};
use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use ndarray::Axis;

use super::NRLDPCDecoder;
use crate::util::cword_valid;
use crate::util::mul_shift;

/* Richardson-Novichkov decoder */
pub struct NRLDPCRNDecoder {
    pub c1: usize,
    pub c2: usize,
}

impl NRLDPCDecoder for NRLDPCRNDecoder {
    fn decode(
        &self,
        bg: ArrayView2<i16>,
        code_rate: f32,
        z: usize,
        channel_llr: ArrayView1<f64>,
        max_iter: usize,
        early_termination: bool,
    ) -> Array1<u8> {
        let mut cword;
        let (m, n) = bg.dim();

        let n_rm = ((n - m) as f32 / code_rate).ceil() as usize + 2;
        let m_rm = n_rm - (n - m);
        assert_eq!(channel_llr.dim(), n_rm * z);

        let slen = bg.iter().filter(|&n| *n != -1).count();
        let mut treg = Array2::<f64>::zeros((slen, z));
        let mut stored = Array2::<f64>::zeros((slen, z));
        let mut output = channel_llr.to_owned();

        let mut itr = 0;
        let mut ri;
        let mut ti;
        let delta: f64 = log(2.);
        while itr < max_iter {
            ri = 0;
            for i in 0..m_rm {
                ti = 0;
                for (j, _) in bg
                    .slice(s![i, ..n_rm])
                    .iter()
                    .enumerate()
                    .filter(|&(_, v)| *v != -1)
                {
                    let mut sl = output.slice_mut(s![j * z..(j + 1) * z]);
                    sl -= &stored.slice(s![ri, 0..z]);
                    mul_shift(sl.view(), *bg.get((i, j)).unwrap())
                        .assign_to(treg.index_axis_mut(Axis(0), ti));
                    ti += 1;
                    ri += 1;
                }
                for k in 0..z {
                    // TODO: change all exp2 below to bit shift operation
                    let sum_inner: f64 = treg
                        .slice(s![..ti, k])
                        .iter()
                        .map(|&x| exp2(-1. * x.abs() / delta))
                        .sum();
                    let alpha_all: f64 = treg
                        .slice(s![..ti, k])
                        .iter()
                        .map(|&x| if x == 0. { 1. } else { x.signum() })
                        .product();
                    for v in treg.slice_mut(s![..ti, k]).iter_mut() {
                        let alpha = if *v == 0. { 1. } else { v.signum() };
                        *v = (alpha_all * alpha)
                            * (self.c1 as f64
                                - log2(sum_inner - exp2(-1. * v.abs() / delta) + self.c2 as f64)
                                    .round());
                    }
                }
                ri -= ti;
                ti = 0;
                for (j, _) in bg
                    .slice(s![i, ..n_rm])
                    .iter()
                    .enumerate()
                    .filter(|&(_, v)| *v != -1)
                {
                    mul_shift(treg.slice(s![ti, ..]), z as i16 - *bg.get((i, j)).unwrap())
                        .assign_to(stored.slice_mut(s![ri, ..]));
                    let mut sl = output.slice_mut(s![j * z..(j + 1) * z]);
                    sl += &stored.slice(s![ri, 0..z]);
                    ti += 1;
                    ri += 1;
                }
            }
            if early_termination {
                cword = output.map(|x| if *x < 0. { 1 } else { 0 });
                if cword_valid(bg, z, cword.view()) {
                    return cword;
                }
            }
            itr += 1;
        }
        cword = output.map(|x| if *x < 0. { 1 } else { 0 });
        return cword;
    }
}

impl Default for NRLDPCRNDecoder {
    fn default() -> NRLDPCRNDecoder {
        NRLDPCRNDecoder { c1: 0, c2: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::bg::*;
    use ndarray::array;
    use ndarray::Array;
    use ndarray::ArrayView;

    #[test]
    fn test_decode() {
        let input = vec![-3., 4., -1., 10., -5., 2., 0., 1.];
        let decoder = NRLDPCRNDecoder::default();
        let cword = decoder.decode(
            unsafe { ArrayView::from_shape_ptr((5, 8), &BG_TEST_1 as *const i16) },
            0.5,
            1,
            Array::from_vec(input).view(),
            10,
            false,
        );
        assert_eq!(cword.view(), array![0, 0, 0, 0, 1, 0, 1, 0].view());
    }
}
