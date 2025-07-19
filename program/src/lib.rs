mod claim;
mod close;
mod initialize;
mod mine;
mod open;
mod reset;
mod update;

use claim::*;
use close::*;
use initialize::*;
use mine::*;
use open::*;
use reset::*;
use update::*;

use gary_api::instruction::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&gary_api::ID, program_id, data)?;

    match ix {
        GaryInstruction::Claim => process_claim(accounts, data)?,
        GaryInstruction::Close => process_close(accounts, data)?,
        GaryInstruction::Mine => process_mine(accounts, data)?,
        GaryInstruction::Open => process_open(accounts, data)?,
        GaryInstruction::Reset => process_reset(accounts, data)?,
        GaryInstruction::Update => process_update(accounts, data)?,
        GaryInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
