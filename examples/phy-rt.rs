use ndarray::s;

use nrldpc::nrldpc::encoder::default::NRLDPCDefaultEncoder;
use nrldpc::nrldpc::encoder::NRLDPCEncoder;
use nrldpc::phy::proc::*;

fn main() {
    let mut b_seq = vec![1; 22 * 256];

    let mut f_seqs = vec![];
    cb_segmentation(&mut b_seq, |bg, pcm, z, c, r, c_seq| {
        let encoder = NRLDPCDefaultEncoder {};
        let cword = encoder.encode(pcm, z, c_seq);
        let d_seq = cword.slice(s![2 * z..]);
        let e_seq = rate_match(bg, c, r, z, d_seq);
        let f_seq = bit_interleave(e_seq.view());
        f_seqs.push(f_seq);
    });
    let g_seq = cb_concat(&f_seqs);
    let h_seq = modulate(g_seq.view());

    /* channel (maybe AWGN) */

    let g_llrs = demodulate(h_seq.view());
    //ul_segmentation() {
    //	let llr_blk = bit_deinterleave();
    //	let llr_seq = rate_match_rx();
    //	let hard_output = nrldpc_decode(llr_seq.view());
    //}
}
