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
    // account iterator
    let accounts_iter = &mut accounts.iter();

    // extraction of required accounts in order
    let counter_account = next_account_info(accounts_iter)?; // new counter account
    let payer_account = next_account_info(accounts_iter)?; // who pays for the account
    let system_program = next_account_info(accounts_iter)?; // system program for cpi

    //calculating the space
    let account_space = 8; // 8bytes for a u64

    // getting the min balance required for rent exemption
    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    //creating the counter account via cpi to the system program
    invoke(
        &system_instruction::create_account(
            payer_account.key,    // Account paying for creation
            counter_account.key,  // New account being created
            required_lamports,    // Lamports to transfer
            account_space as u64, // Space to allocate in bytes
            program_id,           // Program that will own this account (our program)
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    // initialize the counter data
    let counter_data = CounterAccount {
        counter: initial_value,
    };

    // serialize and write  the data to the account
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
