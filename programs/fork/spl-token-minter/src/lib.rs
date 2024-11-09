pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("D8CuAy6k5kFtTVXhkT2kZadUz6b49Hd9voeRiFcPzbM1");

#[program]
pub mod fork_spl_token_minter {
    use super::*;

}
