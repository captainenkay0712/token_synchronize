// use anchor_lang::prelude::*;
// use anchor_lang::solana_program::{
//     program::invoke_signed,
//     system_instruction,
// };
// // use anchor_spl::token::{self, Transfer as SplTransfer};
// use spl_token::instruction::{transfer as spl_transfer, approve as spl_approve, mint_to as spl_mint_to};

// use crate::context::*;
// use crate::error::*;

// #[cfg(feature = "mainnet")]
// declare_id!("G2LUVS7T6NeVEcRNgGFdpnwhUKJj5MfermEu99hf7rEL");

// #[program]
// mod token_synchronize {
//     use super::*;

//     pub fn approve(
//         ctx: Context<ApproveContext>,
//         amount: u64,
//         authority_seeds: Option<&[&[u8]]>,
//     ) -> Result<()> {
//         let token_program_id = ctx.accounts.token_program.key;
    
//         if token_program_id != spl_token::ID && token_program_id != spl_token_2022::ID {
//             return Err(error!(error::ApproveError::InvalidTokenProgram));
//         }
    
//         if amount == 0 {
//             return Err(error!(error::ApproveError::ZeroAmount));
//         }
    
//         if ctx.accounts.delegate.key == ctx.accounts.authority.key {
//             return Err(error!(error::ApproveError::DelegateEqualsAuthority));
//         }
    
//         let ix = spl_approve(
//             &token_program_id,
//             &ctx.accounts.source.key,
//             &ctx.accounts.delegate.key,
//             &ctx.accounts.authority.key,
//             &[],
//             amount,
//         )?;
    
//         let account_infos = [
//             ctx.accounts.source.clone(),
//             ctx.accounts.delegate.clone(),
//             ctx.accounts.authority.clone(),
//         ];
    
//         if let Some(seeds) = authority_seeds {
//             invoke_signed(&ix, &account_infos, &[seeds])?;
//         } else {
//             invoke(&ix, &account_infos)?;
//         }
    
//         Ok(())
//     }
    

//     // pub fn approve_bumps<'a>(
//     //     program_id: &AccountInfo<'a>,
//     //     source: &AccountInfo<'a>,
//     //     delegate: &AccountInfo<'a>,
//     //     authority: &AccountInfo<'a>,
//     //     amount: u64,
//     //     signer_seeds: &[&[&[u8]]],
//     // ) -> Result<(), ProgramError> {
//     //     let pid = program_id.key;

//     //     // Check amount > 0
//     //     if amount == 0 {
//     //         msg!("Approve amount must be greater than zero");
//     //         return Err(ProgramError::InvalidInstructionData);
//     //     }
//     //     // Approve checks
//     //     if authority.key == delegate.key {
//     //         msg!("Authority and delegate must be different");
//     //         return Err(ProgramError::InvalidArgument);
//     //     }
//     //     if source.key == delegate.key {
//     //         msg!("Source and delegate must be different");
//     //         return Err(ProgramError::InvalidArgument);
//     //     }
//     //     if source.key == authority.key {
//     //         msg!("Source and authority must be different");
//     //         return Err(ProgramError::InvalidArgument);
//     //     }

//     //     if pid == &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap()
//     //         || pid == &Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap()
//     //     {
//     //         msg!("Approving delegate with PDA signer...");
//     //         let ix = spl_approve(
//     //             source.key,
//     //             delegate.key,
//     //             authority.key,
//     //             &[],
//     //             amount,
//     //         )?;
//     //         invoke_signed(
//     //             &ix,
//     //             &[source.clone(), delegate.clone(), authority.clone()],
//     //             signer_seeds,
//     //         )?;
//     //     } else {
//     //         msg!("Unsupported program ID for approve_bumps");
//     //         return Err(ProgramError::IncorrectProgramId);
//     //     }

//     //     Ok(())
//     // }

//     // pub fn approve_checks<'a>(
//     //     program_id: &AccountInfo<'a>,
//     //     source: &AccountInfo<'a>,
//     //     delegate: &AccountInfo<'a>,
//     //     authority: &AccountInfo<'a>,
//     //     amount: u64,
//     // ) -> Result<(), ProgramError> {
//     //     if source.owner != program_id.key {
//     //         msg!("Source account is not owned by the expected token program");
//     //         return Err(ProgramError::IllegalOwner);
//     //     }
    
//     //     let pid = program_id.key;
    
//     //     if pid == &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap()
//     //         || pid == &Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap()
//     //     {
//     //         msg!("Approving delegate with checks...");
//     //         let ix = spl_token::instruction::approve(
//     //             pid,
//     //             source.key,
//     //             delegate.key,
//     //             authority.key,
//     //             &[],
//     //             amount,
//     //         )?;
//     //         invoke_signed(
//     //             &ix,
//     //             &[source.clone(), delegate.clone(), authority.clone()],
//     //             &[],
//     //         )?;
//     //     } else {
//     //         msg!("Unsupported program ID for approve_checks");
//     //         return Err(ProgramError::IncorrectProgramId);
//     //     }
    
//     //     Ok(())
//     // }
    

//     // pub fn transfer<'a>(
//     //     program_id: &AccountInfo<'a>,
//     //     owner: &AccountInfo<'a>,
//     //     source: &AccountInfo<'a>,
//     //     destination: &AccountInfo<'a>,
//     //     amount: u64,
//     //     signer_seeds: &[&[&[u8]]],
//     // ) -> Result<()> {
//     //     let pid = program_id.key();

//     //     if pid == &Pubkey::from_str("11111111111111111111111111111111").unwrap() {
//     //         let ix = system_instruction::transfer(
//     //             source.key,
//     //             destination.key,
//     //             amount,
//     //         );
//     //         invoke_signed(&ix, &[source.clone(), destination.clone()], signer_seeds)?;
//     //     } else if pid == &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() || pid == &Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap() {
//     //         let ix = token::Transfer {
//     //             from: source.clone(),
//     //             to: destination.clone(),
//     //             authority: owner.clone(),
//     //         };
//     //         let cpi_ctx = CpiContext::new_with_signer(
//     //             program_id.clone(),
//     //             ix,
//     //             signer_seeds,
//     //         );
//     //         token::transfer(cpi_ctx, amount)?;
//     //     } else {
//     //         return Err(error!(ErrorCode::UnsupportedProgram));
//     //     }

//     //     Ok(())
//     // }


//     // pub fn mint<'a>(
//     //     program_id: &AccountInfo<'a>,
//     //     mint_account: &AccountInfo<'a>,
//     //     destination: &AccountInfo<'a>,
//     //     authority: &AccountInfo<'a>,
//     //     amount: u64,
//     //     signer_seeds: &[&[&[u8]]],
//     // ) -> Result<(), ProgramError> {
//     //     let pid = program_id.key;
    
//     //     if pid == &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap()
//     //         || pid == &Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap()
//     //     {
//     //         msg!("Minting tokens...");
//     //         let ix = spl_mint_to(pid, mint_account.key, destination.key, authority.key, &[], amount)?;
//     //         invoke_signed(&ix, &[mint_account.clone(), destination.clone(), authority.clone()], signer_seeds)?;
//     //     } else {
//     //         msg!("Unsupported program ID for mint");
//     //         return Err(ProgramError::IncorrectProgramId);
//     //     }
    
//     //     Ok(())
//     // }
// }

// // #[error_code]
// // pub enum ErrorCode {
// //     #[msg("Unsupported token program ID")]
// //     UnsupportedProgram,
// // }
