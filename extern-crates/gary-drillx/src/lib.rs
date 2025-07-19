#[cfg(feature = "solve")]
pub use equix_gpu_rust as equix;

#[cfg(feature = "solve")]
use equix_gpu_rust::{SolverMemory, solve, EquiXBuilder};
use equix_gpu_rust::{verify_bytes, Solution as EquixSolution};
use sha3::{Digest, Keccak256};

/// Generates a new drillx hash from a challenge and nonce.
#[inline(always)]
#[cfg(feature = "solve")]
pub fn hash(challenge: &[u8; 32], nonce: &[u8; 8]) -> Result<Hash, DrillxError> {
    let digest = digest(challenge, nonce)?;
    Ok(Hash {
        d: digest,
        h: hashv(&digest, nonce),
    })
}

/// Generates a new drillx hash from a challenge and nonce using pre-allocated memory.
#[inline(always)]
#[cfg(feature = "solve")]
pub fn hash_with_memory(
    memory: &mut SolverMemory,
    challenge: &[u8; 32],
    nonce: &[u8; 8],
) -> Result<Hash, DrillxError> {
    let digest = digest_with_memory(memory, challenge, nonce)?;
    Ok(Hash {
        d: digest,
        h: hashv(&digest, nonce),
    })
}

/// Generates drillx hashes from a challenge and nonce using pre-allocated memory.
#[inline(always)]
#[cfg(feature = "solve")]
pub fn hashes_with_memory(
    memory: &mut SolverMemory,
    challenge: &[u8; 32],
    nonce: &[u8; 8],
) -> Vec<Hash> {
    let mut hashes: Vec<Hash> = Vec::with_capacity(7);
    if let Ok(solutions) = digests_with_memory(memory, challenge, nonce) {
        for solution in solutions {
            let digest = solution.to_bytes();
            hashes.push(Hash {
                d: digest,
                h: hashv(&digest, nonce),
            });
        }
    }
    hashes
}

/// Concatenates a challenge and a nonce into a single buffer.
#[inline(always)]
pub fn seed(challenge: &[u8; 32], nonce: &[u8; 8]) -> [u8; 40] {
    let mut result = [0; 40];
    result[00..32].copy_from_slice(challenge);
    result[32..40].copy_from_slice(nonce);
    result
}

/// Constructs a keccak digest from a challenge and nonce using equix hashes.
#[inline(always)]
#[cfg(feature = "solve")]
fn digest(challenge: &[u8; 32], nonce: &[u8; 8]) -> Result<HashDigest, DrillxError> {
    let seed = seed(challenge, nonce);
    let solutions = solve(&seed).map_err(|e| DrillxError::BadEquix(e.to_string()))?;
    if solutions.is_empty() {
        return Err(DrillxError::NoSolutions);
    }
    // SAFETY: The equix solver guarantees that the first solution is always valid
    let solution = unsafe { solutions.get_unchecked(0) };
    Ok(solution.to_bytes())
}

/// Constructs a keccak digest from a challenge and nonce using equix hashes and pre-allocated memory.
#[inline(always)]
#[cfg(feature = "solve")]
fn digest_with_memory(
    memory: &mut SolverMemory,
    challenge: &[u8; 32],
    nonce: &[u8; 8],
) -> Result<HashDigest, DrillxError> {
    let seed = seed(challenge, nonce);
    let equix = EquiXBuilder::new()
        .build(&seed)
        .map_err(|e| DrillxError::BadEquix(e.to_string()))?;
    let solutions = equix
        .solve_with_memory(memory)
        .map_err(|e| DrillxError::BadEquix(e.to_string()))?;
    if solutions.is_empty() {
        return Err(DrillxError::NoSolutions);
    }
    // SAFETY: The equix solver guarantees that the first solution is always valid
    let solution = unsafe { solutions.get_unchecked(0) };
    Ok(solution.to_bytes())
}

