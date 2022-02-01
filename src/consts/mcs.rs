pub struct Mcs {
    pub order: usize,
    pub rate: f32,
}

pub fn coding_rate(mcs_table_idx: usize, mcs_idx: usize) -> f32 {
    if mcs_table_idx == 1 {
        return MCS_TABLE_1[mcs_idx].rate / 1024.;
    } else if mcs_table_idx == 2 {
        return MCS_TABLE_2[mcs_idx].rate / 1024.;
    } else if mcs_table_idx == 3 {
        return MCS_TABLE_3[mcs_idx].rate / 1024.;
    } else if mcs_table_idx == 4 {
        return MCS_TABLE_4[mcs_idx].rate / 1024.;
    } else if mcs_table_idx == 5 {
        return MCS_TABLE_5[mcs_idx].rate / 1024.;
    }
    panic!(
        "Unsupported mcs table idx/mcs_idx! ({}/{})",
        mcs_table_idx, mcs_idx
    );
}

pub fn modulation_order(mcs_table_idx: usize, mcs_idx: usize) -> usize {
    if mcs_table_idx == 1 {
        return MCS_TABLE_1[mcs_idx].order;
    } else if mcs_table_idx == 2 {
        return MCS_TABLE_2[mcs_idx].order;
    } else if mcs_table_idx == 3 {
        return MCS_TABLE_3[mcs_idx].order;
    } else if mcs_table_idx == 4 {
        return MCS_TABLE_4[mcs_idx].order;
    } else if mcs_table_idx == 5 {
        return MCS_TABLE_5[mcs_idx].order;
    }
    panic!(
        "Unsupported mcs table idx/mcs_idx! ({}/{})",
        mcs_table_idx, mcs_idx
    );
}

/* TS 38.214 Table 5.1.3.1-1 */
#[allow(dead_code)]
#[rustfmt::skip]
pub static MCS_TABLE_1: [Mcs; 29] = [
    Mcs { order: 2,   rate: 120. },   /* MCS Index = 0,  SE = 0.2344 */
    Mcs { order: 2,   rate: 157. },   /* MCS Index = 1,  SE = 0.3066 */
    Mcs { order: 2,   rate: 193. },   /* MCS Index = 2,  SE = 0.3770 */
    Mcs { order: 2,   rate: 251. },   /* MCS Index = 3,  SE = 0.4902 */
    Mcs { order: 2,   rate: 308. },   /* MCS Index = 4,  SE = 0.6016 */
    Mcs { order: 2,   rate: 379. },   /* MCS Index = 5,  SE = 0.7402 */
    Mcs { order: 2,   rate: 449. },   /* MCS Index = 6,  SE = 0.8770 */
    Mcs { order: 2,   rate: 526. },   /* MCS Index = 7,  SE = 1.0273 */
    Mcs { order: 2,   rate: 602. },   /* MCS Index = 8,  SE = 1.1758 */
    Mcs { order: 2,   rate: 679. },   /* MCS Index = 9,  SE = 1.3262 */
    Mcs { order: 4,   rate: 340. },   /* MCS Index = 10, SE = 1.3281 */
    Mcs { order: 4,   rate: 378. },   /* MCS Index = 11, SE = 1.4766 */
    Mcs { order: 4,   rate: 434. },   /* MCS Index = 12, SE = 1.6953 */
    Mcs { order: 4,   rate: 490. },   /* MCS Index = 13, SE = 1.9141 */
    Mcs { order: 4,   rate: 553. },   /* MCS Index = 14, SE = 2.1602 */
    Mcs { order: 4,   rate: 616. },   /* MCS Index = 15, SE = 2.4063 */
    Mcs { order: 4,   rate: 658. },   /* MCS Index = 16, SE = 2.5703 */
    Mcs { order: 6,   rate: 438. },   /* MCS Index = 17, SE = 2.5664 */
    Mcs { order: 6,   rate: 466. },   /* MCS Index = 18, SE = 2.7305 */
    Mcs { order: 6,   rate: 517. },   /* MCS Index = 19, SE = 3.0293 */
    Mcs { order: 6,   rate: 567. },   /* MCS Index = 20, SE = 3.3223 */
    Mcs { order: 6,   rate: 616. },   /* MCS Index = 21, SE = 3.6094 */
    Mcs { order: 6,   rate: 666. },   /* MCS Index = 22, SE = 3.9023 */
    Mcs { order: 6,   rate: 719. },   /* MCS Index = 23, SE = 4.2129 */
    Mcs { order: 6,   rate: 772. },   /* MCS Index = 24, SE = 4.5234 */
    Mcs { order: 6,   rate: 822. },   /* MCS Index = 25, SE = 4.8164 */
    Mcs { order: 6,   rate: 873. },   /* MCS Index = 26, SE = 5.1152 */
    Mcs { order: 6,   rate: 910. },   /* MCS Index = 27, SE = 5.3320 */
    Mcs { order: 6,   rate: 948. },   /* MCS Index = 28, SE = 5.5547 */
];

