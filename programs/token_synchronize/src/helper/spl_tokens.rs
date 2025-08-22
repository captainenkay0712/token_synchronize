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
    let current_epoch: u64 = Clock::get()?.epoch.into();
    let (basis_points, max_fee): (u16, u64) = if current_epoch >= mint_transfer_fee.newer_transfer_fee.epoch.into() {
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

    let raw_fee = (amount as u128) * (basis_points as u128) / (10_000 as u128);
    let expected_fee = min(raw_fee as u64, max_fee);
    Ok(expected_fee)
}

fn append_hook_accounts<'info>(ix: &mut Instruction, infos: &mut Vec<AccountInfo<'info>>, extra_accounts: &[AccountInfo<'info>], hook_program_id: Pubkey) -> Result<()> {
    require!(!extra_accounts.is_empty(), ErrorCode::InvalidExtraAccounts);

    let hook_program_ai = &extra_accounts[0];
    require_keys_eq!(*hook_program_ai.key, hook_program_id, ErrorCode::InvalidExtraAccounts);
    require!(hook_program_ai.executable, ErrorCode::InvalidExtraAccounts);

    let mut metas = ix.accounts.clone();
    for account in extra_accounts.iter() {
        if account.is_writable {
            metas.push(AccountMeta::new(*account.key, account.is_signer));
        } else {
            metas.push(AccountMeta::new_readonly(*account.key, account.is_signer));
        }
    }

    ix.accounts = metas;
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
    let pid = token_program_id.key();

    require_eq!(*mint.owner, pid, ErrorCode::InvalidTokenProgram);
    require_keys_eq!(*source.owner, pid, ErrorCode::InvalidTokenProgram);
    require_keys_eq!(*destination.owner, pid, ErrorCode::InvalidTokenProgram);

    let mut ix: Instruction;
    let mut infos: Vec<AccountInfo<'info>> = vec![
        source.clone(),
        mint.clone(),
        destination.clone(),
        authority.clone(),
    ];

    let signer_keys: Vec<Pubkey> = signer_accounts.iter().map(|a: &AccountInfo<'info>| *a.key).collect();
    let signer_pubkeys: Vec<&Pubkey> = signer_keys.iter().collect();

    if signer_accounts.is_empty() && authority_seeds.is_empty() {
        require!(authority.is_signer, ErrorCode::AuthorityNotSigner);
    } else {
        for signer in signer_accounts.iter() {
            require!(signer.is_signer, ErrorCode::MissingSigner);
        }
        infos.extend(signer_accounts.iter().cloned());
    }

    let data = mint.try_borrow_data()?;

    match pid {
        id if id == spl_token::ID => {
            let mint_token = spl_token::state::Mint::unpack(data.as_ref())?;

            let source_account = spl_token::state::Account::unpack(&source.data.borrow())?;
            let destination_account = spl_token::state::Account::unpack(&destination.data.borrow())?;

            require_keys_eq!(source_account.mint, mint.key(), ErrorCode::InvalidTokenMint);
            require_keys_eq!(destination_account.mint, mint.key(), ErrorCode::InvalidTokenMint);

            require!(source_account.amount >= amount, ErrorCode::InsufficientFunds);

            ix = spl_token::instruction::transfer_checked(
                &pid,
                &source.key(),
                &mint.key(),
                &destination.key(),
                &authority.key(),
                &signer_pubkeys,
                amount,
                mint_token.decimals,
            )?;
        }
        id if id == spl_token_2022::ID => {
            let mint_token: StateWithExtensions<'_, Mint> = StateWithExtensions::<Mint>::unpack(data.as_ref())?;

            let source_data = source.try_borrow_data()?;
            let destination_data = destination.try_borrow_data()?;

            let source_account: StateWithExtensions<'_, spl_token_2022::state::Account> =
                StateWithExtensions::<spl_token_2022::state::Account>::unpack(&source_data)?;

            let destination_account: StateWithExtensions<'_, spl_token_2022::state::Account> =
                StateWithExtensions::<spl_token_2022::state::Account>::unpack(&destination_data)?;

            require_keys_eq!(source_account.base.mint, mint.key(), ErrorCode::InvalidTokenMint);
            require_keys_eq!(destination_account.base.mint, mint.key(), ErrorCode::InvalidTokenMint);

            require!(source_account.base.amount >= amount, ErrorCode::InsufficientFunds);

            if let Ok(mint_transfer_fee) = mint_token.get_extension::<TransferFeeConfig>() {
                ix = spl_token_2022::extension::transfer_fee::instruction::transfer_checked_with_fee(
                    &pid,
                    &source.key(),
                    &mint.key(),
                    &destination.key(),
                    &authority.key(),
                    &signer_pubkeys,
                    amount,
                    mint_token.base.decimals,
                    compute_expected_fee(mint_transfer_fee, amount)?,
                )?;
            } else {
                ix = spl_token_2022::instruction::transfer_checked(
                    &pid,
                    &source.key(),
                    &mint.key(),
                    &destination.key(),
                    &authority.key(),
                    &signer_pubkeys,
                    amount,
                    mint_token.base.decimals,
                )?;
            }

            if let Ok(mint_transfer_hook) = mint_token.get_extension::<TransferHook>() {
                let hook_program_id = mint_transfer_hook.program_id.0;
                drop(data);

                append_hook_accounts(&mut ix, &mut infos, &extra_accounts, hook_program_id)?;
            }
        }
        _ => return Err(error!(ErrorCode::InvalidTokenProgram)),
    }

    let result = if authority_seeds.is_empty() {
        invoke(&ix, &infos)
    } else {
        invoke_signed(&ix, &infos, &authority_seeds)
    };

    result.map_err(Into::into)
}