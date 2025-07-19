#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};

#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "cuda", derive(cust_core::DeviceCopy))]
pub struct ArrayVec<T, const N: usize> {
    inner: [T; N],
    len: usize
}

// todo: add error
impl<T: Default + Copy + PartialEq, const N: usize> ArrayVec<T, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(self.inner[self.len])
    }

    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if self.len == N {
            Err(())
        } else {
            self.inner[self.len] = value;
            self.len += 1;
            Ok(())
        }
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner[0..self.len].get(index)
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn try_extend_from_slice(&mut self, other: &[T]) -> Result<(), ()> {
        if self.remaining_capacity() < other.len() {
            Err(())
        } else {
            for x in other {
                self.push(*x)?;
            }
            Ok(())
        }
    }

    pub const fn remaining_capacity(&self) -> usize {
        N - self.len()
    }
    
    pub fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) -> Result<(), ()>  {
        let mut iter = iter.into_iter();
        loop {
            if let Some(x) = iter.next() {
                self.push(x)?;
            } else {
                return Ok(())
            }
        }
    }
    
    pub fn is_full(&self) -> bool {
        self.len() >= N
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    pub fn to_vec(&self) -> Vec<T> {
        self.inner[0..self.len].to_vec()
    }
    
    pub fn contains(&self, value: &T) -> bool {
        self.inner[0..self.len].contains(value)
    }

    pub fn last(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(&self.inner[self.len - 1])
        }
    }
}

impl<T: Default + Copy, const N: usize> Default for ArrayVec<T, N> {
    fn default() -> Self {
        Self {
            inner: [T::default(); N],
            len: 0
        }
    }
}

impl<T: Debug, const N: usize> Debug for ArrayVec<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ArrayVec")
            .field("inner", &&self.inner[..self.len])
            .field("len", &self.len)
            .finish()
    }
}

// impl<T, const CAP: usize> Deref for ArrayVec<T, CAP> {
//     type Target = Self;
//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         self
//     }
// }
// 
// impl<T, const CAP: usize> DerefMut for ArrayVec<T, CAP> {
//     #[inline]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self
//     }
// }