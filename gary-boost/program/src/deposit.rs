use gary_api::state::Proof;
use gary_boost_api::prelude::*;
use steel::*;

/// Deposit adds tokens to a stake account.
pub fn process_deposit(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Deposit::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, boost_info, boost_deposits_info, boost_proof_info, boost_rewards_info, mint_info, sender_info, stake_info, treasury_info, treasury_tokens_info, gary_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let boost = boost_info
        .as_account_mut::<Boost>(&gary_boost_api::ID)?
        .assert_mut(|b| b.mint == *mint_info.key)?;
    boost_deposits_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, &boost.mint)?;
    let boost_proof = boost_proof_info
        .as_account::<Proof>(&gary_api::ID)?
        .assert(|p| p.authority == *boost_info.key)?;
    boost_rewards_info
        .is_writable()?
        .as_associated_token_account(boost_info.key, &gary_api::consts::MINT_ADDRESS)?;
    mint_info.as_mint()?;
    let sender = sender_info
        .is_writable()?
        .as_associated_token_account(signer_info.key, &boost.mint)?;
    let stake = stake_info
        .as_account_mut::<Stake>(&gary_boost_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?
        .assert_mut(|s| s.boost == *boost_info.key)?;
    gary_program.is_program(&gary_api::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Accumulate personal stake rewards.
    stake.accumulate_rewards(boost, &boost_proof);
    invoke_signed(
        &gary_api::sdk::claim(
            *boost_info.key,
            *boost_rewards_info.key,
            boost_proof.balance,
        ),
        &[
            boost_info.clone(),
            boost_rewards_info.clone(),
            boost_proof_info.clone(),
            treasury_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
            gary_program.clone(),
        ],
        &gary_boost_api::ID,
        &[BOOST, boost.mint.as_ref()],
    )?;

    // Update deposit balances.
    let amount = amount.min(sender.amount());
    boost.total_deposits += amount;
    stake.balance += amount;
    stake.last_deposit_at = clock.unix_timestamp;
    transfer(
        signer_info,
        sender_info,
        boost_deposits_info,
        token_program,
        amount,
    )?;

    Ok(())
}
