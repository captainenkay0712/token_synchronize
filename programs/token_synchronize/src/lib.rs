pub mod context;
pub mod cpi;
pub mod error;

use anchor_lang::prelude::*;
use anchor_spl::{token, token_2022};
use anchor_spl::token::{TokenAccount};

use crate::context::*;
use crate::cpi::*;
use crate::error::{
    ErrorCode,
};

declare_id!("G2LUVS7T6NeVEcRNgGFdpnwhUKJj5MfermEu99hf7rEL");


#[program]
mod token_synchronize {
    use super::*;

    pub fn amount_to_ui_amount(ctx: Context<AmountToUiAmountContext>, amount: u64) -> Result<String> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }

        let _accounts = token_2022::AmountToUiAmount {
            account: ctx.accounts.account.to_account_info(),
        };

        let _cpi_ctx = CpiContext::new(ctx.accounts.token_program_id.to_account_info(), _accounts);

        let _result = token_2022::amount_to_ui_amount(_cpi_ctx, amount)?;
        Ok(_result)
    }

    pub fn approve(
        ctx: Context<ApproveContext>,
        amount: u64,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        if amount == 0 {
            return Err(error!(ErrorCode::ZeroAmount));
        }

        if ctx.accounts.delegate.key() == ctx.accounts.authority.key() {
            return Err(error!(ErrorCode::DelegateEqualsAuthority));
        }

        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::Approve {
                    to: ctx.accounts.to.to_account_info(),
                    delegate: ctx.accounts.delegate.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, amount, None, authority_seeds.as_ref(), |_ctx, _amount, _| token::approve(_ctx, _amount))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::Approve {
                    to: ctx.accounts.to.to_account_info(),
                    delegate: ctx.accounts.delegate.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, amount, None, authority_seeds.as_ref(), |_ctx, _amount, _| token_2022::approve(_ctx, _amount))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        msg!("{} approved {} tokens for {}", ctx.accounts.authority.key(), amount, ctx.accounts.delegate.key());
        Ok(())
    }

    pub fn approve_checked(
        ctx: Context<ApproveCheckedContext>,
        amount: u64,
        decimals: u8,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        if amount == 0 {
            return Err(error!(ErrorCode::ZeroAmount));
        }

        if *ctx.accounts.mint.to_account_info().owner != ctx.accounts.token_program_id.key() {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }

        if ctx.accounts.delegate.key() == ctx.accounts.authority.key() {
            return Err(error!(ErrorCode::DelegateEqualsAuthority));
        }

        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::ApproveChecked {
                    to: ctx.accounts.to.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    delegate: ctx.accounts.delegate.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, amount, Some(decimals), authority_seeds.as_ref(), |_ctx, _amount, _dec| token::approve_checked(_ctx, _amount, _dec.unwrap()))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::ApproveChecked {
                    to: ctx.accounts.to.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    delegate: ctx.accounts.delegate.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, amount, Some(decimals), authority_seeds.as_ref(), |_ctx, _amount, _dec| token_2022::approve_checked(_ctx, _amount, _dec.unwrap()))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn burn(
        ctx: Context<BurnContext>,
        amount: u64,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        if amount == 0 {
            return Err(error!(ErrorCode::ZeroAmount));
        }

        if ctx.accounts.from.delegate.unwrap() != ctx.accounts.authority.key() {
            return Err(error!(ErrorCode::DelegateNotAuthority));
        }
        if ctx.accounts.from.delegated_amount < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let accounts = token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.from.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, accounts, amount, None, authority_seeds.as_ref(), |_ctx, _amount, _| token::burn(_ctx, _amount))?;
            }
            _id if _id == token_2022::ID => {
                let accounts = token_2022::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.from.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, accounts, amount, None, authority_seeds.as_ref(), |_ctx, _amount, _| token_2022::burn(_ctx, _amount))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }
}
