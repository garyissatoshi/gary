use crate::collision::{last_buckets_bit, remain_buckets_bit};
use crate::mem::LayerDataHash;
use crate::Index;
use core::cmp::Ordering;
use cuda_std::{kernel, thread};

#[kernel]
pub(crate) unsafe fn reset_indices(a: *mut Index, n: usize) {
    unsafe {
        let idx = thread::index_1d() as usize;
        if idx >= n {
            return;
        }
        a.add(idx).write(idx as Index);
    }
}

pub(crate) fn cmp_hashes<H: LayerDataHash>(hx: H, hy: H) -> Ordering {
    // sort by last `NUM_BUCKETS_BIT` bits, then the remainder bits to fit `EQUIHASH_N_DIV_K`
    let bucket_idx_x = last_buckets_bit(hx);
    let bucket_idx_y = last_buckets_bit(hy);
    if bucket_idx_x != bucket_idx_y {
        bucket_idx_x.cmp(&bucket_idx_y)
    } else {
        let r_x = remain_buckets_bit(hx);
        let r_y = remain_buckets_bit(hy);
        r_x.cmp(&r_y)
    }
}

pub(crate) fn cmp_layer_hash<H: LayerDataHash>(hx: H, hy: H) -> Ordering {
    if hx == H::zero() || hy == H::zero() {
        hx.cmp(&hy)
    } else {
        cmp_hashes(hx, hy)
    }
}
