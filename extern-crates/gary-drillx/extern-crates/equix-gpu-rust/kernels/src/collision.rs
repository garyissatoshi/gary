//! Utilities for representing and finding partial sum collisions in the solver

use crate::mem::LayerDataHash;
use crate::params::NUM_BUCKETS_BIT;
use crate::{Index, SolutionItem, COLLISIONS_PER_BUCKET, NUM_BUCKETS, NUM_BUCKETS_MASK, REMAINDER_NUM_BUCKETS_MASK};
use core::cmp::Ordering;
use core::fmt::Display;
use cuda_std::thread;
use merge_sort_kernels::{lower_bound, upper_bound};

pub(crate) fn search_layer<H: LayerDataHash + Display>(
    hashes: *const H,
    indices: *const Index,
    n: usize,
    num_bits: usize,
    cmp: &impl Fn(&Index, &usize) -> Ordering,
    mut predicate: impl FnMut(H, (SolutionItem, SolutionItem)),
) {
    unsafe {
        let first_bucket = thread::index_1d() as usize;
        if first_bucket > NUM_BUCKETS / 2 {
            return;
        }
        let second_bucket = first_bucket.wrapping_neg() & NUM_BUCKETS_MASK;
        let num_bits_mask = (H::zero().not() << num_bits).not();

        let start_range_first_bucket = lower_bound(indices, 0, n, &first_bucket, cmp);
        let end_range_second_bucket = upper_bound(indices, 0, n, &second_bucket, cmp) - 1;
        let mut first_iter = start_range_first_bucket;
        let mut second_iter = end_range_second_bucket;
        // skip 0
        if first_bucket == 0 {
            let mut first_iter_0_r = first_iter + 1;
            loop {
                let h = *hashes.add(*indices.add(first_iter_0_r) as usize);
                if in_range(h, 0) && remain_buckets_bit(h) == 0 {
                    first_iter_0_r += 1;
                } else {
                    break;
                }
            }
            for i in first_iter..first_iter_0_r {
                let first_item = *indices.add(i);
                let first_hash = *hashes.add(first_item as usize);
                for j in i..first_iter_0_r.min(i+1+COLLISIONS_PER_BUCKET) {
                    let second_item = *indices.add(j);
                    let second_hash = *hashes.add(second_item as usize);
                    let sum = first_hash.wrapping_add(&second_hash);
                    if (sum & num_bits_mask) == H::zero() {
                        predicate(sum >> num_bits, (first_item as SolutionItem, second_item as SolutionItem));
                    }
                }
            }
            first_iter = first_iter_0_r;
        }
        let offset = if first_bucket == 0 { 0 } else { 1 };
        while first_iter < second_iter {
            let first_hash = *hashes.add(*indices.add(first_iter) as usize);
            let second_hash = *hashes.add(*indices.add(second_iter) as usize);
            if !in_range(first_hash, first_bucket) || !in_range(second_hash, second_bucket) {
                return;
            }
            let first_remainder = remain_buckets_bit(first_hash);
            let mut first_r = first_iter + 1;
            while first_r < second_iter {
                let h = *hashes.add(*indices.add(first_r) as usize);
                if !in_range(h, first_bucket) || remain_buckets_bit(h) != first_remainder {
                    break;
                } else {
                    first_r += 1;
                }
            }
            let first_remainder_complement =
                (first_remainder + offset).wrapping_neg() & REMAINDER_NUM_BUCKETS_MASK;

            let mut second_remainder = remain_buckets_bit(second_hash);
            while second_remainder > first_remainder_complement {
                second_iter -= 1;
                let h = *hashes.add(*indices.add(second_iter) as usize);
                if !in_range(h, second_bucket) {
                    return;
                }
                second_remainder = remain_buckets_bit(h);
            }
            while second_remainder == first_remainder_complement {
                let second_item = *indices.add(second_iter);
                let second_hash = *hashes.add(second_item as usize);
                for i in first_iter..first_r.min(first_iter + COLLISIONS_PER_BUCKET) {
                    let first_item = *indices.add(i);
                    let first_hash = *hashes.add(first_item as usize);
                    let sum = first_hash.wrapping_add(&second_hash);
                    if (sum & num_bits_mask) == H::zero() {
                        predicate(
                            sum >> num_bits,
                            (first_item as SolutionItem, second_item as SolutionItem),
                        )
                    }
                }
                second_iter -= 1;
                let h = *hashes.add(*indices.add(second_iter) as usize);
                if !in_range(h, second_bucket) {
                    return;
                }
                second_remainder = remain_buckets_bit(h);
            }
            first_iter = first_r;
        }
    }
}

fn in_range<H: LayerDataHash>(h: H, bucket: usize) -> bool {
    last_buckets_bit(h) == bucket
}

pub(crate) fn last_buckets_bit<H: LayerDataHash>(h: H) -> usize {
    h.into_usize() & NUM_BUCKETS_MASK
}

pub(crate) fn remain_buckets_bit<H: LayerDataHash>(h: H) -> usize {
    (h >> NUM_BUCKETS_BIT).into_usize() & REMAINDER_NUM_BUCKETS_MASK
}
