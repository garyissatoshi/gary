use crate::collision::{last_buckets_bit, search_layer};
use crate::mem::LayerData;
use crate::params::{EQUIHASH_N_DIV_K, NUM_BUCKETS};
use crate::sorting::cmp_layer_hash;
use crate::{Index, Layer1, COLLISIONS_PER_LAYER, COLLISIONS_PER_THREAD};
use core::cmp::Ordering;
use cuda_std::{kernel, thread};
use merge_sort_kernels::merge_sort_kernel;

pub type Layer2 = LayerData<COLLISIONS_PER_LAYER, u64>;

#[kernel]
pub unsafe fn handle_layer2(
    prev_layer: &Layer1,
    prev_indices: *const Index,
    current_layer: *mut Layer2,
) {
    let thread_id = thread::index_1d() as usize;
    if thread_id > NUM_BUCKETS / 2 {
        return;
    }
    let current_layer = unsafe { &mut *current_layer };

    let collision_block_start = thread_id * COLLISIONS_PER_THREAD;
    let mut cnt = 0;
    search_layer(
        prev_layer.hashes.as_ptr(),
        prev_indices,
        COLLISIONS_PER_LAYER,
        EQUIHASH_N_DIV_K,
        &|&i, val| {
            if prev_layer.hashes[i as usize] == 0 {
                Ordering::Less
            } else {
                last_buckets_bit(prev_layer.hashes[i as usize]).cmp(val)
            }
        },
        |sum, loc| {
            if cnt < COLLISIONS_PER_THREAD {
                current_layer.set(collision_block_start + cnt, sum as u64, loc);
                cnt += 1;
            }
        },
    );
}

#[kernel]
pub unsafe fn reset_hashes_layer2(layer2: *mut Layer2) {
    let idx = thread::index_1d() as usize;
    if idx >= COLLISIONS_PER_LAYER {
        return;
    }
    let layer2 = unsafe { &mut *layer2 };
    layer2.hashes[idx] = 0;
}

#[kernel]
pub unsafe fn sort_layer2(
    layer2: &Layer2,
    indices: *mut Index,
    tmp: *mut Index,
    n: usize,
    block_length: usize,
) {
    unsafe {
        merge_sort_kernel(indices, tmp, n, block_length, &|&x, &y| {
            cmp_layer_hash(layer2.hashes[x as usize], layer2.hashes[y as usize])
        });
    }
}
