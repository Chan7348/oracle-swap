use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::{errors, config::*};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,

    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(mut, address = config.sol_vault)]
    pub sol_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    transfer(
        CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
            from: ctx.accounts.admin.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        }),
        amount,
    ).map_err(|_| errors::ErrorCode::TransferFailed)?;
    Ok(())
}