/// Constructs a keccak digest from a challenge and nonce using equix hashes and pre-allocated memory.
#[inline(always)]
#[cfg(feature = "solve")]
fn digests_with_memory(
    memory: &mut SolverMemory,
    challenge: &[u8; 32],
    nonce: &[u8; 8],
) -> Result<Vec<EquixSolution>, DrillxError> {
    let seed = seed(challenge, nonce);
    let equix = EquiXBuilder::new()
        .build(&seed)
        .map_err(|e| DrillxError::BadEquix(e.to_string()))?;
    equix
        .solve_with_memory(memory)
        .map_err(|e| DrillxError::BadEquix(e.to_string()))
}

/// Sorts the provided digest as a list of u16 values.
#[inline(always)]
fn sorted(mut digest: HashDigest) -> HashDigest {
    unsafe {
        let u16_slice: &mut [u16; 8] = core::mem::transmute(&mut digest);
        u16_slice.sort_unstable();
        digest
    }
}

/// Calculates a hash from the provided digest and nonce.
/// The digest is sorted prior to hashing to prevent malleability.
#[inline(always)]
fn hashv(digest: &[u8; EquixSolution::NUM_BYTES], nonce: &[u8; 8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(sorted(*digest));
    hasher.update(nonce);
    hasher.finalize().into()
}

/// Returns true if the digest if valid equihash construction from the challenge and nonce.
pub fn is_valid_digest(challenge: &[u8; 32], nonce: &[u8; 8], digest: &HashDigest) -> bool {
    let seed = seed(challenge, nonce);
    verify_bytes(&seed, digest).is_ok()
}

/// Returns the number of leading zeros on a 32 byte buffer.
pub fn difficulty(hash: [u8; 32]) -> u32 {
    let mut count = 0;
    for &byte in &hash {
        let lz = byte.leading_zeros();
        count += lz;
        if lz < 8 {
            break;
        }
    }
    count
}

pub type HashDigest = [u8; EquixSolution::NUM_BYTES];

/// The result of a drillx hash
#[derive(Default, Debug)]
pub struct Hash {
    pub d: HashDigest, // digest
    pub h: [u8; 32],   // hash
}

impl Hash {
    /// The leading number of zeros on the hash
    pub fn difficulty(&self) -> u32 {
        difficulty(self.h)
    }
}

/// A drillx solution which can be efficiently validated on-chain
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Solution {
    pub d: HashDigest, // digest
    pub n: [u8; 8],    // nonce
}

impl Solution {
    /// Builds a new verifiable solution from a hash and nonce
    pub fn new(digest: HashDigest, nonce: [u8; 8]) -> Solution {
        Solution {
            d: digest,
            n: nonce,
        }
    }

    /// Returns true if the solution is valid
    pub fn is_valid(&self, challenge: &[u8; 32]) -> bool {
        is_valid_digest(challenge, &self.n, &self.d)
    }

    /// Calculates the result hash for a given solution
    pub fn to_hash(&self) -> Hash {
        let d = self.d;
        Hash {
            d: self.d,
            h: hashv(&d, &self.n),
        }
    }

    pub fn from_bytes(bytes: [u8; 24]) -> Self {
        let mut d = [0; EquixSolution::NUM_BYTES];
        let mut n = [0u8; 8];
        d.copy_from_slice(&bytes[..16]);
        n.copy_from_slice(&bytes[16..]);
        Solution { d, n }
    }

    pub fn to_bytes(&self) -> [u8; 24] {
        let mut bytes = [0; 24];
        bytes[..16].copy_from_slice(&self.d);
        bytes[16..].copy_from_slice(&self.n);
        bytes
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DrillxError {
    #[error("Equix error: {0}")]
    BadEquix(String),
    #[error("No solution found")]
    NoSolutions,
}

#[cfg(feature = "default")]
#[cfg(test)]
mod tests {
    use crate::{hashes_with_memory, Solution};
    use equix_gpu_rust::solver::SolverMemory;

    #[test]
    fn test() {
        let _ctx = cust::quick_init().unwrap();
        let mut memory = SolverMemory::new().unwrap();
        let hxs = hashes_with_memory(&mut memory, &[0; 32], &[0; 8]);
        let hx = &hxs[0];
        let solution = Solution::new(hx.d, [0; 8]);
        assert!(solution.is_valid(&[0; 32]));
        dbg!(&hx);
    }
}
