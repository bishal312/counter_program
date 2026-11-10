use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
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
        count: initial_value,
    };

    // serialize and write  the data to the account
    let mut account_data = &mut counter_account.data.borrow_mut()[..];
    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value: {}", initial_value);
    Ok(())
}

// handle function for incrementing a counter
fn process_increment_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();

    // get the counter account to increment
    let counter_account = next_account_info(account_iter)?;

    // security check: verifying this program owns the account
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // get a mutable reference to the account's data
    let mut data = counter_account.data.borrow_mut();

    // deserialize the current counter value
    let mut counter_data: CounterAccount = CounterAccount::try_from_slice(&data)?;

    // increment the counter value
    counter_data.count = counter_data
        .count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    // serialize the updated data back to the account
    counter_data.serialize(&mut &mut data[..])?;

    msg!("Counter incremented to: {}", counter_data.count);
    Ok(())
}

// defining program state: Defines program state (account data).
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CounterAccount {
    pub count: u64,
}

// definging instruction enum: Defines the instructions that the program can execute.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    InitializeCounter { initial_value: u64 },
    IncrementCounter,
}


#[cfg(test)]
mod test {
    use super::*;
    use litesvm::LiteSVM;
use solana_program::example_mocks::{solana_keypair::Keypair, solana_signer::Signer};
use solana_sdk::{
        account::ReadableAccount,
        instruction::{AccountMeta, Instruction},
        message::Message,
        signature::{keypair,Signer},
        system_program,
        transaction::Transaction,
    };

    #[test]
    fn test_counter_program() {
        // creating a new instance of the solana vm for testing
        let mut svm = LiteSVM::new();

        // creating a keypair for the transaction payer
        let payer = keypair::new();

        //Fund the payer account with 1 SOL ( 1 billion lamports)
        // This is needed to pay for transactionfee and account creation
        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");

        // loading our compiled program into the test environment
        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        // loading the compiled program
        svm.add_program_from_file(
            program_id,
            "target/deploy/counter_program.so"
        ).expect("Failed to load program");

        // step1: init the counter
        let counter_keypair = keypair::new();
        let initial_value: u64 = 42;
        
        println!("Testing counter initialization...");

        // use borsh serialization for the instruction
        let init_instruction_data = borsh::to_vec(&CounterInstruction::InitializeCounter { initial_value })
            .expect("Failed to serialize instruction");

        let initialize_instruction = Instruction::new_with_bytes(
            program_id,
            &init_instruction_data,
            vec![
                // account1: Counter account (signer, writable)
                AccountMeta::new(counter_keypair.pubkey(), true),
                // account2: Payer (signer, writable)
                AccountMeta::new(payer.pubkey(), true),
                // account3: System program (not signer, not writable)
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        
    }
}