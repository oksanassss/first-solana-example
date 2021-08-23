pub mod errors;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entry_point;

// TEMPORARY
solana_program::declare_id!("SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv");
