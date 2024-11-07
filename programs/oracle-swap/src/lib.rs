use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("C6qJS8UHcGznE7ekkvqA2ZRx7toAHURz1WCKp58GyhKN");

#[program]
pub mod oracle_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // 管理员充值SOL
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
                from: ctx.accounts.admin.to_account_info(),
                to: ctx.accounts.sol_vault.to_account_info(),
            }),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub sol_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuySol<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub sol_vault: SystemAccount<'info>,
    /// CHECK: This is the Pyth price feed account for SOL/USD, It is read-only and trusted.
    pub price_feed: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode{
    #[msg("Insufficient payment provided.")]
    InsufficientPayment,
}


