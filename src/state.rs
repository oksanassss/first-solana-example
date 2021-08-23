use arrayref::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_memory::sol_memcpy,
    program_pack::{IsInitialized, Pack, Sealed},
};

/// Initialization flag size for account state
const INITIALIZED_BYTE: usize = 1;
/// Storage for the serialized size of the UserData instance
const USERDATA_LENGTH: usize = 4;
/// Storage for the serialized UserData container
pub const USERDATA_STORAGE: usize = 1019;
/// Sum of all account state lengths
pub const USERDATA_STATE_SPACE: usize = INITIALIZED_BYTE + USERDATA_LENGTH + USERDATA_STORAGE;

#[derive(Debug, Default, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UserData {
    is_initialized: bool,
    name: String,
    surname: String,
}

impl Sealed for UserData {}

impl IsInitialized for UserData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl UserData {
    /// Check that the UserData 'state' is initialized
    pub fn set_initialized(&mut self, init_flag: bool) {
        self.is_initialized = init_flag;
    }
    /// Sets the name for the UserData
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Sets the surnam for the UserData
    pub fn set_surname(&mut self, surname: String) {
        self.surname = surname;
    }

    /// Gets the UserData name
    pub fn name_as_ref(&self) -> &String {
        &self.name
    }

    /// Gets the UserData surname
    pub fn surname_as_ref(&self) -> &String {
        &self.surname
    }
}

impl Pack for UserData {
    const LEN: usize = USERDATA_STATE_SPACE;
    #[allow(clippy::ptr_offset_with_cast)]
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, USERDATA_STATE_SPACE];
        // Setup pointers to key areas of account state data
        let (is_initialized_dst, data_len_dst, data_dst) =
            mut_array_refs![dst, INITIALIZED_BYTE, USERDATA_LENGTH, USERDATA_STORAGE];
        // Set the initialized flag
        is_initialized_dst[0] = self.is_initialized() as u8;
        // Store the core data length and serialized content
        let user_store_data = self.try_to_vec().unwrap();
        let data_len = user_store_data.len();
        if data_len < USERDATA_STORAGE {
            data_len_dst[..].copy_from_slice(&(data_len as u32).to_le_bytes());
            sol_memcpy(data_dst, &user_store_data, data_len);
        } else {
            panic!();
        }
    }
    #[allow(clippy::ptr_offset_with_cast)]
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, USERDATA_STATE_SPACE];
        // Setup pointers to key areas of account state data
        let (is_initialized_src, data_len_src, data_src) =
            array_refs![src, INITIALIZED_BYTE, USERDATA_LENGTH, USERDATA_STORAGE];
        let is_initialized = match is_initialized_src {
            [0] => false,
            [1] => true,
            _ => true,
        };

        // Get current size of content in data area
        let data_len = u32::from_le_bytes(*data_len_src) as usize;
        // If emptry, create a default
        if data_len == 0 {
            Ok(Self::default())
        } else {
            let mut data_dser = UserData::try_from_slice(&data_src[0..data_len]).unwrap();
            data_dser.set_initialized(is_initialized);
            Ok(data_dser)
        }
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
