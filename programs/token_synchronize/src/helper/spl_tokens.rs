use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{
        AccountMeta,
        Instruction,
    },
    program::invoke,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token;
use spl_token_2022::{
    state::{
        Mint
    },
    extension::{
        StateWithExtensions,
        BaseStateWithExtensions,
        transfer_fee::TransferFeeConfig,
        transfer_hook::TransferHook,
    },

};
use std::cmp::min;

use crate::error::ErrorCode;

fn compute_expected_fee(mint_transfer_fee: &TransferFeeConfig, amount: u64) -> Result<u64> {
    let _current_epoch: u64 = Clock::get()?.epoch.into();
    let (_basis_points, _max_fee): (u16, u64) = if _current_epoch >= mint_transfer_fee.newer_transfer_fee.epoch.into() {
        (
            mint_transfer_fee
                .newer_transfer_fee
                .transfer_fee_basis_points
                .into(),
                mint_transfer_fee
                .newer_transfer_fee
                .maximum_fee
                .into(),
        )
    } else {
        (
            mint_transfer_fee
                .older_transfer_fee
                .transfer_fee_basis_points
                .into(),
                mint_transfer_fee
                .older_transfer_fee
                .maximum_fee
                .into(),
        )
    };

    let _raw_fee = (amount as u128) * (_basis_points as u128) / (10_000 as u128);
    let _expected_fee = min(_raw_fee as u64, _max_fee);
    Ok(_expected_fee)
}

fn append_hook_accounts<'info>(ix: &mut Instruction, infos: &mut Vec<AccountInfo<'info>>, extra_accounts: &[AccountInfo<'info>], hook_program_id: Pubkey) -> Result<()> {
    require!(!extra_accounts.is_empty(), ErrorCode::InvalidExtraAccounts);

    let _hook_program_ai = &extra_accounts[0];
    require_keys_eq!(*_hook_program_ai.key, hook_program_id, ErrorCode::InvalidExtraAccounts);
    require!(_hook_program_ai.executable, ErrorCode::InvalidExtraAccounts);

    let mut _metas = ix.accounts.clone();
    for _account in extra_accounts.iter() {
        match _account.is_writable {
            true => _metas.push(AccountMeta::new(*_account.key, _account.is_signer)),
            false => _metas.push(AccountMeta::new_readonly(*_account.key, _account.is_signer)),
        }
    }

    ix.accounts = _metas;
    infos.extend_from_slice(extra_accounts);

    Ok(())
}

pub fn transfer<'info>(
    token_program_id: &AccountInfo<'info>,
    source: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    destination: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    signer_accounts: &[AccountInfo<'info>],
    amount: u64,
    authority_seeds: &[&[&[u8]]],
    extra_accounts: &[AccountInfo<'info>]
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let _pid = token_program_id.key();

    require_eq!(*mint.owner, _pid, ErrorCode::InvalidTokenProgram);
    require_keys_eq!(*source.owner, _pid, ErrorCode::InvalidTokenProgram);
    require_keys_eq!(*destination.owner, _pid, ErrorCode::InvalidTokenProgram);

    let mut _ix: Instruction;
    let mut _infos: Vec<AccountInfo<'info>> = vec![
        source.clone(),
        mint.clone(),
        destination.clone(),
        authority.clone(),
    ];

    let _signer_keys: Vec<Pubkey> = signer_accounts.iter().map(|a: &AccountInfo<'info>| *a.key).collect();
    let _signer_pubkeys: Vec<&Pubkey> = _signer_keys.iter().collect();

    if signer_accounts.is_empty(){
        require!(authority.is_signer, ErrorCode::AuthorityNotSigner);
    } else {
        for _signer in signer_accounts.iter() {
            require!(_signer.is_signer, ErrorCode::MissingSigner);
        }
        _infos.extend(signer_accounts.iter().cloned());
    }

    let _data = mint.try_borrow_data()?;

    match _pid {
        _id if _id == spl_token::ID => {
            let _mint = spl_token::state::Mint::unpack(_data.as_ref())?;

            let source_account = spl_token::state::Account::unpack(&source.data.borrow())?;
            let destination_account = spl_token::state::Account::unpack(&destination.data.borrow())?;

            require_keys_eq!(source_account.mint, mint.key(), ErrorCode::InvalidTokenMint);
            require_keys_eq!(destination_account.mint, mint.key(), ErrorCode::InvalidTokenMint);

            require!(source_account.amount >= amount, ErrorCode::InsufficientFunds);

            _ix = spl_token::instruction::transfer_checked(
                &_pid,
                &source.key(),
                &mint.key(),
                &destination.key(),
                &authority.key(),
                &_signer_pubkeys,
                amount,
                _mint.decimals,
            )?;
        }
        _id if _id == spl_token_2022::ID => {
            let _mint: StateWithExtensions<'_, Mint> = StateWithExtensions::<Mint>::unpack(_data.as_ref())?;

            let source_data = source.try_borrow_data()?;
            let destination_data = destination.try_borrow_data()?;

            let source_account: StateWithExtensions<'_, spl_token_2022::state::Account> =
                StateWithExtensions::<spl_token_2022::state::Account>::unpack(&source_data)?;

            let destination_account: StateWithExtensions<'_, spl_token_2022::state::Account> =
                StateWithExtensions::<spl_token_2022::state::Account>::unpack(&destination_data)?;

            require_keys_eq!(source_account.base.mint, mint.key(), ErrorCode::InvalidTokenMint);
            require_keys_eq!(destination_account.base.mint, mint.key(), ErrorCode::InvalidTokenMint);

            require!(source_account.base.amount >= amount, ErrorCode::InsufficientFunds);

            if let Ok(_mint_transfer_fee) = _mint.get_extension::<TransferFeeConfig>() {
                _ix = spl_token_2022::extension::transfer_fee::instruction::transfer_checked_with_fee(
                    &_pid,
                    &source.key(),
                    &mint.key(),
                    &destination.key(),
                    &authority.key(),
                    &_signer_pubkeys,
                    amount,
                    _mint.base.decimals,
                    compute_expected_fee(_mint_transfer_fee, amount)?,
                )?;
            } else {
                _ix = spl_token_2022::instruction::transfer_checked(
                    &_pid,
                    &source.key(),
                    &mint.key(),
                    &destination.key(),
                    &authority.key(),
                    &_signer_pubkeys,
                    amount,
                    _mint.base.decimals,
                )?;
            }

            if let Ok(_mint_transfer_hook) = _mint.get_extension::<TransferHook>() {
                let _hook_program_id = _mint_transfer_hook.program_id.0;
                drop(_data);

                append_hook_accounts(&mut _ix, &mut _infos, &extra_accounts, _hook_program_id)?;
            }
        }
        _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
    }

    match authority_seeds.is_empty() {
        true => invoke(&_ix, &_infos)?,
        false => invoke_signed(&_ix, &_infos, &authority_seeds)?,
    }

    Ok(())
}
