#![allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
#![no_std]
extern crate alloc;

use core::cmp::Ordering;
use core::fmt::Display;
use cuda_std::{kernel, thread};

#[kernel]
pub unsafe fn merge_sort_gpu(a: *const u128, tmp: *mut u128, n: usize, block_length: usize) {
    unsafe {
        merge_sort_kernel(a, tmp, n, block_length, &|x, y| {
            x.cmp(y)
        });
    }
}

pub unsafe fn merge_sort_kernel<T: Clone + Display>(
    a: *const T,
    tmp: *mut T,
    n: usize,
    block_length: usize,
    cmp: &impl Fn(&T, &T) -> Ordering,
) {
    unsafe {
        let idx = thread::index_1d() as usize;
        if idx >= n {
            return;
        }
        let l = idx / block_length * block_length;
        let r = n.min(l + block_length);
        let m = r.min(l + (block_length >> 1));
        let a_idx = &*a.add(idx);
        let new_idx = if idx < m {
            // handle left part
            let mut c = idx; // greater than in left part
            c += lower_bound(a, m, r, a_idx, cmp) - m; // greater than in right part
            c
        } else {
            // handle right part
            let mut c = idx - m;
            c += upper_bound(a, l, m, a_idx, cmp); // greater or equal than in left
            c
        };
        *tmp.add(new_idx) = a_idx.clone();
    }
}

pub unsafe fn lower_bound<T: Display, K>(
    a: *const T,
    mut l: usize,
    mut r: usize,
    val: &K,
    cmp: &impl Fn(&T, &K) -> Ordering,
) -> usize {
    unsafe {
        while l < r {
            let m = l + (r - l) / 2;
            if cmp(&*a.add(m), val) == Ordering::Less {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

pub unsafe fn upper_bound<T: Display, K>(
    a: *const T,
    mut l: usize,
    mut r: usize,
    val: &K,
    cmp: &impl Fn(&T, &K) -> Ordering,
) -> usize {
    unsafe {
        while l < r {
            let m = l + (r - l) / 2;
            if cmp(&*a.add(m), val) == Ordering::Greater {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use crate::{lower_bound, upper_bound};
    use alloc::vec;
    use print_no_std::println;

    #[test]
    fn test_lower_bound() {
        let mut a = vec![1, 2, 2, 5, 6, 7, 8, 8, 11, 11, 15];
        let lb = unsafe { lower_bound(a.as_ptr(), 0, a.len(), &2, &|x, y| x.cmp(y)) };
        let ub = unsafe { upper_bound(a.as_ptr(), 0, a.len(), &11, &|x, y| x.cmp(y)) };
        assert_eq!(lb, 1);
        assert_eq!(ub, 10);
    }
}
