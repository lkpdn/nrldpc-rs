use crc_any::CRCu16;
use crc_any::CRCu32;
use libm::{ceil, floor, sqrt};
use ndarray::concatenate;
use ndarray::s;
use ndarray::Array;
use ndarray::Array1;
use ndarray::ArrayView;
use ndarray::ArrayView1;
use ndarray::ArrayView2;
use ndarray::Axis;

use crate::consts::bg::*;
use crate::consts::mcs;
use crate::params;

use super::util::*;

pub fn cb_segmentation(
    b_seq: &mut Vec<u8>,
    mut f: impl FnMut(BG, ArrayView2<i16>, usize, usize, usize, ArrayView1<u8>),
) {
    let mcs_table_idx = params::MCS_TABLE_IDX;
    let mcs_idx = params::MCS_IDX;
    let tb_size = b_seq.len();
    let crc_size = crc_size(tb_size);
    if crc_size == 16 {
        let mut crc16 = CRCu16::crc16xmodem();
        crc16.digest(b_seq);
        let curr = crc16.get_crc();
        for i in 0..16 {
            b_seq.push((curr >> i) as u8 & 1);
        }
    } else if crc_size == 24 {
        let mut crc24 = CRCu32::crc24lte_a();
        crc24.digest(b_seq);
        let curr = crc24.get_crc();
        for i in 0..24 {
            b_seq.push((curr >> i) as u8 & 1);
        }
    } else {
        panic!("Unexpected crc size {}", crc_size);
    }

    let msg = Array::from(b_seq.clone());
    let coding_rate = mcs::coding_rate(mcs_table_idx, mcs_idx);

    let k_dash = k_dash(tb_size, coding_rate);
    let (kb, z) = calc_kb_and_z(tb_size, coding_rate);
    let bg = select_bg(tb_size, coding_rate);
    let pcm = if bg == BG::BG1 {
        let m = unsafe { ArrayView::from_shape_ptr((46, 68), get_pcm(BG::BG1, z)) };
        m.mapv(|e| if e >= 0 { e % z as i16 } else { e });
        m
    } else {
        let m = unsafe { ArrayView::from_shape_ptr((42, 52), get_pcm(BG::BG2, z)) };
        m.mapv(|e| if e >= 0 { e % z as i16 } else { e });
        m
    };
    let cb_size = if bg == BG::BG1 { 22 * z } else { 10 * z };
    let num_segments = num_segments(bg, tb_size);
    println!(
        "bg={:?} kb={} k_dash={} z={} tb_size={} cb_size={}",
        bg, kb, k_dash, z, tb_size, cb_size
    );

    let mut s = 0;

    let l = if num_segments > 1 { 24 } else { 0 };
    for r in 0..num_segments {
        // coding
        println!("r={}", r);
        let mut cb = Array1::<u8>::zeros(cb_size);
        msg.slice(s![s..s + k_dash - l])
            .assign_to(cb.slice_mut(s![..k_dash - l]));

        if l == 24 {
            // TODO: bitwise.
            let mut crc24 = CRCu32::crc24lte_b();
            crc24.digest(msg.slice(s![s..s + k_dash - l]).as_slice().unwrap());
            let curr = crc24.get_crc();
            println!("crc={:#b}", curr);
            for i in 0..24 {
                *(cb.get_mut(k_dash - 24 as usize + i).unwrap()) = (curr >> i) as u8 & 1;
            }
        }

        for i in k_dash..cb_size {
            *(cb.get_mut(i).unwrap()) = 2; // NULL
        }

        f(bg, pcm, z, num_segments, r, cb.view());

        s += k_dash - l;
    }
}

pub fn rate_match(bg: BG, c: usize, r: usize, z: usize, d_seq: ArrayView1<u8>) -> Array1<u8> {
    let mcs_table_idx = params::MCS_TABLE_IDX;
    let mcs_idx = params::MCS_IDX;
    let rv_id = params::RV_ID;
    let n_layers = params::NUM_LAYERS;

    let modulation_order = mcs::modulation_order(mcs_table_idx, mcs_idx);

    let i_lbrm = 0; /* TODO */

    let n_cb = if i_lbrm == 0 {
        d_seq.dim()
    } else {
        floor(tbs_lbrm() as f64 / (c * 2 / 3) as f64) as usize
    };
    let g = c * (d_seq.dim() - d_seq.iter().filter(|&&x| x == 2 /* NULL */).count());
    let c_dash = c; /* TODO: handle CBGTI case */
    let e_r = if r <= c_dash - (g / (n_layers * modulation_order) % c_dash) - 1 {
        n_layers
            * modulation_order
            * floor(g as f64 / (n_layers * modulation_order * c_dash) as f64) as usize
    } else {
        n_layers
            * modulation_order
            * ceil(g as f64 / (n_layers * modulation_order * c_dash) as f64) as usize
    };

    let k0 = rv_starting_position(rv_id, bg, n_cb, z);
    let mut k = 0;
    let mut j = 0;
    let mut e_seq = vec![0; e_r];
    while k < e_r {
        let e_bit = *d_seq.get((k0 + j) % n_cb).unwrap();
        if e_bit != 2
        /* NULL */
        {
            e_seq[k] = e_bit;
            k += 1;
        }
        j += 1;
    }

    return Array1::from(e_seq);
}

