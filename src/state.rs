use crate::errors::SampleError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Unallocated,
    Invalid,
}
impl Default for State {
    fn default() -> Self {
        Self::Unallocated
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum ProgramInstruction {
    SetName { name: String },
    SetSurname { surname: String },
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UserData {
    pub name: String,
    pub surname: String,
}

impl Sealed for UserData {}

impl Pack for UserData {
    const LEN: usize = 96;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut p = src;
        UserData::deserialize(&mut p).map_err(|_| {
            msg!("Failed to deserialize name record");
            ProgramError::InvalidAccountData
        })
    }
}

impl State {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction_vec = Vec::<Vec<u8>>::try_from_slice(input).unwrap();
        match instruction_vec[0][0] {
            0 => {
                let in_string = String::try_from_slice(&instruction_vec[1]).unwrap();
                // etc...
                Ok(())
            }
            1 => {
                // etc.
                Ok()
            }
            _ => Err(SampleError::DeserializationFailure.into()),
        }
        // let instruction = ProgramInstruction::try_from_slice(input)
        //     .map_err(|_| ProgramError::InvalidInstructionData)?;

        // match instruction {
        //     ProgramInstruction::SetName { name } => {
        //         msg!("Instruction: SetName");
        //         State::process_change_name(accounts, name)?;
        //     }
        //     ProgramInstruction::SetSurname { surname } => {
        //         msg!("Instruction: SetSurname");
        //         State::process_change_name(accounts, surname)?;
        //     }
        // }

        Ok(())
    }

    pub fn process_change_name(accounts: &[AccountInfo], name: String) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let storage_account = next_account_info(accounts_iter)?;

        let mut my_user = UserData::unpack_from_slice(&storage_account.data.borrow())?;
        my_user.name = name;
        my_user.pack_into_slice(&mut storage_account.data.borrow_mut()[..UserData::LEN]);
        Ok(())
    }

    pub fn process_change_surname(accounts: &[AccountInfo], surname: String) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let storage_account = next_account_info(accounts_iter)?;

        let mut my_user = UserData::unpack_from_slice(&storage_account.data.borrow())?;
        my_user.surname = surname;
        my_user.pack_into_slice(&mut storage_account.data.borrow_mut()[..UserData::LEN]);
        Ok(())
    }
}

// // Sanity tests
// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     use std::mem;

//     #[test]
//     fn test_sanity() {
//         let program_id = Pubkey::default();
//         let key = Pubkey::default();
//         let mut lamports = 0;
//         let mut data = vec![0; mem::size_of::<u32>()];
//         let owner = Pubkey::default();
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default(),
//         );

//         let my_function = String::from("SetName");
//         let my_name =  String::from("Ozan");

//         // let mut instruction_data = Vec::<Vec<u8>>::new();
//         let instruction_data: Vec<u8> = Vec::new();

//         instruction_data.push(String::try_to_vec(&my_function).unwrap());
//         instruction_data.push(String::try_to_vec(&my_name).unwrap());

//         let accounts = vec![account];

//         State::process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         // assert_eq!(
//         //     GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//         //         .unwrap()
//         //         .counter,
//         //     1
//         // );
//         // process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         // assert_eq!(
//         //     GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//         //         .unwrap()
//         //         .counter,
//         //     2
//         // );
//     }
// }
