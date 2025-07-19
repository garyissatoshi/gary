use crate::collision::{last_buckets_bit, search_layer};
use crate::mem::LayerData;
use crate::params::EQUIHASH_N_DIV_K;
use crate::params::{COLLISIONS_PER_LAYER, NUM_BUCKETS};
use crate::sorting::cmp_layer_hash;
use crate::{HashValue, Index, COLLISIONS_PER_THREAD, MAX_ITEMS};
use cuda_std::{kernel, thread};
use merge_sort_kernels::merge_sort_kernel;

pub type Layer1 = LayerData<COLLISIONS_PER_LAYER, u128>;

#[kernel]
pub unsafe fn handle_layer1(hashes: *const HashValue, indices: *const Index, layer1: *mut Layer1) {
    unsafe {
        let thread_id = thread::index_1d() as usize;
        if thread_id > NUM_BUCKETS / 2 {
            return;
        }
        let layer1 = &mut *layer1;

        let collision_block_start = thread_id * COLLISIONS_PER_THREAD;
        let mut cnt = 0;
        search_layer(
            hashes,
            indices,
            MAX_ITEMS as usize,
            EQUIHASH_N_DIV_K,
            &|&i, val| last_buckets_bit(*hashes.add(i as usize)).cmp(val),
            |sum, item| {
                if cnt < COLLISIONS_PER_THREAD {
                    layer1.set(collision_block_start + cnt, sum, item);
                    cnt += 1;
                }
            },
        );
    }
}

#[kernel]
pub unsafe fn reset_hashes_layer1(layer1: *mut Layer1) {
    let idx = thread::index_1d() as usize;
    if idx >= COLLISIONS_PER_LAYER {
        return;
    }
    let layer1 = unsafe { &mut *layer1 };
    layer1.hashes[idx] = 0;
}

#[kernel]
pub unsafe fn sort_layer1(
    layer1: &Layer1,
    indices: *mut Index,
    tmp: *mut Index,
    n: usize,
    block_length: usize,
) {
    unsafe {
        merge_sort_kernel(indices, tmp, n, block_length, &|&x, &y| {
            cmp_layer_hash(layer1.hashes[x as usize], layer1.hashes[y as usize])
        });
    }
}
