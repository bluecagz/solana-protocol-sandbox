#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("9UzoNLvQcx38KNYTZDH48ZfXcGEoWeteWQYG6idWfcos");

#[program]
pub mod mysandboxdapp {
    use super::*;

  pub fn close(_ctx: Context<CloseMysandboxdapp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.mysandboxdapp.count = ctx.accounts.mysandboxdapp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.mysandboxdapp.count = ctx.accounts.mysandboxdapp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeMysandboxdapp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.mysandboxdapp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeMysandboxdapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Mysandboxdapp::INIT_SPACE,
  payer = payer
  )]
  pub mysandboxdapp: Account<'info, Mysandboxdapp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseMysandboxdapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub mysandboxdapp: Account<'info, Mysandboxdapp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub mysandboxdapp: Account<'info, Mysandboxdapp>,
}

#[account]
#[derive(InitSpace)]
pub struct Mysandboxdapp {
  count: u8,
}
