use steel::*;

use super::GaryAccount;

/// Proof accounts track a miner's current hash, claimable rewards, and lifetime stats.
/// Every miner is allowed one proof account which is required by the program to mine or claim rewards.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Proof {
    /// The signer authorized to use this proof.
    pub authority: Pubkey,

    /// The quantity of tokens this miner has staked or earned.
    pub balance: u64,

    /// The current mining challenge.
    pub challenge: [u8; 32],

    /// The last hash the miner provided.
    pub last_hash: [u8; 32],

    /// Timestamp of the last time this account provided a hash.
    pub last_hash_at: i64,

    /// Timestamp of the last claim.
    pub last_claim_at: i64,

    /// The keypair which has permission to submit hashes for mining.
    pub miner: Pubkey,

    /// The total lifetime hashes provided by this miner.
    pub total_hashes: u64,

    /// The total lifetime rewards distributed to this miner.
    pub total_rewards: u64,
}

account!(GaryAccount, Proof);
