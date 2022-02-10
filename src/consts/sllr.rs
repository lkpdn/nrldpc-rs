use libm::sqrt;
use ndarray::array;
use ndarray::Array1;

fn estimate_llr_16qam(area: u8, i: f64, q: f64, p: f64) -> Array1<f64> {
    let llr0 = if area & (1 << 1) == 1 {
        -4. * q / sqrt(10. * p) + 8. / 10.
    } else {
        4. * q / sqrt(10. * p) + 8. / 10.
    };
    let llr1 = if area & (1 << 0) == 1 {
        if area & (1 << 1) == 1 {
            8. * q / sqrt(10. * p) + 8. / 10.
        } else {
            8. * q / sqrt(10. * p) - 8. / 10.
        }
    } else {
        4. * q / sqrt(10. * p)
    };
    let llr2 = if area & (1 << 3) == 1 {
        -4. * i / sqrt(10. * p) + 8. / 10.
    } else {
        4. * i / sqrt(10. * p) + 8. / 10.
    };
    let llr3 = if area & (1 << 2) == 1 {
        if area & (1 << 3) == 1 {
            8. * i / sqrt(10. * p) + 8. / 10.
        } else {
            8. * i / sqrt(10. * p) - 8. / 10.
        }
    } else {
        4. * i / sqrt(10. * p)
    };
    return array![];
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_estimate_llr_16qam() {
        assert_eq!(
            estimate_llr_16qam(0000, 1, 1, sqrt(10.)),
            array![0.4, 0.4, 0.4, 0.4]
        );
    }
}
