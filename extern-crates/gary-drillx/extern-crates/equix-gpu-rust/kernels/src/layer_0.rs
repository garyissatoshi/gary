use crate::hashx::item_hash;
use crate::params::{HashValue, MAX_ITEMS};
use crate::sorting::cmp_hashes;
use crate::Index;
use cuda_std::{kernel, thread};
#[cfg(feature = "solve")]
use hashx_cuda::HashX;
#[cfg(not(feature = "solve"))]
use hashx::HashX;
use merge_sort_kernels::merge_sort_kernel;

#[kernel]
pub unsafe fn handle_layer0(hashx: &HashX, hashes: *mut HashValue) {
    let item = thread::index_1d();
    if item >= MAX_ITEMS {
        return;
    }
    unsafe {
        *hashes.add(item as usize) = item_hash(hashx, item);
    }
}

#[kernel]
pub unsafe fn sort_hashes(
    hashes: *const HashValue,
    indices: *mut Index,
    tmp: *mut Index,
    n: usize,
    block_length: usize,
) {
    unsafe {
        merge_sort_kernel(indices, tmp, n, block_length, &|&x, &y| {
            cmp_hashes(*hashes.add(x as usize), *hashes.add(y as usize))
        });
    }
}
