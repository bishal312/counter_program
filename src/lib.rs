use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

// entrypoint: Defines the entrypoint that routes incoming instructions.
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize instruction data using Borsh
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    Ok(())
}

// defining program state: Defines program state (account data).
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CounterAccount {
    pub counter: u64,
}

// definging instruction enum: Defines the instructions that the program can execute.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    InitializeCounter {initial_value: u64},
    IncrementCounter,
}