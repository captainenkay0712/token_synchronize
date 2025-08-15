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
        // Populate the ExtraAccountMetaList so Token-2022 forwards our desired
        // additional account (we'll use the payer as the "config" account in tests).
        // NOTE: The payer here is the wallet public key used in tests.
        use spl_tlv_account_resolution::account::ExtraAccountMeta;

        let config_pubkey = ctx.accounts.payer.key();
        let account_metas = vec![
            // Require the specific pubkey so the token program forwards it to the hook CPI.
            // We don't require it to be signer/writable for this simple logging demo.
            ExtraAccountMeta::new_with_pubkey(&config_pubkey, false, false)?,
        ];

        let mut account_data = ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?;
        ExtraAccountMetaList::init::<ExecuteInstruction>(&mut account_data, &account_metas)?;

        Ok(())
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        msg!("Custom transfer hook invoked!");

        msg!("Config account: {}", ctx.accounts.config.key());
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
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: PDA to be created
    #[account(
        init,
        payer = payer,
        // Reserve space for up to 2 extra accounts to be added later
        space = 8 + ExtraAccountMetaList::size_of(2).unwrap(),
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}
