use gary_drillx::Solution;
use gary_api::prelude::*;
use gary_pool_api::prelude::*;
use steel::*;

/// Submit sends the pool's best hash to the GARY mining contract.
pub fn process_submit(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let args = Submit::try_from_bytes(data)?;

    // Load accounts.
    let (required_accounts, boost_accounts) = accounts.split_at(9);
    let [signer_info, bus_info, config_info, pool_info, proof_info, gary_program, system_program, instructions_sysvar, slot_hashes_sysvar] =
        required_accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let pool = pool_info
        .as_account_mut::<Pool>(&gary_pool_api::ID)?
        .assert_mut(|p| p.authority == *signer_info.key)?;
    let proof = proof_info
        .is_writable()?
        .as_account::<Proof>(&gary_api::ID)?
        .assert(|p| p.authority == *pool_info.key)?;
    gary_program.is_program(&gary_api::ID)?;
    system_program.is_program(&system_program::ID)?;
    instructions_sysvar.is_sysvar(&sysvar::instructions::ID)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Build instruction for submitting solution to the GARY program
    let solution = Solution::new(args.digest, args.nonce);
    let mut mine_accounts = vec![
        signer_info.clone(),
        bus_info.clone(),
        config_info.clone(),
        proof_info.clone(),
        instructions_sysvar.clone(),
        slot_hashes_sysvar.clone(),
    ];
    let [boost_info, _boost_proof_info, boost_config_info] = boost_accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    mine_accounts = [mine_accounts, boost_accounts.to_vec()].concat();

    // Invoke mine CPI
    solana_program::program::invoke(
        &gary_api::sdk::mine(
            *signer_info.key,
            *pool_info.key,
            *bus_info.key,
            solution,
            *boost_info.key,
            *boost_config_info.key,
        ),
        &mine_accounts,
    )?;

    // Update pool state.
    pool.attestation = args.attestation;
    pool.last_hash_at = proof.last_hash_at;
    pool.last_total_members = pool.total_members;
    pool.total_submissions += 1;

    Ok(())
}
