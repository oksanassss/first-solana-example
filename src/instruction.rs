use crate::errors::SampleError;

use {borsh::BorshDeserialize, solana_program::program_error::ProgramError};

#[derive(Debug, PartialEq)]
/// All custom program instructions
pub enum ProgramInstruction {
    InitializeAccount,
    SetName { name: String },
    SetSurname { surname: String },
}

impl ProgramInstruction {
    /// Unpack inbound buffer to associated Instruction
    /// The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let block = Vec::<Vec<u8>>::try_from_slice(input).unwrap();
        match block[0][0] {
            0 => Ok(ProgramInstruction::InitializeAccount),
            1 => Ok(Self::SetName {
                name: String::try_from_slice(&block[1])?,
            }),
            2 => Ok(Self::SetSurname {
                surname: String::try_from_slice(&block[1])?,
            }),
            _ => Err(SampleError::DeserializationFailure.into()),
        }
    }
}
