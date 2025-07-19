#![allow(clippy::missing_safety_doc)]
#![allow(improper_ctypes_definitions)]
#![cfg_attr(feature = "solve", no_std)]
extern crate alloc;

#[cfg(feature = "solve")]
mod layer_0;
#[cfg(feature = "solve")]
mod layer_1;
#[cfg(feature = "solve")]
mod layer_2;
#[cfg(feature = "solve")]
mod layer_3;
mod params;

#[cfg(feature = "solve")]
mod collision;
mod err;
mod hashx;
#[cfg(feature = "solve")]
pub mod mem;
mod solution;
#[cfg(feature = "solve")]
mod sorting;

pub use err::*;
#[cfg(feature = "solve")]
pub use layer_0::*;
#[cfg(feature = "solve")]
pub use layer_1::*;
#[cfg(feature = "solve")]
pub use layer_2::*;
pub use params::*;
pub use solution::*;
