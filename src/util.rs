use ndarray::Array1;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use ndarray::Axis;
use ndarray::{concatenate, s};
use num_traits;

pub fn cword_valid(bg: ArrayView2<i16>, z: usize, cword: ArrayView1<u8>) -> bool {
    let (m, n) = bg.dim();
    assert_eq!(cword.dim(), n * z);
    let mut syndrome = Array1::<u8>::zeros(m * z);
    for i in 0..m {
        for j in 0..n {
            let shifted = mul_shift(
                cword.slice(s![j * z..(j + 1) * z]),
                *bg.get((i, j)).unwrap(),
            );
            let mut win = syndrome.slice_mut(s![i * z..(i + 1) * z]);
            win ^= &shifted;
        }
    }
    println!("syndrome={:?}", syndrome);
    return syndrome.iter().all(|&b| b == 0);
}

pub fn mul_shift<T>(x: ArrayView1<T>, k: i16) -> Array1<T>
where
    T: Clone + num_traits::identities::Zero,
{
    if k == -1 {
        return Array1::<T>::zeros(x.dim());
    } else {
        return concatenate![
            Axis(0),
            x.slice(s![k as usize..]),
            x.slice(s![..k as usize])
        ];
    }
}
