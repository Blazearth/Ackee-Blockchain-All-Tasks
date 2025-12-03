//-------------------------------------------------------------------------------
///
/// IMPLEMENTATION: Deposit Instruction
///
/// This instruction allows users to deposit lamports into the vault.
///
/// Key Concepts Demonstrated:
/// - Account constraints and validation
/// - Vault lock check
/// - Cross-Program Invocation (CPI) to transfer lamports
/// - Event emission after successful deposit
///
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke,
    system_instruction::transfer,
};
use crate::state::Vault;
use crate::events::DepositEvent;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// The user depositing lamports
    #[account(mut)]
    pub user: Signer<'info>,

    /// The vault account to deposit into
    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    /// The Solana system program
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let user = &ctx.accounts.user;

    // ✅ 1. Check vault is not locked
    if vault.locked {
        return err!(VaultError::VaultLocked);
    }

    // ✅ 2. Check user has enough lamports
    if **user.lamports.borrow() < amount {
        return err!(VaultError::InsufficientBalance);
    }

    // ✅ 3. Transfer lamports from user to vault PDA
    let transfer_ix = transfer(
        &user.key(),
        &vault.key(),
        amount,
    );

    invoke(
        &transfer_ix,
        &[
            user.to_account_info(),
            vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // ✅ 4. Emit deposit event
    emit!(DepositEvent {
        amount,
        user: user.key(),
        vault: vault.key(),
    });

    Ok(())
}
