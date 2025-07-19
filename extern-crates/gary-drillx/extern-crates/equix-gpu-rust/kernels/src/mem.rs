use core::ops::{Add, BitAnd, Not, Shl, Shr};
use cuda_std::vek::num_traits::{One, WrappingAdd, Zero};
use cust_core::DeviceCopy;
use crate::SolutionItem;

/// Types that can be converted to usize
/// This type is an alternative to [`Into<usize>`] because [`u32`] does not implement it.
pub trait IntoUsize {
    /// Convert to usize
    fn into_usize(self) -> usize;
}

/// Specific impl for u32 (override generic)
macro_rules! impl_into_usize {
    ($($t:ty),*) => {
        $(
            impl IntoUsize for $t {
                fn into_usize(self) -> usize {
                    self as usize
                }
            }
        )*
    };
}

impl_into_usize!(u8, u16, u32, usize, u64, u128);

pub trait LayerDataHash:
    PartialEq
    + Ord
    + Clone
    + Copy
    + DeviceCopy
    + IntoUsize
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + Add<Self, Output = Self>
    + WrappingAdd
    + BitAnd<Output = Self>
    + Zero
    + One
    + Not<Output = Self>
{
}

impl<T> LayerDataHash for T where
    T: PartialEq
        + Ord
        + Clone
        + Copy
        + DeviceCopy
        + IntoUsize
        + Shr<usize, Output = Self>
        + Shl<usize, Output = Self>
        + Add<Self, Output = Self>
        + WrappingAdd
        + BitAnd<Output = Self>
        + Zero
        + One
        + Not<Output = Self>
{
}

#[derive(Clone, Copy, cust_core::DeviceCopy)]
pub struct LayerData<const N: usize, H: LayerDataHash> {
    pub hashes: [H; N],
    pub items: [(SolutionItem, SolutionItem); N],
}

impl<const N: usize, H: LayerDataHash> LayerData<N, H> {
    pub fn set(&mut self, i: usize, h: H, packed_item: (SolutionItem, SolutionItem)) {
        self.hashes[i] = h;
        self.items[i] = packed_item;
    }
}
