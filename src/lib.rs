use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::system_instruction,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{Sysvar, rent::Rent},
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

    // route to the appropriate handler based on the instruction varient
    match instruction {
        CounterInstruction::InitializeCounter { initial_value } => {
            process_initialize_counter(program_id, accounts, initial_value)?
        }
        CounterInstruction::IncrementCounter => process_increment_counter(program_id, accounts)?,
    }

    Ok(())
}

// handler function for initializing a counter
fn process_initialize_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_space = 8;

    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    invoke(
        &system_instruction::create_account(
            payer_account.key,
            counter_account.key,
            required_lamports,
            account_space as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    let counter_data = CounterAccount {
        counter: initial_value,
    };

    let mut account_data = &mut counter_account.data.borrow_mut()[..];
    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value: {}", initial_value);
    Ok(())
}

// handle function for incrementing a counter
fn process_increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
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
    InitializeCounter { initial_value: u64 },
    IncrementCounter,
}
