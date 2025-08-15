pub mod context;
pub mod cpi;
pub mod helper;
pub mod error;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke,
    program::invoke_signed,
    system_instruction,
    system_program,
};
use anchor_spl::{token, token_2022, token_2022_extensions};
use std::convert::TryFrom;
// use spl_token_2022::extension::ExtensionType;

use crate::context::*;
use crate::cpi::*;
use crate::helper::*;
use crate::error::{
    ErrorCode,
};

declare_id!("G2LUVS7T6NeVEcRNgGFdpnwhUKJj5MfermEu99hf7rEL");


#[program]
mod token_synchronize {
    use anchor_spl::token_2022_extensions;

    use super::*;


    // --- Transfer Different Token Program Instructions ---
    pub fn transfer_unified<'info>(
        ctx: Context<'_, '_, '_, 'info, TransferCheckedContext<'info>>,
        amount: u64,
    ) -> Result<()> {
        crate::helper::spl_tokens::transfer(
            &ctx.accounts.token_program_id,
            &ctx.accounts.from,
            &ctx.accounts.mint,
            &ctx.accounts.to,
            &ctx.accounts.authority,
            &[],
            amount,
            &[],
            &ctx.remaining_accounts
        )?;

        Ok(())
    }

    // --- Default Instructions ---
    pub fn amount_to_ui_amount(ctx: Context<AmountToUiAmountContext>, amount: u64) -> Result<String> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }

        let _accounts = token_2022::AmountToUiAmount {
            account: ctx.accounts.account.to_account_info(),
        };

        let _cpi_ctx = CpiContext::new(_cpi_program, _accounts);

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
                token_cpi(_cpi_program, _accounts, Some(amount), None, None, authority_seeds.as_ref(), |_ctx, _amount, _, _| token::approve(_ctx, _amount.unwrap()))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::Approve {
                    to: ctx.accounts.to.to_account_info(),
                    delegate: ctx.accounts.delegate.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), None, None, authority_seeds.as_ref(), |_ctx, _amount, _, _| token_2022::approve(_ctx, _amount.unwrap()))?;
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
                token_cpi(_cpi_program, _accounts, Some(amount), Some(decimals), None, authority_seeds.as_ref(), |_ctx, _amount, _dec, _| token::approve_checked(_ctx, _amount.unwrap(), _dec.unwrap()))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::ApproveChecked {
                    to: ctx.accounts.to.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    delegate: ctx.accounts.delegate.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), Some(decimals), None, authority_seeds.as_ref(), |_ctx, _amount, _dec, _| token_2022::approve_checked(_ctx, _amount.unwrap(), _dec.unwrap()))?;
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

        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.from.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), None, None, authority_seeds.as_ref(), |_ctx, _amount, _, _| token::burn(_ctx, _amount.unwrap()))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.from.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), None, None, authority_seeds.as_ref(), |_ctx, _amount, _, _| token_2022::burn(_ctx, _amount.unwrap()))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn burn_checked(
        ctx: Context<BurnContext>,
        amount: u64,
        decimals: u8,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        if amount == 0 {
            return Err(error!(ErrorCode::ZeroAmount));
        }

        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }
        
        let _accounts = token_2022::BurnChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token_cpi(_cpi_program, _accounts, Some(amount), Some(decimals), None, authority_seeds.as_ref(), |_ctx, _amount, _dec, _| token_2022::burn_checked(_ctx, _amount.unwrap(), _dec.unwrap()))?;

        Ok(())
    }

    pub fn close_account(
        ctx: Context<CloseAccountContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::CloseAccount {
                    account: ctx.accounts.account.to_account_info(),
                    destination: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token::close_account(_ctx))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::CloseAccount {
                    account: ctx.accounts.account.to_account_info(),
                    destination: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::close_account(_ctx))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn freeze_account(
        ctx: Context<FreezeAccountContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::FreezeAccount {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token::freeze_account(_ctx))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::FreezeAccount {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::freeze_account(_ctx))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn initialize_account(
        ctx: Context<InitializeAccountContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::InitializeAccount {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token::initialize_account(_ctx))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::InitializeAccount {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::initialize_account(_ctx))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn initialize_account3(
        ctx: Context<InitializeAccount3Context>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::InitializeAccount3 {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token::initialize_account3(_ctx))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::InitializeAccount3 {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::initialize_account3(_ctx))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn initialize_immutable_owner(
        ctx: Context<InitializeImmutableOwnerContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }

        let _accounts = token_2022::InitializeImmutableOwner {
            account: ctx.accounts.account.to_account_info(),
        };
        
        token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::initialize_immutable_owner(_ctx))?;

        Ok(())
    }

    pub fn initialize_mint(
        ctx: Context<InitializeMintContext>,
        decimals: u8,
        authority: Pubkey,
        freeze_authority: Option<Pubkey>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                };
                cpi::initialize_mint_cpi(_cpi_program, _accounts, decimals, &authority, freeze_authority.as_ref(), authority_seeds.as_ref(), |_ctx, decimals, authority, freeze_authority, | token::initialize_mint(_ctx, decimals, authority, freeze_authority))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                };
                cpi::initialize_mint_cpi(_cpi_program, _accounts, decimals, &authority, freeze_authority.as_ref(), authority_seeds.as_ref(), |_ctx, decimals, authority, freeze_authority, | token_2022::initialize_mint(_ctx, decimals, authority, freeze_authority))?
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn initialize_mint2(
        ctx: Context<InitializeMint2Context>,
        decimals: u8,
        authority: Pubkey,
        freeze_authority: Option<Pubkey>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::InitializeMint2 {
                    mint: ctx.accounts.mint.to_account_info(),
                };
                initialize_mint_cpi(_cpi_program, _accounts, decimals, &authority, freeze_authority.as_ref(), authority_seeds.as_ref(), |_ctx, decimals, authority, freeze_authority, | token::initialize_mint2(_ctx, decimals, authority, freeze_authority))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::InitializeMint2 {
                    mint: ctx.accounts.mint.to_account_info(),
                };
                initialize_mint_cpi(_cpi_program, _accounts, decimals, &authority, freeze_authority.as_ref(), authority_seeds.as_ref(), |_ctx, decimals, authority, freeze_authority, | token_2022::initialize_mint2(_ctx, decimals, authority, freeze_authority))?
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn initialize_mint_close_authority(
        ctx: Context<InitializeMintCloseAuthorityContext>,
        close_authority: Option<Pubkey>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }

        let _accounts = token_2022::InitializeMintCloseAuthority {
            mint: ctx.accounts.mint.to_account_info(),
        };
        
        initialize_mint_close_authority_cpi(_cpi_program, _accounts, close_authority.as_ref(), authority_seeds.as_ref(), |_ctx, close_authority| token_2022::initialize_mint_close_authority(_ctx, close_authority))?;

        Ok(())
    }

    pub fn mint_to(
        ctx: Context<MintToContext>,
        amount: u64,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), None, None, authority_seeds.as_ref(), |_ctx, amount, _, _| token::mint_to(_ctx, amount.unwrap()))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), None, None, authority_seeds.as_ref(), |_ctx, amount, _, _| token_2022::mint_to(_ctx, amount.unwrap()))?
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn mint_to_checked(
        ctx: Context<MintToContext>,
        amount: u64,
        decimals: u8,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }

        let _accounts = token_2022::MintToChecked {
            authority: ctx.accounts.authority.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
        };
        
        token_cpi(_cpi_program, _accounts, Some(amount), Some(decimals), None, authority_seeds.as_ref(), |_ctx, amount, decimals, _| token_2022::mint_to_checked(_ctx, amount.unwrap(), decimals.unwrap()))?;

        Ok(())
    }

    pub fn revoke(
        ctx: Context<RevokeContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::Revoke {
                    source: ctx.accounts.source.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token::revoke(_ctx))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::Revoke {
                    source: ctx.accounts.source.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::revoke(_ctx))?
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn set_authority(
        ctx: Context<SetAuthorityContext>,
        authority_type: u8,
        new_authority: Option<Pubkey>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _authority_type_converted = parse_type::parse_authority_type(authority_type)?;

                let _accounts = token::SetAuthority {
                    current_authority: ctx.accounts.current_authority.to_account_info(),
                    account_or_mint: ctx.accounts.account_or_mint.to_account_info(),
                };
                set_authority_cpi(_cpi_program, _accounts, _authority_type_converted, new_authority, authority_seeds.as_ref(), |_ctx, auth_type, new_auth| token::set_authority(_ctx, auth_type, new_auth))?;
            }
            _id if _id == token_2022::ID => {
                let _authority_type_converted = parse_type::parse_authority_type_2022(authority_type)?;
                let _accounts = token_2022::SetAuthority {
                    current_authority: ctx.accounts.current_authority.to_account_info(),
                    account_or_mint: ctx.accounts.account_or_mint.to_account_info(),
                };
                set_authority_2022_cpi(_cpi_program, _accounts, _authority_type_converted, new_authority, authority_seeds.as_ref(), |ctx, auth_type, new_auth| token_2022::set_authority(ctx, auth_type, new_auth))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn sync_native(
        ctx: Context<SyncNativeContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::SyncNative {
                    account: ctx.accounts.account.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token::sync_native(_ctx))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::SyncNative {
                    account: ctx.accounts.account.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::sync_native(_ctx))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn thaw_account(
        ctx: Context<ThawAccountContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::ThawAccount {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token::thaw_account(_ctx))?;
            }
            _id if _id == token_2022::ID => {
                let _accounts = token_2022::ThawAccount {
                    account: ctx.accounts.account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022::thaw_account(_ctx))?;
            }
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn transfer(
        ctx: Context<TransferContext>,
        amount: u64,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), None, None, authority_seeds.as_ref(), |_ctx, amount, _, _| token::transfer(_ctx, amount.unwrap()))?;
            }
            _id if _id == token_2022::ID => return Err(error!(ErrorCode::Token2022TransferDeprecated)),
            _id if _id == system_program::ID => {
                let _transfer_instruction = system_instruction::transfer(
                    ctx.accounts.from.key,
                    ctx.accounts.to.key,
                    amount,
                );

                let _account_infos = &[
                    ctx.accounts.from.to_account_info(),
                    ctx.accounts.to.to_account_info(),
                ];

                if let Some(seeds) = authority_seeds {
                    let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
            
                    invoke_signed(
                        &_transfer_instruction,
                        _account_infos,
                        &[&seeds_vec[..]],
                    )?;
                } else {
                    invoke(
                        &_transfer_instruction,
                        _account_infos,
                    )?;
                }
            },
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn transfer_checked(
        ctx: Context<TransferCheckedContext>,
        amount: u64,
        decimals: u8,
        fee: Option<u64>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        match _token_program_id {
            _id if _id == token::ID => {
                let _accounts = token::TransferChecked {
                    from: ctx.accounts.from.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                };
                token_cpi(_cpi_program, _accounts, Some(amount), Some(decimals), None, authority_seeds.as_ref(), |_ctx, amount, decimals, _| token::transfer_checked(_ctx, amount.unwrap(), decimals.unwrap()))?;
            }
            _id if _id == token_2022::ID => {
                if fee.is_none() {
                    let _accounts = token_2022::TransferChecked {
                        from: ctx.accounts.from.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                        to: ctx.accounts.to.to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                    };
                    token_cpi(_cpi_program, _accounts, Some(amount), Some(decimals), None, authority_seeds.as_ref(), |_ctx, amount, decimals, _| token_2022::transfer_checked(_ctx, amount.unwrap(), decimals.unwrap()))?;
                }
                else {
                    let _accounts = token_2022_extensions::TransferCheckedWithFee {
                        token_program_id: ctx.accounts.token_program_id.to_account_info(),
                        source: ctx.accounts.from.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                        destination: ctx.accounts.to.to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                    };
                    token_cpi(_cpi_program, _accounts, Some(amount), Some(decimals), fee, authority_seeds.as_ref(), |_ctx, amount, decimals, fee| token_2022_extensions::transfer_checked_with_fee(_ctx, amount.unwrap(), decimals.unwrap(), fee.unwrap()))?;
                }
            },
            _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
        }

        Ok(())
    }

    pub fn ui_amount_to_amount(ctx: Context<UiAmountToAmountContext>, ui_amount: String) -> Result<u64> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();    
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }

        let _accounts = token_2022::UiAmountToAmount {
            account: ctx.accounts.account.to_account_info(),
        };

        let _cpi_ctx = CpiContext::new(_cpi_program, _accounts);

        let _result = token_2022::ui_amount_to_amount(_cpi_ctx, &ui_amount)?;
        Ok(_result)
    }

    // --- Token 2022 Extensions Instructions ---
    // CPI Guard
    pub fn cpi_guard_disable(
        ctx: Context<CpiGuardContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }
        
        let _accounts = token_2022_extensions::cpi_guard::CpiGuard {
            token_program_id: ctx.accounts.token_program_id.to_account_info(),
            account: ctx.accounts.account.to_account_info(),
            owner: ctx.accounts.owner.to_account_info(),
        };
        token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022_extensions::cpi_guard::cpi_guard_disable(_ctx))?;

        Ok(())
    }

    pub fn cpi_guard_enable(
        ctx: Context<CpiGuardContext>,
        authority_seeds: Option<Vec<Vec<u8>>>
    ) -> Result<()> {
        let _token_program_id: Pubkey = ctx.accounts.token_program_id.key();
        let _cpi_program = ctx.accounts.token_program_id.to_account_info();

        if _token_program_id != token_2022::ID {
            return Err(error!(ErrorCode::InvalidTokenProgram));
        }
        
        let _accounts = token_2022_extensions::cpi_guard::CpiGuard {
            token_program_id: ctx.accounts.token_program_id.to_account_info(),
            account: ctx.accounts.account.to_account_info(),
            owner: ctx.accounts.owner.to_account_info(),
        };
        token_cpi(_cpi_program, _accounts, None, None, None, authority_seeds.as_ref(), |_ctx, _, _, _| token_2022_extensions::cpi_guard::cpi_guard_enable(_ctx))?;

        Ok(())
    }
}