/* TS 38.214 Table 5.1.3.1-2 */
#[allow(dead_code)]
#[rustfmt::skip]
pub static MCS_TABLE_2: [Mcs; 28] = [
    Mcs { order: 2,   rate: 120. },   /* MCS Index = 0,  SE = 0.2344 */
    Mcs { order: 2,   rate: 193. },   /* MCS Index = 1,  SE = 0.3770 */
    Mcs { order: 2,   rate: 308. },   /* MCS Index = 2,  SE = 0.6016 */
    Mcs { order: 2,   rate: 449. },   /* MCS Index = 3,  SE = 0.8770 */
    Mcs { order: 2,   rate: 602. },   /* MCS Index = 4,  SE = 1.1758 */
    Mcs { order: 4,   rate: 378. },   /* MCS Index = 5,  SE = 1.4766 */
    Mcs { order: 4,   rate: 434. },   /* MCS Index = 6,  SE = 1.6953 */
    Mcs { order: 4,   rate: 490. },   /* MCS Index = 7,  SE = 1.9141 */
    Mcs { order: 4,   rate: 553. },   /* MCS Index = 8,  SE = 2.1602 */
    Mcs { order: 4,   rate: 616. },   /* MCS Index = 9,  SE = 2.4063 */
    Mcs { order: 4,   rate: 658. },   /* MCS Index = 10, SE = 2.5703 */
    Mcs { order: 6,   rate: 466. },   /* MCS Index = 11, SE = 2.7305 */
    Mcs { order: 6,   rate: 517. },   /* MCS Index = 12, SE = 3.0293 */
    Mcs { order: 6,   rate: 567. },   /* MCS Index = 13, SE = 3.3223 */
    Mcs { order: 6,   rate: 616. },   /* MCS Index = 14, SE = 3.6094 */
    Mcs { order: 6,   rate: 666. },   /* MCS Index = 15, SE = 3.9023 */
    Mcs { order: 6,   rate: 719. },   /* MCS Index = 16, SE = 4.2129 */
    Mcs { order: 6,   rate: 772. },   /* MCS Index = 17, SE = 4.5234 */
    Mcs { order: 6,   rate: 822. },   /* MCS Index = 18, SE = 4.8164 */
    Mcs { order: 6,   rate: 873. },   /* MCS Index = 19, SE = 5.1152 */
    Mcs { order: 8,   rate: 682.5 },  /* MCS Index = 20, SE = 5.3320 */
    Mcs { order: 8,   rate: 711. },   /* MCS Index = 21, SE = 5.5547 */
    Mcs { order: 8,   rate: 754. },   /* MCS Index = 22, SE = 5.8906 */
    Mcs { order: 8,   rate: 797. },   /* MCS Index = 23, SE = 6.2266 */
    Mcs { order: 8,   rate: 841. },   /* MCS Index = 24, SE = 6.5703 */
    Mcs { order: 8,   rate: 885. },   /* MCS Index = 25, SE = 6.9141 */
    Mcs { order: 8,   rate: 916.5 },  /* MCS Index = 26, SE = 7.1602 */
    Mcs { order: 8,   rate: 948. },   /* MCS Index = 27, SE = 7.4063 */
];

