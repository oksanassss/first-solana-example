use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    program_error::{PrintProgramError, ProgramError},
};
use std::fmt;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SampleError {
    DeserializationFailure,
}

impl From<SampleError> for ProgramError {
    fn from(e: SampleError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SampleError {
    fn type_of() -> &'static str {
        "SampleError"
    }
}

impl fmt::Display for SampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SampleError::DeserializationFailure => f.write_str("Error Deserializing input data"),
        }
    }
}

impl PrintProgramError for SampleError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            SampleError::DeserializationFailure => println!("Error Deserializing input data"),
        }
    }
}
