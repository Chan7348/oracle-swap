use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub sol_vault: Pubkey,
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = * + 32 + 32, //
    )]
}


pub fn configInit() -> Result<()> {

    Ok(())
}