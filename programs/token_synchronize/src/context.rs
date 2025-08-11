use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

#[derive(Accounts)]
pub struct AmountToUiAmountContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Account owner is checked by the CPI to the token program
    pub account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApproveContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Account owner is checked by the CPI to the token program
    #[account(mut)]
    pub to: AccountInfo<'info>,

    /// CHECK: This is a raw account used as delegate
    #[account(mut)]
    pub delegate: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApproveCheckedContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Account owner is checked by the CPI to the token program
    #[account(mut)]
    pub to: AccountInfo<'info>,

    /// CHECK: Mint account
    pub mint: AccountInfo<'info>,

    /// CHECK: This is a raw account used as delegate
    #[account(mut)]
    pub delegate: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Mint account
    pub mint: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    // pub from: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}
