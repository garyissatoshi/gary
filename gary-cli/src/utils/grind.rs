use std::sync::atomic::{AtomicBool, Ordering};
use rand::Rng;
use solana_sdk::pubkey::Pubkey;
use gary_api::consts::MINT;
use const_crypto::ed25519;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use gary_api::prelude::PROGRAM_ID;

pub fn find_mint_grind() -> ([u8; 16], Pubkey) {
    let found = AtomicBool::new(false);
    let result = (0..10).into_par_iter().find_map_any(|_| {
        let mut rng = rand::thread_rng();
        while !found.load(Ordering::Relaxed) {
            let mut noise = [0u8; 16];
            rng.fill(&mut noise);
            let pda = Pubkey::new_from_array(
                ed25519::derive_program_address(&[MINT, &noise], &PROGRAM_ID).0,
            );

            if pda.to_string().starts_with("gary") {
                found.store(true, Ordering::Relaxed);
                return Some((noise, pda));
            }
        }
        None
    });

    result.expect("Failed to find a PDA with 'gary'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mint_grind() {
        let (noise, pda) = find_mint_grind();
        dbg!(noise, pda);
    }
}