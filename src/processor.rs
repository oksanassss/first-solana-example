use crate::{errors::SampleError, instruction::ProgramInstruction, state::UserData};
use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

/// Checks each tracking account to confirm it is owned by our program
/// This function assumes that the program account is always the last
/// in the array
fn check_account_ownership(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Accounts must be owned by the program.
    for account in accounts.iter().take(accounts.len() - 1) {
        if account.owner != program_id {
            msg!(
                "Fail: The user account owner is {} and it should be {}.",
                account.owner,
                program_id
            );
            return Err(ProgramError::IncorrectProgramId);
        }
    }
    Ok(())
}

/// Initialize the programs account, which is the first in accounts
fn initialize_account(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Initialize account");
    let account_info_iter = &mut accounts.iter();
    let storage_account = next_account_info(account_info_iter)?;
    let mut account_data = storage_account.data.borrow_mut();
    // Just using unpack will check to see if initialized and will
    // fail if not
    let mut account_state = UserData::unpack_unchecked(&account_data)?;
    // Where this is a logic error in trying to initialize the same
    // account more than once
    if account_state.is_initialized() {
        return Err(SampleError::AlreadyInitializedState.into());
    } else {
        account_state.set_initialized(true);
    }

    UserData::pack(account_state, &mut account_data).unwrap();
    Ok(())
}

pub fn process_change_name(accounts: &[AccountInfo], name: String) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let storage_account = next_account_info(accounts_iter)?;
    // Get the account state data block
    let mut account_data = storage_account.data.borrow_mut();
    // Get the UserData object from state block
    let mut my_user = UserData::unpack(&account_data)?;
    // Modify
    my_user.set_name(name);
    // Store UserData back in account state date
    UserData::pack(my_user, &mut account_data)?;
    Ok(())
}

pub fn process_change_surname(accounts: &[AccountInfo], surname: String) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let storage_account = next_account_info(accounts_iter)?;
    // Get the account state data block
    let mut account_data = storage_account.data.borrow_mut();
    // Get the UserData object from state block
    let mut my_user = UserData::unpack(&account_data)?;
    // Modify
    my_user.set_surname(surname);
    // Store UserData back in account state date
    UserData::pack(my_user, &mut account_data)?;
    Ok(())
}

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Ensure that the accounts being processed are program owned
    if let Err(error) = check_account_ownership(program_id, accounts) {
        return Err(error);
    }
    // Deserialze instruction and data
    let instruction = ProgramInstruction::unpack(instruction_data)?;
    match instruction {
        ProgramInstruction::InitializeAccount => initialize_account(accounts),
        ProgramInstruction::SetName { name } => process_change_name(accounts, name),
        ProgramInstruction::SetSurname { surname } => process_change_surname(accounts, surname),
    }
}
