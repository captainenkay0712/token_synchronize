use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;
use anchor_spl::token_2022::spl_token_2022;
use std::result::Result;

pub fn parse_authority_type(index: u8) -> Result<spl_token::instruction::AuthorityType, ProgramError> {
    match index {
        0 => Ok(spl_token::instruction::AuthorityType::MintTokens),
        1 => Ok(spl_token::instruction::AuthorityType::FreezeAccount),
        2 => Ok(spl_token::instruction::AuthorityType::AccountOwner),
        3 => Ok(spl_token::instruction::AuthorityType::CloseAccount),
        
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn parse_authority_type_2022(index: u8) -> Result<spl_token_2022::instruction::AuthorityType, ProgramError> {
    match index {
        0 => Ok(spl_token_2022::instruction::AuthorityType::MintTokens),
        1 => Ok(spl_token_2022::instruction::AuthorityType::FreezeAccount),
        2 => Ok(spl_token_2022::instruction::AuthorityType::AccountOwner),
        3 => Ok(spl_token_2022::instruction::AuthorityType::CloseAccount),
        4 => Ok(spl_token_2022::instruction::AuthorityType::TransferFeeConfig),
        5 => Ok(spl_token_2022::instruction::AuthorityType::WithheldWithdraw),
        6 => Ok(spl_token_2022::instruction::AuthorityType::CloseMint),
        7 => Ok(spl_token_2022::instruction::AuthorityType::InterestRate),
        8 => Ok(spl_token_2022::instruction::AuthorityType::PermanentDelegate),
        9 => Ok(spl_token_2022::instruction::AuthorityType::ConfidentialTransferMint),
        10 => Ok(spl_token_2022::instruction::AuthorityType::TransferHookProgramId),
        11 => Ok(spl_token_2022::instruction::AuthorityType::ConfidentialTransferFeeConfig),
        12 => Ok(spl_token_2022::instruction::AuthorityType::MetadataPointer),
        13 => Ok(spl_token_2022::instruction::AuthorityType::GroupPointer),
        14 => Ok(spl_token_2022::instruction::AuthorityType::GroupMemberPointer),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
