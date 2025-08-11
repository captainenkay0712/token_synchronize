use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_program;
use anchor_lang::prelude::ProgramError;
use anchor_lang::ToAccountInfo;
use solana_program_test::*;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

// Import the program
use token_synchronize::*;

#[tokio::test]
async fn test_transfer_sol() {
    // Khởi tạo test context
    let program_id = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let mut program_test = ProgramTest::default();
    let payer = Keypair::new();
    let user1 = Keypair::new();
    let user2 = Keypair::new();
    program_test.add_account(
        user1.pubkey(),
        solana_sdk::account::Account {
            lamports: 10_000_000_000,
            ..solana_sdk::account::Account::default()
        },
    );
    program_test.add_account(
        user2.pubkey(),
        solana_sdk::account::Account {
            lamports: 0,
            ..solana_sdk::account::Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    // Gửi 1 SOL từ user1 sang user2
    // (Giả lập call CPI, kiểm tra balance)
}


#[tokio::test]
async fn test_transfer_spl_token() {
    // Khởi tạo test context cho SPL Token
    // (Tạo mint, account, mint token cho user1, call transfer, kiểm tra balance)
}


#[tokio::test]
async fn test_approve_and_bumps() {
    // Khởi tạo test context cho approve và approve_bumps
    // (Tạo PDA, dùng PDA làm authority, call approve, kiểm tra delegate)
}


#[tokio::test]
async fn test_mint() {
    // Khởi tạo test context cho mint
    // (Tạo mint, account, call mint, kiểm tra balance)
}

