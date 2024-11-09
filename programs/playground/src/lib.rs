use anchor_lang::prelude::*;

declare_id!("BG2eGBn2xY8AQN5EZe6NoqSSLwC3LKHZeiwFYujMnyHG");

#[program]
pub mod playground {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        return Ok(());
    }

    pub fn Greeting(ctx: Context<Greet>) -> Result<()> {
        msg!("Hello world Anchor program entrypoint!");

        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.counter += 1;

        msg!("Greeted {} time(s)!", greeting_account.counter);

        return Ok(());
    }


}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Greet<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct GreetingAccount {
    pub counter: u32, // 记录问候次数
}