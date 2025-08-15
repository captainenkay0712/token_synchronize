use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};

declare_id!("FQpzwCkunhGsTrxsBEaMsASqTqjAuGpnLWvgeVHxKd5P");

#[program(fallback)]
pub mod custom_transfer_hook {
    use super::*;

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        use spl_tlv_account_resolution::account::ExtraAccountMeta;

        let config_pubkey = ctx.accounts.payer.key();
        let fee_vault_pubkey = ctx.accounts.mint.key();
        let account_metas = vec![
            // config
            ExtraAccountMeta::new_with_pubkey(&config_pubkey, false, false)?,
            // fee_vault
            ExtraAccountMeta::new_with_pubkey(&fee_vault_pubkey, false, false)?,
        ];

        let mut account_data = ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut account_data, &account_metas)?;

        Ok(())
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        msg!("Custom transfer hook invoked!");

        msg!("Config account: {}", ctx.accounts.config.key());
        msg!("Fee vault account: {}", ctx.accounts.fee_vault.key());
        Ok(())
    }

    pub fn fallback<'a>(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        data: &[u8],
    ) -> Result<()> {
        let instruction = TransferHookInstruction::unpack(data)?;
        match instruction {
            TransferHookInstruction::Execute { amount } => {
                let amount_bytes = amount.to_le_bytes();
                __private::__global::transfer_hook(program_id, accounts, &amount_bytes)
            },
            TransferHookInstruction::InitializeExtraAccountMetaList { .. } => Ok(()),
            TransferHookInstruction::UpdateExtraAccountMetaList { .. } => Ok(()),
        }
    }
}

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(token::mint = mint, token::authority = owner)]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList PDA
    #[account(seeds = [b"extra-account-metas", mint.key().as_ref()], bump)]
    pub extra_account_meta_list: AccountInfo<'info>,
    /// CHECK: Optional extra config account resolved/passed by caller
    pub config: UncheckedAccount<'info>,
    /// CHECK: Optional fee vault account resolved/passed by caller
    pub fee_vault: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: PDA to be created
    #[account(
        init,
        payer = payer,
        space = 8 + ExtraAccountMetaList::size_of(2).unwrap(),
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}
