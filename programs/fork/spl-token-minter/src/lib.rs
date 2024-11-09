pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("D8CuAy6k5kFtTVXhkT2kZadUz6b49Hd9voeRiFcPzbM1");

#[program]
pub mod fork_spl_token_minter {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        token_name: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, token_name, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint::mint_token(ctx, amount)
    }
}
