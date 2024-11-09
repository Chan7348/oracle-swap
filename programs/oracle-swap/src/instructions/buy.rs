use anchor_lang::prelude::*;
use crate::config::*;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, address = config.sol_vault)]
    pub sol_vault: SystemAccount<'info>,

    /// CHECK: This is the Pyth price feed account for SOL/USD, It is read-only and trusted.
    pub price_feed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}