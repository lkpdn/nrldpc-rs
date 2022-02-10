use ndarray::s;
use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;

use super::NRLDPCEncoder;
use crate::util::cword_valid;
use crate::util::mul_shift;

pub struct NRLDPCDefaultEncoder;

impl NRLDPCEncoder for NRLDPCDefaultEncoder {
    fn encode(&self, bg: ArrayView2<i16>, z: usize, msg: ArrayView1<u8>) -> Array1<u8> {
        let (m, n) = bg.dim();
        assert_eq!(msg.dim(), (n - m) * z);
        let mut cword = Array1::<u8>::zeros(n * z);
        msg.assign_to(cword.slice_mut(s![..msg.dim()]));
        println!("cword={:?}", cword);

        let mut temp = Array1::<u8>::zeros(z);
        for i in 0..4 {
            for j in 0..(n - m) {
                let shifted =
                    mul_shift(msg.slice(s![j * z..(j + 1) * z]), *bg.get((i, j)).unwrap());
                temp ^= &shifted;
            }
        }
        let p1_sh = if *bg.get((1, n - m)).unwrap() == -1 {
            *bg.get((2, n - m)).unwrap()
        } else {
            *bg.get((1, n - m)).unwrap()
        };
        mul_shift(temp.view(), z as i16 - p1_sh)
            .assign_to(cword.slice_mut(s![(n - m) * z..(n - m + 1) * z]));
        for i in 0..3 {
            let mut temp = Array1::<u8>::zeros(z);
            for j in 0..n - m + i + 1 {
                temp ^= &mul_shift(
                    cword.slice(s![j * z..(j + 1) * z]),
                    *bg.get((i, j)).unwrap(),
                );
            }
            temp.assign_to(cword.slice_mut(s![(n - m + i + 1) * z..(n - m + i + 2) * z]));
        }
        for i in 4..m {
            let mut temp = Array1::<u8>::zeros(z);
            for j in 0..n - m + 4 {
                temp ^= &mul_shift(
                    cword.slice(s![j * z..(j + 1) * z]),
                    *bg.get((i, j)).unwrap(),
                );
            }
            temp.assign_to(cword.slice_mut(s![(n - m + i) * z..(n - m + i + 1) * z]));
        }
        assert!(cword_valid(bg, z, cword.view()));
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
    fn test_encode() {
        let msg = vec![0, 1, 1];
        let encoder = NRLDPCDefaultEncoder {};
        let cword = encoder.encode(
            unsafe { ArrayView::from_shape_ptr((5, 8), &BG_TEST_1 as *const i16) },
            1,
            Array::from_vec(msg).view(),
        );
        assert_eq!(cword.view(), array![0, 1, 1, 0, 0, 0, 1, 0]);
    }
}
