
use anchor_lang::prelude::*;

// pub fn cpi_approve<'info, A>(    
//     program_id: AccountInfo<'info>,
//     accounts: A,
//     amount: u64,
//     authority_seeds: Option<&Vec<Vec<u8>>>,
//     approve_fn: impl Fn(CpiContext<'_, '_, '_, 'info, A>, u64) -> Result<()>,
// ) -> Result<()>
// where
//     A: ToAccountInfos<'info> + ToAccountMetas,
// {
//     if let Some(seeds) = &authority_seeds {
//         let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
//         let signer_seeds = &[&seeds_vec[..]];
//         let cpi_ctx = CpiContext::new(program_id, accounts).with_signer(signer_seeds);

//         approve_fn(cpi_ctx, amount)?;
//     } else {
//         let cpi_ctx = CpiContext::new(program_id, accounts);
//         approve_fn(cpi_ctx, amount)?;
//     }

//     Ok(())
// }

pub fn token_cpi<'info, A>(    
    program_id: AccountInfo<'info>,
    accounts: A,
    amount: u64,
    decimals: Option<u8>,
    authority_seeds: Option<&Vec<Vec<u8>>>,
    token_cpi_fn: impl Fn(CpiContext<'_, '_, '_, 'info, A>, u64, Option<u8>) -> Result<()>,
) -> Result<()>
where
    A: ToAccountInfos<'info> + ToAccountMetas,
{
    if let Some(seeds) = &authority_seeds {
        let seeds_vec: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
        let signer_seeds = &[&seeds_vec[..]];
        let cpi_ctx = CpiContext::new(program_id, accounts).with_signer(signer_seeds);

        token_cpi_fn(cpi_ctx, amount, decimals)?;
    } else {
        let cpi_ctx = CpiContext::new(program_id, accounts);
        token_cpi_fn(cpi_ctx, amount, decimals)?;
    }

    Ok(())
}