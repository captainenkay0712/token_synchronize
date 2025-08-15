use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction,
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
    require_eq!(mint.owner, token_program_id.key, ErrorCode::InvalidTokenProgram);

    let _signer_keys: Vec<Pubkey> = signer_accounts.iter().map(|a: &AccountInfo<'info>| *a.key).collect();
    let _signer_pubkeys: Vec<&Pubkey> = _signer_keys.iter().collect();

    let mut _ix: Instruction;
    let mut _infos: Vec<AccountInfo<'info>> = vec![
        source.clone(),
        mint.clone(),
        destination.clone(),
        authority.clone(),
    ];

    if signer_accounts.is_empty(){
        require!(authority.is_signer, ErrorCode::AuthorityNotSigner);
    } else {
        for _signer in signer_accounts.iter() {
            require!(_signer.is_signer, ErrorCode::MissingSigner);
        }

        _infos.extend(signer_accounts.iter().cloned());
    }

    let _data = mint.try_borrow_data()?;

    match token_program_id.key() {
        _id if _id == spl_token::ID => {
            let _mint = spl_token::state::Mint::unpack(_data.as_ref())?;

            _ix = spl_token::instruction::transfer_checked(
                &token_program_id.key(),
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
            let _mint = StateWithExtensions::<Mint>::unpack(_data.as_ref())?;

            if let Ok(_mint_transfer_fee) = _mint.get_extension::<TransferFeeConfig>() {
                let (_basis_points, _max_fee): (u16, u64) = if Clock::get()?.epoch >= _mint_transfer_fee.newer_transfer_fee.epoch.into() {
                    (
                        _mint_transfer_fee
                            .newer_transfer_fee
                            .transfer_fee_basis_points
                            .into(),
                        _mint_transfer_fee
                            .newer_transfer_fee
                            .maximum_fee
                            .into(),
                    )
                } else {
                    (
                        _mint_transfer_fee
                            .older_transfer_fee
                            .transfer_fee_basis_points
                            .into(),
                        _mint_transfer_fee
                            .older_transfer_fee
                            .maximum_fee
                            .into(),
                    )
                };

                let _raw_fee = (amount as u128) * (_basis_points as u128) / (10_000 as u128);
                let _expected_fee = min(_raw_fee as u64, _max_fee);

                _ix = spl_token_2022::extension::transfer_fee::instruction::transfer_checked_with_fee(
                    &token_program_id.key(),
                    &source.key(),
                    &mint.key(),
                    &destination.key(),
                    &authority.key(),
                    &_signer_pubkeys,
                    amount,
                    _mint.base.decimals,
                    _expected_fee,
                )?;
            } else {
                _ix = spl_token_2022::instruction::transfer_checked(
                    &token_program_id.key(),
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
                require!(!extra_accounts.is_empty(), ErrorCode::InvalidExtraAccounts);

                let _hook_program_ai = &extra_accounts[0];
                require_keys_eq!(*_hook_program_ai.key, _mint_transfer_hook.program_id.0, ErrorCode::InvalidExtraAccounts);
                require!(_hook_program_ai.executable, ErrorCode::InvalidExtraAccounts);

                let mut _metas = _ix.accounts.clone();
                for _account in extra_accounts.iter() {
                    match _account.is_writable {
                        true => _metas.push(AccountMeta::new(*_account.key, _account.is_signer)),
                        false => _metas.push(AccountMeta::new_readonly(*_account.key, _account.is_signer)),
                    }
                }

                _ix.accounts = _metas;
                _infos.extend(extra_accounts.iter().cloned());
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
