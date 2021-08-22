use crate::{instruction::ProgramInstruction, state::State};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::unpack(instruction_data)?;
    match instruction {
        ProgramInstruction::InitializeAccount => Ok(()),
        ProgramInstruction::SetName { name } => State::process_change_name(accounts, name),
        ProgramInstruction::SetSurname { surname } => {
            State::process_change_surname(accounts, surname)
        }
    }
}
