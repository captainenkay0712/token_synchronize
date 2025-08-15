use anchor_lang::prelude::*;

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
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    // pub from: Account<'info, TokenAccount>,

    /// CHECK: Can be either a signer or PDA
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseAccountContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,

    /// CHECK: Destination account
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FreezeAccountContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetAccountDataSizeContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Mint account
    pub mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeAccountContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,

    /// CHECK: Rent account
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeAccount3Context<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeImmutableOwnerContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMintContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Rent account
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMint2Context<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeMintCloseAuthorityContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintToContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub to: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RevokeContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    #[account(mut)]
    pub source: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetAuthorityContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    #[account(mut)]
    pub current_authority: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    #[account(mut)]
    pub account_or_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SyncNativeContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ThawAccountContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub from: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub to: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferCheckedContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub from: AccountInfo<'info>,

    /// CHECK: Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub to: AccountInfo<'info>,

    /// CHECK: Can be either a signer or PDA
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UiAmountToAmountContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Account owner is checked by the CPI to the token program
    pub account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CpiGuardContext<'info> {
    /// CHECK: This is not dangerous because we check the key in the instruction
    pub token_program_id: AccountInfo<'info>,

    /// CHECK: Token account
    #[account(mut)]
    pub account: AccountInfo<'info>,

    /// CHECK: Account owner is checked by the CPI to the token program
    pub owner: AccountInfo<'info>,
}