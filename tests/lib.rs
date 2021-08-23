use borsh::BorshSerialize;
use first_program::processor::process;
use solana_program::hash::Hash;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};

/// Sets up the Program test and initializes 'n' program_accounts
async fn setup(program_id: &Pubkey, program_accounts: &[Pubkey]) -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "first_program", // Run the BPF version with `cargo test-bpf`
        *program_id,
        processor!(process), // Run the native version with `cargo test`
    );
    for account in program_accounts {
        program_test.add_account(
            *account,
            Account {
                lamports: 5,
                data: vec![0_u8; 1024],
                owner: *program_id,
                ..Account::default()
            },
        );
    }
    program_test.start().await
}

/// Submit transaction with relevant instruction data
#[allow(clippy::ptr_arg)]
async fn submit_txn(
    program_id: &Pubkey,
    instruction_data: &Vec<Vec<u8>>,
    accounts: &[AccountMeta],
    payer: &dyn Signer,
    recent_blockhash: Hash,
    banks_client: &mut BanksClient,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            *program_id,
            instruction_data,
            accounts.to_vec(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], recent_blockhash);
    banks_client.process_transaction(transaction).await
}

#[tokio::test]
/// Initialization test
async fn test_initialize_pass() {
    let program_id = Pubkey::new_unique();
    let account_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) = setup(&program_id, &[account_pubkey]).await;

    // Initialize the account
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
/// Mint test
async fn test_set_name() {
    let program_id = Pubkey::new_unique();
    let account_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) = setup(&program_id, &[account_pubkey]).await;

    // Initialize the account
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do name
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let set_name_value = String::from("first_name");
    let set_name_instruction = vec![1u8];
    instruction_data.push(set_name_instruction);
    instruction_data.push(String::try_to_vec(&set_name_value).unwrap());

    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
    // Check the data
}

#[tokio::test]
async fn test_set_surname() {
    let program_id = Pubkey::new_unique();
    let account_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) = setup(&program_id, &[account_pubkey]).await;

    // Initialize the account
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do surname
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let set_surname_value = String::from("surname_name");
    let set_surname_instruction = vec![2u8];
    instruction_data.push(set_surname_instruction);
    instruction_data.push(String::try_to_vec(&set_surname_value).unwrap());

    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
    // Check the data
}

#[tokio::test]
async fn test_set_both_names() {
    let program_id = Pubkey::new_unique();
    let account_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) = setup(&program_id, &[account_pubkey]).await;

    // Initialize the account
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do name
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let set_name_value = String::from("first_name");
    let set_name_instruction = vec![1u8];
    instruction_data.push(set_name_instruction);
    instruction_data.push(String::try_to_vec(&set_name_value).unwrap());

    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do surname
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let set_surname_value = String::from("surname_name");
    let set_surname_instruction = vec![2u8];
    instruction_data.push(set_surname_instruction);
    instruction_data.push(String::try_to_vec(&set_surname_value).unwrap());

    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
    // Check the data
}
