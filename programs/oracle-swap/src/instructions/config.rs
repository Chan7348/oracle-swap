use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub sol_vault: Pubkey, // 自动计算的 PDA 地址
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,           // 管理员账户，支付账户创建费用

    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32,             // Config account: discriminator + 2 Pubkeys
        seeds = [b"config"],             // PDA 种子
        bump                             // 自动计算 PDA 的 bump
    )]
    pub config: Account<'info, Config>,  // Config 账户

    /// CHECK: This is a PDA for SOL vault, manually initialized below
    #[account(
        seeds = [b"vault"],              // 使用 "vault" 作为种子，确保地址唯一
        bump                             // 自动计算 bump 值
    )]
    pub sol_vault: AccountInfo<'info>,   // 使用 AccountInfo 来表示 PDA 账户

    pub system_program: Program<'info, System>, // 系统程序
}

pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;

    // 设置 admin 为 Config 的管理员
    config.admin = ctx.accounts.admin.key();
    // 将 sol_vault PDA 地址存储在 Config 中
    config.sol_vault = ctx.accounts.sol_vault.key();

    // 获取 vault 的 PDA 地址和 bump
    let (vault_pda, bump) = Pubkey::find_program_address(&[b"vault"], ctx.program_id);
    let seeds = &[b"vault".as_ref(), &[bump]];

    // 手动调用系统程序创建 vault PDA 账户
    let lamports = Rent::get()?.minimum_balance(0);
    let ix = anchor_lang::solana_program::system_instruction::create_account(
        ctx.accounts.admin.key,
        &vault_pda,
        lamports,
        0, // 无需空间
        ctx.program_id,
    );

    // 执行账户创建指令
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.sol_vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[seeds],
    )?;

    msg!("Config initialized with admin: {:?} and sol_vault: {:?}", config.admin, config.sol_vault);
    Ok(())
}