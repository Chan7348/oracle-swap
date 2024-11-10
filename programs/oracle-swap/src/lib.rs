use anchor_lang::prelude::*;
// use pyth_solana_receiver_sdk::price_update::{};
mod instructions;
mod errors;
use instructions::*;


declare_id!("C6qJS8UHcGznE7ekkvqA2ZRx7toAHURz1WCKp58GyhKN");

#[program]
pub mod oracle_swap {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        config::initialize_config(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::withdraw(ctx, amount)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        buy::buy(ctx, amount)
    }
}