/* TS 38.214 Table 5.1.3.1-3 */
#[allow(dead_code)]
#[rustfmt::skip]
pub static MCS_TABLE_3: [Mcs; 29] = [
    Mcs { order: 2,   rate: 30. },    /* MCS Index = 0,  SE = 0.0586 */
    Mcs { order: 2,   rate: 40. },    /* MCS Index = 1,  SE = 0.0781 */
    Mcs { order: 2,   rate: 50. },    /* MCS Index = 2,  SE = 0.0977 */
    Mcs { order: 2,   rate: 64. },    /* MCS Index = 3,  SE = 0.1250 */
    Mcs { order: 2,   rate: 78. },    /* MCS Index = 4,  SE = 0.1523 */
    Mcs { order: 2,   rate: 99. },    /* MCS Index = 5,  SE = 0.1934 */
    Mcs { order: 2,   rate: 120. },   /* MCS Index = 6,  SE = 0.2344 */
    Mcs { order: 2,   rate: 157. },   /* MCS Index = 7,  SE = 0.3066 */
    Mcs { order: 2,   rate: 193. },   /* MCS Index = 8,  SE = 0.3770 */
    Mcs { order: 2,   rate: 251. },   /* MCS Index = 9,  SE = 0.4902 */
    Mcs { order: 2,   rate: 308. },   /* MCS Index = 10, SE = 0.6016 */
    Mcs { order: 2,   rate: 379. },   /* MCS Index = 11, SE = 0.7402 */
    Mcs { order: 2,   rate: 449. },   /* MCS Index = 12, SE = 0.8770 */
    Mcs { order: 2,   rate: 526. },   /* MCS Index = 13, SE = 1.0273 */
    Mcs { order: 2,   rate: 602. },   /* MCS Index = 14, SE = 1.1758 */
    Mcs { order: 4,   rate: 340. },   /* MCS Index = 15, SE = 1.3281 */
    Mcs { order: 4,   rate: 378. },   /* MCS Index = 16, SE = 1.4766 */
    Mcs { order: 4,   rate: 434. },   /* MCS Index = 17, SE = 1.6953 */
    Mcs { order: 4,   rate: 490. },   /* MCS Index = 18, SE = 1.9141 */
    Mcs { order: 4,   rate: 553. },   /* MCS Index = 19, SE = 2.1602 */
    Mcs { order: 4,   rate: 616. },   /* MCS Index = 20, SE = 2.4063 */
    Mcs { order: 6,   rate: 438. },   /* MCS Index = 21, SE = 2.5664 */
    Mcs { order: 6,   rate: 466. },   /* MCS Index = 22, SE = 2.7305 */
    Mcs { order: 6,   rate: 517. },   /* MCS Index = 23, SE = 3.0293 */
    Mcs { order: 6,   rate: 567. },   /* MCS Index = 24, SE = 3.3223 */
    Mcs { order: 6,   rate: 616. },   /* MCS Index = 25, SE = 3.6094 */
    Mcs { order: 6,   rate: 666. },   /* MCS Index = 26, SE = 3.9023 */
    Mcs { order: 6,   rate: 719. },   /* MCS Index = 27, SE = 4.2129 */
    Mcs { order: 6,   rate: 772. },   /* MCS Index = 28, SE = 4.5234 */
];

/* TS 38.214 Table 6.1.4.1-1 */
#[allow(dead_code)]
#[rustfmt::skip]
pub static MCS_TABLE_4: [Mcs; 28] = [
    Mcs { order: 1,   rate: 240. },   /* MCS Index = 0,  SE = 0.2344 (Qm 1 or 2) */
    Mcs { order: 1,   rate: 314. },   /* MCS Index = 1,  SE = 0.3066 */
    Mcs { order: 2,   rate: 193. },   /* MCS Index = 2,  SE = 0.3770 */
    Mcs { order: 2,   rate: 251. },   /* MCS Index = 3,  SE = 0.4902 */
    Mcs { order: 2,   rate: 308. },   /* MCS Index = 4,  SE = 0.6016 */
    Mcs { order: 2,   rate: 379. },   /* MCS Index = 5,  SE = 0.7402 */
    Mcs { order: 2,   rate: 449. },   /* MCS Index = 6,  SE = 0.8770 */
    Mcs { order: 2,   rate: 526. },   /* MCS Index = 7,  SE = 1.0273 */
    Mcs { order: 2,   rate: 602. },   /* MCS Index = 8,  SE = 1.1758 */
    Mcs { order: 2,   rate: 679. },   /* MCS Index = 9,  SE = 1.3262 */
    Mcs { order: 4,   rate: 340. },   /* MCS Index = 10, SE = 1.3281 */
    Mcs { order: 4,   rate: 378. },   /* MCS Index = 11, SE = 1.4766 */
    Mcs { order: 4,   rate: 434. },   /* MCS Index = 12, SE = 1.6953 */
    Mcs { order: 4,   rate: 490. },   /* MCS Index = 13, SE = 1.9141 */
    Mcs { order: 4,   rate: 553. },   /* MCS Index = 14, SE = 2.1602 */
    Mcs { order: 4,   rate: 616. },   /* MCS Index = 15, SE = 2.4063 */
    Mcs { order: 4,   rate: 658. },   /* MCS Index = 16, SE = 2.5703 */
    Mcs { order: 6,   rate: 466. },   /* MCS Index = 17, SE = 2.7305 */
    Mcs { order: 6,   rate: 517. },   /* MCS Index = 18, SE = 3.0293 */
    Mcs { order: 6,   rate: 567. },   /* MCS Index = 19, SE = 3.3223 */
    Mcs { order: 6,   rate: 616. },   /* MCS Index = 20, SE = 3.6094 */
    Mcs { order: 6,   rate: 666. },   /* MCS Index = 21, SE = 3.9023 */
    Mcs { order: 6,   rate: 719. },   /* MCS Index = 22, SE = 4.2129 */
    Mcs { order: 6,   rate: 772. },   /* MCS Index = 23, SE = 4.5234 */
    Mcs { order: 6,   rate: 822. },   /* MCS Index = 24, SE = 4.8164 */
    Mcs { order: 6,   rate: 873. },   /* MCS Index = 25, SE = 5.1152 */
    Mcs { order: 6,   rate: 910. },   /* MCS Index = 26, SE = 5.3320 */
    Mcs { order: 6,   rate: 948. },   /* MCS Index = 27, SE = 5.5547 */
];

