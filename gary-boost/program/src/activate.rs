use gary_boost_api::state::*;
use steel::*;

/// Activate adds a boost to the directory.
pub fn process_activate(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, boost_info, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    boost_info.as_account::<Boost>(&gary_boost_api::ID)?;
    let config = config_info
        .as_account_mut::<Config>(&gary_boost_api::ID)?
        .assert_mut(|c| c.admin == *signer_info.key)?;

    // Check if boost is already in directory
    if config.boosts.contains(boost_info.key) {
        return Ok(());
    }

    // Add boost to directory if not found
    config.boosts[config.len as usize] = *boost_info.key;
    config.len += 1;

    Ok(())
}
