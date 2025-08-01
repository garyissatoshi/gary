use gary_boost_api::{
    consts::STAKE,
    state::{Boost, Stake},
};
use solana_program::system_program;
use steel::*;

/// Open creates a new stake account.
pub fn process_open(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, payer_info, boost_info, mint_info, stake_info, system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    payer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&gary_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    mint_info.as_mint()?;
    stake_info.is_empty()?.is_writable()?.has_seeds(
        &[STAKE, signer_info.key.as_ref(), boost_info.key.as_ref()],
        &gary_boost_api::ID,
    )?;
    system_program.is_program(&system_program::ID)?;

    // Initialize the stake account.
    create_program_account::<Stake>(
        stake_info,
        system_program,
        payer_info,
        &gary_boost_api::ID,
        &[STAKE, signer_info.key.as_ref(), boost_info.key.as_ref()],
    )?;
    let clock = Clock::get()?;
    let stake = stake_info.as_account_mut::<Stake>(&gary_boost_api::ID)?;
    stake.authority = *signer_info.key;
    stake.balance = 0;
    stake.boost = *boost_info.key;
    stake.last_claim_at = clock.unix_timestamp;
    stake.last_deposit_at = clock.unix_timestamp;
    stake.last_withdraw_at = clock.unix_timestamp;
    stake.last_rewards_factor = boost.rewards_factor;
    stake.rewards = 0;
    stake._buffer = [0; 1024];

    // Increment the total number of stakers.
    boost.total_stakers += 1;

    Ok(())
}
