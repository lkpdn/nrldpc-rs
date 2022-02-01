use libm::{ceil, floor, log2, pow};
use std::cmp::max;

use crate::consts::bg::*;
use crate::consts::mcs;
use crate::consts::phy::select_tbs_lt_3824;
use crate::params;

// TS 38.212 - 7.2.2
pub fn select_bg(tb_size: usize, coding_rate: f32) -> BG {
    if coding_rate <= 0.250 {
        return BG::BG2;
    }
    if tb_size <= 292 {
        return BG::BG2;
    }
    if tb_size <= 3824 && coding_rate <= 0.670 {
        return BG::BG2;
    }
    return BG::BG1;
}

pub fn crc_size(tb_size: usize) -> usize {
    return if tb_size > 3824 { 24 } else { 16 };
}

// TS 38.212 - 5.2.2
pub fn num_segments(bg: BG, tb_size: usize) -> usize {
    let crc_size = crc_size(tb_size);
    let max_cb_size = if bg == BG::BG1 { 8448 } else { 3840 };
    if tb_size + crc_size > max_cb_size {
        return (tb_size + crc_size + max_cb_size - 24 - 1) / (max_cb_size - 24);
    }
    return 1;
}

// TS 38.212 - 5.2.2
pub fn k_dash(tb_size: usize, coding_rate: f32) -> usize {
    let crc_size = crc_size(tb_size);
    let bg = select_bg(tb_size, coding_rate);
    let num_segments = num_segments(bg, tb_size);
    return if num_segments == 1 {
        tb_size + crc_size
    } else {
        // TODO
        ((tb_size + crc_size + num_segments - 1) / num_segments) + 24
    };
}

// TS 38.212 - 5.3.2
pub fn select_lifting_size(kb: usize, k_dash: usize) -> usize {
    let mut min_lft_size = usize::MAX;
    for (_, lft_sizes) in LIFTING_SIZES.iter().enumerate() {
        for sz in lft_sizes.iter() {
            if kb * sz >= k_dash {
                if *sz < min_lft_size {
                    min_lft_size = *sz;
                }
                break;
            }
        }
    }
    return min_lft_size;
}

pub fn calc_kb_and_z(tb_size: usize, coding_rate: f32) -> (usize, usize) {
    let crc_size = crc_size(tb_size);
    let k_dash = k_dash(tb_size, coding_rate);
    let bg = select_bg(tb_size, coding_rate);
    match bg {
        BG::BG1 => {
            let kb = 22;
            let z = select_lifting_size(kb, k_dash);
            return (kb, z);
        }
        BG::BG2 => {
            let kb = if tb_size + crc_size > 640 {
                10
            } else if tb_size + crc_size > 560 {
                9
            } else if tb_size + crc_size > 192 {
                8
            } else {
                6
            };
            let z = select_lifting_size(kb, k_dash);
            return (kb, z);
        }
    }
}

// TS 38.214 - 5.1.3.2
pub fn tbs_lbrm() -> usize {
    let mcs_table_idx = params::MCS_TABLE_IDX;
    let mcs_idx = params::MCS_IDX;
    let modulation_order = mcs::modulation_order(mcs_table_idx, mcs_idx);
    let target_code_rate = mcs::coding_rate(mcs_table_idx, mcs_idx);
    let n_layers = params::NUM_LAYERS;

    if (mcs_table_idx == 2 && mcs_idx <= 27) || mcs_idx <= 28 {
        let n_re_dash = 0;
        let n_re = 0;
        let n_info = ((n_re * modulation_order * n_layers) as f32 * target_code_rate) as usize;

        // TODO: scaling factor if Paging or RAR

        if n_info <= 3824 {
            let n = max(3, floor(log2(n_info as f64)) as u32 - 6);
            let n_info_dash = max(
                24,
                pow(2., n as f64) as usize * floor(n_info as f64 / pow(2., n as f64)) as usize,
            );
            return select_tbs_lt_3824(n_info_dash);
        } else {
            let n = floor(log2((n_info - 24) as f64)) as u32 - 5;
            let n_info_dash = max(
                3824,
                pow(2., n as f64) as usize
                    * floor((n_info - 24) as f64 / pow(2., n as f64)) as usize,
            );
            if target_code_rate <= 0.25 {
                let c = ceil((n_info_dash + 24) as f64 / 3816.) as usize;
                return 8 * c * ceil((n_info_dash + 24) as f64 / (8 * c) as f64) as usize - 24;
            } else if n_info_dash > 8424 {
                let c = ceil((n_info_dash + 24) as f64 / 8424.) as usize;
                return 8 * c * ceil((n_info_dash + 24) as f64 / (8 * c) as f64) as usize - 24;
            } else {
                return 8 * ceil((n_info_dash + 24) as f64 / 8.) as usize - 24;
            }
        };
    } else {
        // TODO: explicit TBS setting (reserved target coding rate and SE).
        panic!("unimplemented");
    }
}

pub fn rv_starting_position(rv_id: usize, bg: BG, n_cb: usize, z_c: usize) -> usize {
    if rv_id == 0 {
        return 0;
    } else if rv_id == 1 {
        if bg == BG::BG1 {
            return floor((17 * n_cb) as f64 / (66 * z_c) as f64) as usize * z_c;
        } else {
            return floor((13 * n_cb) as f64 / (50 * z_c) as f64) as usize * z_c;
        }
    } else if rv_id == 2 {
        if bg == BG::BG1 {
            return floor((33 * n_cb) as f64 / (66 * z_c) as f64) as usize * z_c;
        } else {
            return floor((25 * n_cb) as f64 / (50 * z_c) as f64) as usize * z_c;
        }
    } else if rv_id == 3 {
        if bg == BG::BG1 {
            return floor((56 * n_cb) as f64 / (66 * z_c) as f64) as usize * z_c;
        } else {
            return floor((43 * n_cb) as f64 / (50 * z_c) as f64) as usize * z_c;
        }
    }
    panic!("rv_id={} bg={:?} n_cb={} z_c={}", rv_id, bg, n_cb, z_c);
}
