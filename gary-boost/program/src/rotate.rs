use gary_boost_api::prelude::*;
use solana_program::keccak::hashv;
use steel::*;

/// Rotates the active boost to a randomly selected boost in the directory.
pub fn process_rotate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts
    let clock = Clock::get()?;
    let [signer_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let config = config_info.as_account_mut::<Config>(&gary_boost_api::ID)?;

    // Silent error
    if clock.unix_timestamp < config.ts + ROTATION_DURATION {
        return Ok(());
    }

    // Sample random number
    let noise = &config.noise[..8];
    let random_number = u64::from_le_bytes(noise.try_into().unwrap()) as usize;

    // Activate a boost.
    if config.len > 0 {
        let boost = config.boosts[random_number % config.len as usize];
        config.current = boost;
    }

    // Update the noise
    config.noise = hashv(&[&config.noise]).0;
    config.ts = clock.unix_timestamp;

    Ok(())
}
