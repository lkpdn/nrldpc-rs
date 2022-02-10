use libm::exp;
use ndarray::s;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use ndarray::ArrayViewMut2;
use ndarray::Axis;
use ndarray::Slice;
use std::cmp::Ordering;
use std::ops::SubAssign;

use super::NRLDPCDecoder;
use crate::util::cword_valid;
use crate::util::mul_shift;

// DC (Divide and Concur)
pub struct NRLDPCDCDecoder {
    pub e_max_epsilon: f64,
}

// Devide Projection
fn projection_d(mut sl: ArrayViewMut2<f64>) -> ArrayViewMut2<f64> {
    sl.swap_axes(0, 1);
    for mut r in sl.axis_iter_mut(Axis(0)) {
        let h: Array1<f64> = r
            .iter()
            .map(|x| if *x == 0. { 1. } else { x.signum() })
            .collect();
        if h.product() == 1. {
            continue;
        }
        let i = r
            .iter()
            .enumerate()
            .min_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();
        *r.get_mut(i).unwrap() *= -1.;
    }
    sl.swap_axes(0, 1);
    sl
}

// Concur Projection
fn projection_c(treg_next: ArrayView2<f64>, ec: ArrayView1<f64>) -> Array1<f64> {
    let cnt = ec.dim();
    let mut b = Array1::<f64>::zeros(cnt);
    for r in 0..cnt {
        let num = treg_next.index_axis(Axis(1), r).iter().count();
        *b.get_mut(r).unwrap() =
            (treg_next.index_axis(Axis(1), r).sum() + *ec.get(r).unwrap()) / (num as f64 + 1.);
    }
    b
}

impl NRLDPCDecoder for NRLDPCDCDecoder {
    fn decode(
        &self,
        bg: ArrayView2<i16>,
        code_rate: f32,
        z: usize,
        ch_llr: ArrayView1<f64>,
        max_iter: usize,
        early_termination: bool,
    ) -> Array1<u8> {
        let (m, n) = bg.dim();

        let n_rm = ((n - m) as f32 / code_rate).ceil() as usize + 2;
        let m_rm = n_rm - (n - m);
        assert_eq!(ch_llr.dim(), n_rm * z);

        let sum_llr = ch_llr.sum();
        let slen = bg.iter().filter(|&n| *n != -1).count();
        let mut treg = Array2::<f64>::zeros((slen, z));

        let mut itr = 0;

        // signed probability vector
        let p: Array1<f64> = 2. * (ch_llr.mapv(exp) / (1. + ch_llr.mapv(exp))) - 1.;

        // initialize
        let mut r_m_idx = vec![(0, 0); m_rm];
        let mut r_n_idx = Array1::<Vec<usize>>::default(n_rm);
        let mut r_m_i = 0;
        for i in 0..m_rm {
            r_m_idx[i].0 = r_m_i;
            for (j, _) in bg
                .slice(s![i, ..n_rm])
                .iter()
                .enumerate()
                .filter(|&(_, v)| *v != -1)
            {
                let x = mul_shift(p.slice(s![j * z..(j + 1) * z]), *bg.get((i, j)).unwrap());
                treg.index_axis_mut(Axis(0), r_m_i).assign(&x);
                r_n_idx[j].push(r_m_i);
                r_m_i += 1;
            }
            r_m_idx[i].1 = r_m_i;
        }

        // energy constraint
        let mut ec = p.to_owned();

        // belief information
        let mut b = Array1::<f64>::zeros(n_rm * z);
        while itr < max_iter {
            let mut treg_next = treg.clone();

            // compute the overshoot
            for &(start, end) in r_m_idx.iter() {
                if start == end {
                    continue;
                }
                let sl = treg_next.slice_axis_mut(Axis(0), Slice::from(start..end));
                projection_d(sl);
            }
            let diff = treg_next - treg.clone();
            let mut q = treg.clone() + 2. * diff.clone();

            // energy constraint
            let e_max = -(1. + self.e_max_epsilon) * sum_llr;
            let ec_diff =
                -(ch_llr.t().dot(&ec) + e_max) * ch_llr.to_owned() / ch_llr.t().dot(&ch_llr);
            let ec_overshoot = ec + 2. * ec_diff.clone();

            // concur
            for (j, v) in r_n_idx.iter().enumerate() {
                let sl = q.select(Axis(0), &v[..]);
                let p_c = projection_c(sl.view(), ec_overshoot.slice(s![j * z..(j + 1) * z]));
                p_c.assign_to(b.slice_mut(s![j * z..(j + 1) * z]));
                for &si in v {
                    q.index_axis_mut(Axis(0), si).assign(&p_c);
                }
            }

            if early_termination {
                let cword = b.map(|&x| if x < 0. { 1 } else { 0 });
                if cword_valid(bg, z, cword.view()) {
                    return cword;
                }
            }

            // difference map
            q.sub_assign(&diff);
            ec = b.clone() - ec_diff;
            itr += 1;
        }
        b.map(|&x| if x < 0. { 1 } else { 0 })
    }
}

impl Default for NRLDPCDCDecoder {
    fn default() -> NRLDPCDCDecoder {
        NRLDPCDCDecoder { e_max_epsilon: 0.5 }
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
        let channel_llr = vec![1.5, 0.08, -0.2, -1.9, 1.5, -1.3, 1.1, -1.2];
        let decoder = NRLDPCDCDecoder::default();
        let cword = decoder.decode(
            unsafe { ArrayView::from_shape_ptr((5, 8), &BG_TEST_1 as *const i16) },
            0.5,
            1,
            Array::from_vec(channel_llr).view(),
            10,
            false,
        );
        assert_eq!(cword.view(), array![0, 0, 1, 1, 0, 1, 0, 0].view());
    }
}
