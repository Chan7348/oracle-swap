use anchor_lang::prelude::*;
use crate::config::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,

    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(mut, address = config.sol_vault)]
    pub sol_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    Ok(())
}