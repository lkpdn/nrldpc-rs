use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use ndarray::Axis;

use super::NRLDPCDecoder;
use crate::util::cword_valid;
use crate::util::mul_shift;

pub struct NRLDPCAMinStarDecoder {
    pub scale: f64,
    pub offset: f64,
}

impl NRLDPCDecoder for NRLDPCAMinStarDecoder {
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

        // TODO: do we accept 2Z punctured input?
        //output.slice_mut(s![..2 * z]).fill(0);

        let mut itr = 0;
        let mut ri;
        let mut ti;
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
                    let (mut min1, mut min2) = (f64::MAX, f64::MAX);
                    let mut pos1 = 0;
                    for (l, v) in treg.slice(s![..ti, k]).iter().enumerate() {
                        if v.abs() < min1 {
                            if min1 != f64::MAX {
                                min2 = min1;
                            }
                            min1 = v.abs();
                            pos1 = l;
                        } else if v.abs() < min2 {
                            /* Notice that v.abs() may equal to min2 */
                            min2 = v.abs();
                        }
                    }
                    assert!(min1 != f64::MAX);
                    assert!(min2 != f64::MAX);
                    let sign = treg
                        .slice(s![..ti, k])
                        .map(|x| if *x == 0. { 1. } else { x.signum() })
                        .product();
                    treg.slice_mut(s![..ti, k])
                        .iter_mut()
                        .enumerate()
                        .for_each(|(l, v)| {
                            if l == pos1 {
                                *v = sign * min2;
                            } else {
                                *v = sign * min1;
                            }
                        });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::bg::*;
    use ndarray::array;
    use ndarray::Array;
    use ndarray::ArrayView;

    #[test]
    fn test_decode() {
        let input = vec![-3, 4, -1, 10, -5, 2, 0, 1];
        let output = nrldpc_decode(
            unsafe { ArrayView::from_shape_ptr((5, 8), &BG_TEST_1 as *const i16) },
            0.5,
            1,
            Array::from_vec(input).view(),
            10,
            false,
        );
        assert_eq!(output.view(), array![1, 0, 0, 0, 1, 0, 1, 0].view());
    }
}