pub fn bit_interleave(e_seq: ArrayView1<u8>) -> Array1<u8> {
    let mcs_table_idx = params::MCS_TABLE_IDX;
    let mcs_idx = params::MCS_IDX;

    let qm = mcs::modulation_order(mcs_table_idx, mcs_idx);
    let e = e_seq.dim();
    let mut f_seq = vec![0; e];
    for j in 0..(e / qm - 1) {
        for i in 0..(qm - 1) {
            f_seq[i + j * qm] = *e_seq.get(i * (e / qm) + j).unwrap();
        }
    }
    return Array1::from(f_seq);
}

pub fn cb_concat(f_seqs: &Vec<Array1<u8>>) -> Array1<u8> {
    return concatenate(
        Axis(0),
        &f_seqs
            .iter()
            .map(|x| x.view())
            .collect::<Vec<ArrayView1<u8>>>()[..],
    )
    .unwrap();
}

pub fn modulate(g_seqs: ArrayView1<u8>) -> Array1<(f64, f64)> {
    let mcs_table_idx = params::MCS_TABLE_IDX;
    let mcs_idx = params::MCS_IDX;

    let qm = mcs::modulation_order(mcs_table_idx, mcs_idx);

    let mut h_seq = Array::from_elem(g_seqs.dim() / qm, (0., 0.));
    match qm {
        1 => {
            for i in 0..g_seqs.dim() {
                h_seq[i].0 = (1. / sqrt(2.)) * (1. - g_seqs[i] as f64);
                h_seq[i].1 = (1. / sqrt(2.)) * (1. - g_seqs[i] as f64);
            }
        }
        2 => {
            for i in 0..g_seqs.dim() / 2 {
                h_seq[i].0 = (1. / sqrt(2.)) * (1. - g_seqs[2 * i] as f64);
                h_seq[i].1 = (1. / sqrt(2.)) * (1. - g_seqs[2 * i + 1] as f64);
            }
        }
        4 => {
            for i in 0..g_seqs.dim() / 4 {
                h_seq[i].0 = (1. / sqrt(10.))
                    * (1. - 2. * g_seqs[4 * i] as f64)
                    * (2. - (1. - 2. * g_seqs[4 * i + 2] as f64));
                h_seq[i].1 = (1. / sqrt(10.))
                    * (1. - 2. * g_seqs[4 * i + 1] as f64)
                    * (2. - (1. - 2. * g_seqs[4 * i + 3] as f64));
            }
        }
        6 => {
            for i in 0..g_seqs.dim() / 6 {
                h_seq[i].0 = (1. / sqrt(42.))
                    * (1. - 2. * g_seqs[6 * i] as f64)
                    * (4.
                        - (1. - 2. * g_seqs[6 * i + 2] as f64)
                            * (2. - (1. - 2. * g_seqs[6 * i + 4] as f64)));
                h_seq[i].1 = (1. / sqrt(42.))
                    * (1. - 2. * g_seqs[6 * i + 1] as f64)
                    * (4.
                        - (1. - 2. * g_seqs[6 * i + 3] as f64)
                            * (2. - (1. - 2. * g_seqs[6 * i + 5] as f64)));
            }
        }
        8 => {
            for i in 0..g_seqs.dim() / 8 {
                h_seq[i].0 = (1. / sqrt(170.))
                    * (1. - 2. * g_seqs[8 * i] as f64)
                    * (8.
                        - (1. - 2. * g_seqs[8 * i + 2] as f64)
                            * (4.
                                - (1. - 2. * g_seqs[8 * i + 4] as f64)
                                    * (2. - (1. - 2. * g_seqs[8 * i + 6] as f64))));
                h_seq[i].1 = (1. / sqrt(170.))
                    * (1. - 2. * g_seqs[8 * i + 1] as f64)
                    * (8.
                        - (1. - 2. * g_seqs[8 * i + 3] as f64)
                            * (4.
                                - (1. - 2. * g_seqs[8 * i + 5] as f64)
                                    * (2. - (1. - 2. * g_seqs[8 * i + 7] as f64))));
            }
        }
        _ => {
            panic!("unexpected qm {}", qm);
        }
    }
    return h_seq;
}

pub fn demodulate(h_seq: ArrayView1<(f64, f64)>) -> ArrayView1<u8> {
    panic!("unimplemented");
}