/* TS 38.214 Table 6.1.4.1-2 */
#[allow(dead_code)]
#[rustfmt::skip]
pub static MCS_TABLE_5: [Mcs; 28] = [
    Mcs { order: 1,   rate: 60. },    /* MCS Index = 0,  SE = 0.0586 (Qm 1 or 2) */
    Mcs { order: 1,   rate: 80. },    /* MCS Index = 1,  SE = 0.0781 (Qm 1 or 2) */
    Mcs { order: 1,   rate: 100. },   /* MCS Index = 2,  SE = 0.0977 (Qm 1 or 2) */
    Mcs { order: 1,   rate: 128. },   /* MCS Index = 3,  SE = 0.1250 (Qm 1 or 2) */
    Mcs { order: 1,   rate: 156. },   /* MCS Index = 4,  SE = 0.1523 (Qm 1 or 2) */
    Mcs { order: 1,   rate: 198. },   /* MCS Index = 5,  SE = 0.1934 (Qm 1 or 2) */
    Mcs { order: 2,   rate: 120. },   /* MCS Index = 6,  SE = 0.2344 */
    Mcs { order: 2,   rate: 157. },   /* MCS Index = 7,  SE = 0.3066 */
    Mcs { order: 2,   rate: 193. },   /* MCS Index = 8,  SE = 0.3770 */
    Mcs { order: 2,   rate: 251. },   /* MCS Index = 9,  SE = 0.4902 */
    Mcs { order: 2,   rate: 308. },   /* MCS Index = 10, SE = 0.6016 */
    Mcs { order: 2,   rate: 379. },   /* MCS Index = 11, SE = 0.7402 */
    Mcs { order: 2,   rate: 449. },   /* MCS Index = 12, SE = 0.8770 */
    Mcs { order: 2,   rate: 526. },   /* MCS Index = 13, SE = 1.0273 */
    Mcs { order: 2,   rate: 602. },   /* MCS Index = 14, SE = 1.1758 */
    Mcs { order: 2,   rate: 679. },   /* MCS Index = 15, SE = 1.3262 */
    Mcs { order: 4,   rate: 378. },   /* MCS Index = 16, SE = 1.4766 */
    Mcs { order: 4,   rate: 434. },   /* MCS Index = 17, SE = 1.6953 */
    Mcs { order: 4,   rate: 490. },   /* MCS Index = 18, SE = 1.9141 */
    Mcs { order: 4,   rate: 553. },   /* MCS Index = 19, SE = 2.1602 */
    Mcs { order: 4,   rate: 616. },   /* MCS Index = 20, SE = 2.4063 */
    Mcs { order: 4,   rate: 658. },   /* MCS Index = 21, SE = 2.5703 */
    Mcs { order: 4,   rate: 699. },   /* MCS Index = 22, SE = 2.7305 */
    Mcs { order: 4,   rate: 772. },   /* MCS Index = 23, SE = 3.0156 */
    Mcs { order: 6,   rate: 567. },   /* MCS Index = 24, SE = 3.3223 */
    Mcs { order: 6,   rate: 616. },   /* MCS Index = 25, SE = 3.6094 */
    Mcs { order: 6,   rate: 666. },   /* MCS Index = 26, SE = 3.9023 */
    Mcs { order: 6,   rate: 712. },   /* MCS Index = 27, SE = 4.5234 */
];
