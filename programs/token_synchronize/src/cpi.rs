
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;
use anchor_spl::token_2022::spl_token_2022;

pub fn token_cpi<'info, A>(    
    program_id: AccountInfo<'info>,
    accounts: A,
    amount: Option<u64>,
    decimals: Option<u8>,
    fee: Option<u64>,
    authority_seeds: Option<&Vec<Vec<u8>>>,
    token_cpi_fn: impl Fn(CpiContext<'_, '_, '_, 'info, A>, Option<u64>, Option<u8>, Option<u64>) -> Result<()>,
) -> Result<()>
where
    A: ToAccountInfos<'info> + ToAccountMetas,
{
    if let Some(seeds) = &authority_seeds {
        let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
        let signer_seeds = &[&seeds_vec[..]];
        let cpi_ctx = CpiContext::new(program_id, accounts).with_signer(signer_seeds);

        token_cpi_fn(cpi_ctx, amount, decimals, fee)?;
    } else {
        let cpi_ctx = CpiContext::new(program_id, accounts);
        token_cpi_fn(cpi_ctx, amount, decimals, fee)?;
    }

    Ok(())
}

pub fn initialize_mint_cpi<'info, A>(    
    program_id: AccountInfo<'info>,
    accounts: A,
    decimals: u8,
    authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    authority_seeds: Option<&Vec<Vec<u8>>>,
    initialize_mint_fn: impl Fn(CpiContext<'_, '_, '_, 'info, A>, u8, &Pubkey, Option<&Pubkey>) -> Result<()>,
) -> Result<()>
where
    A: ToAccountInfos<'info> + ToAccountMetas,
{
    if let Some(seeds) = &authority_seeds {
        let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
        let signer_seeds = &[&seeds_vec[..]];
        let cpi_ctx = CpiContext::new(program_id, accounts).with_signer(signer_seeds);

        initialize_mint_fn(cpi_ctx, decimals, authority, freeze_authority)?;
    } else {
        let cpi_ctx = CpiContext::new(program_id, accounts);
        initialize_mint_fn(cpi_ctx, decimals, authority, freeze_authority)?;
    }

    Ok(())
}

pub fn initialize_mint_close_authority_cpi<'info, A>(    
    program_id: AccountInfo<'info>,
    accounts: A,
    close_authority: Option<&Pubkey>,
    authority_seeds: Option<&Vec<Vec<u8>>>,
    initialize_mint_fn: impl Fn(CpiContext<'_, '_, '_, 'info, A>, Option<&Pubkey>) -> Result<()>,
) -> Result<()>
where
    A: ToAccountInfos<'info> + ToAccountMetas,
{
    if let Some(seeds) = &authority_seeds {
        let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
        let signer_seeds = &[&seeds_vec[..]];
        let cpi_ctx = CpiContext::new(program_id, accounts).with_signer(signer_seeds);

        initialize_mint_fn(cpi_ctx, close_authority)?;
    } else {
        let cpi_ctx = CpiContext::new(program_id, accounts);
        initialize_mint_fn(cpi_ctx, close_authority)?;
    }

    Ok(())
}

pub fn set_authority_cpi<'info, A>(    
    program_id: AccountInfo<'info>,
    accounts: A,
    authority_type: spl_token::instruction::AuthorityType,
    new_authority: Option<Pubkey>,
    authority_seeds: Option<&Vec<Vec<u8>>>,
    set_authority_fn: impl Fn(CpiContext<'_, '_, '_, 'info, A>, spl_token::instruction::AuthorityType, Option<Pubkey>) -> Result<()>,
) -> Result<()>
where
    A: ToAccountInfos<'info> + ToAccountMetas,
{
    if let Some(seeds) = &authority_seeds {
        let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
        let signer_seeds = &[&seeds_vec[..]];
        let cpi_ctx = CpiContext::new(program_id, accounts).with_signer(signer_seeds);

        set_authority_fn(cpi_ctx, authority_type, new_authority)?;
    } else {
        let cpi_ctx = CpiContext::new(program_id, accounts);
        set_authority_fn(cpi_ctx, authority_type, new_authority)?;
    }

    Ok(())
}

pub fn set_authority_2022_cpi<'info, A>(    
    program_id: AccountInfo<'info>,
    accounts: A,
    authority_type: spl_token_2022::instruction::AuthorityType,
    new_authority: Option<Pubkey>,
    authority_seeds: Option<&Vec<Vec<u8>>>,
    set_authority_fn: impl Fn(CpiContext<'_, '_, '_, 'info, A>, spl_token_2022::instruction::AuthorityType, Option<Pubkey>) -> Result<()>,
) -> Result<()>
where
    A: ToAccountInfos<'info> + ToAccountMetas,
{
    if let Some(seeds) = &authority_seeds {
        let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
        let signer_seeds = &[&seeds_vec[..]];
        let cpi_ctx = CpiContext::new(program_id, accounts).with_signer(signer_seeds);

        set_authority_fn(cpi_ctx, authority_type, new_authority)?;
    } else {
        let cpi_ctx = CpiContext::new(program_id, accounts);
        set_authority_fn(cpi_ctx, authority_type, new_authority)?;
    }

    Ok(())
}