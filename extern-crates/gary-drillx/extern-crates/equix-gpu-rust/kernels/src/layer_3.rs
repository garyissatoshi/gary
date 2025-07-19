use crate::collision::{last_buckets_bit, search_layer};
use crate::params::EQUIHASH_N_DIV_K;
use crate::{
    Index, Layer1, Layer2, SolutionItemArray, COLLISIONS_PER_LAYER, MAX_SOLUTIONS,
    NUM_BUCKETS,
};
use arrayvec::ArrayVec;
use core::cmp::Ordering;
use cuda_std::{kernel, thread};

#[kernel]
pub unsafe fn handle_layer3(
    layer1: &Layer1,
    layer2: &Layer2,
    layer2_indices: *const Index,
    results: *mut ArrayVec<SolutionItemArray, MAX_SOLUTIONS>,
) {
    let thread_id = thread::index_1d() as usize;
    if thread_id > NUM_BUCKETS / 2 {
        return;
    }
    let results = unsafe { &mut *results };
    search_layer(
        layer2.hashes.as_ptr(),
        layer2_indices,
        COLLISIONS_PER_LAYER,
        EQUIHASH_N_DIV_K * 2,
        &|&i, val| {
            if layer2.hashes[i as usize] == 0 {
                Ordering::Less
            } else {
                last_buckets_bit(layer2.hashes[i as usize]).cmp(val)
            }
        },
        |_sum, (l2_first_item, l2_second_item)| {
            let _ = results.push([
                layer1.items[layer2.items[l2_first_item as usize].0 as usize].0,
                layer1.items[layer2.items[l2_first_item as usize].0 as usize].1,
                layer1.items[layer2.items[l2_first_item as usize].1 as usize].0,
                layer1.items[layer2.items[l2_first_item as usize].1 as usize].1,
                layer1.items[layer2.items[l2_second_item as usize].0 as usize].0,
                layer1.items[layer2.items[l2_second_item as usize].0 as usize].1,
                layer1.items[layer2.items[l2_second_item as usize].1 as usize].0,
                layer1.items[layer2.items[l2_second_item as usize].1 as usize].1,
            ]);
        },
    );
}
