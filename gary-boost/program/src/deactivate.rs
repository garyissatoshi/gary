use gary_boost_api::state::{Boost, Config};
use steel::*;

/// Deactivate removes a boost from the directory.
pub fn process_deactivate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info.as_account::<Boost>(&gary_boost_api::ID)?;
    let config = config_info
        .as_account_mut::<Config>(&gary_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;

    // Find and remove boost from directory
    for i in 0..(config.len as usize) {
        if config.boosts[i] == *boost_info.key {
            // Move last element to this position and decrease length
            config.boosts[i] = config.boosts[config.len as usize - 1];
            config.boosts[config.len as usize - 1] = Pubkey::default();
            config.len -= 1;
            break;
        }
    }

    Ok(())
}
