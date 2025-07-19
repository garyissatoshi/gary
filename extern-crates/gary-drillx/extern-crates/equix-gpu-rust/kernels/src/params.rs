/// Equihash N parameter for Equi-X, number of bits used from the hash output
pub(crate) const EQUIHASH_N: usize = 112;

/// Equihash K parameter for Equi-X, the number of tree layers
pub(crate) const EQUIHASH_K: usize = 3;

/// Equihash N/(K+1) parameter for Equi-X, the number of collision bits in each layer
pub(crate) const EQUIHASH_N_DIV_K: usize = EQUIHASH_N / (EQUIHASH_K + 1);

/// Maximum number of items in the solution space
pub const MAX_ITEMS: SolutionItem = 1 << (EQUIHASH_N_DIV_K + 1);
/// Number of bits used for the hash table buckets
pub(crate) const NUM_BUCKETS_BIT: usize = EQUIHASH_N_DIV_K / 2;
/// Number of buckets in the hash table
pub const NUM_BUCKETS: usize = 1 << NUM_BUCKETS_BIT;
pub const NUM_BUCKETS_MASK: usize = NUM_BUCKETS - 1;
pub const REMAINDER_NUM_BUCKETS_MASK: usize = (1 << (EQUIHASH_N_DIV_K - NUM_BUCKETS_BIT)) - 1;

/// One item in the solution
///
/// The Equihash paper also calls these "indices", to reference the way they
/// index into a space of potential hash outputs. They form the leaf nodes in
/// a binary tree of hashes.
pub type SolutionItem = u32;

/// One hash value, computed from a [`SolutionItem`]
///
/// Must hold [`EQUIHASH_N`] bits.
pub type HashValue = u128;
pub type Index = u32;

pub const COLLISIONS_PER_LAYER: usize = (NUM_BUCKETS / 2 + 1) * COLLISIONS_PER_THREAD;
pub const COLLISIONS_PER_THREAD: usize = 60000;
pub const COLLISIONS_PER_BUCKET: usize = 3;

pub const MAX_SOLUTIONS: usize = 8;