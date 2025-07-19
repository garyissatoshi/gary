#[cfg(feature = "solve")]
use hashx_cuda::HashX;
#[cfg(not(feature = "solve"))]
use hashx::HashX;

use crate::params::{HashValue, SolutionItem};

/// Compute a [`HashValue`] from a [`SolutionItem`]
#[inline(always)]
pub fn item_hash(func: &HashX, item: SolutionItem) -> HashValue {
    HashValue::from_le_bytes(
        func.hash_to_bytes(u64::from(item))[0..16]
            .try_into()
            .expect(""),
    )
}