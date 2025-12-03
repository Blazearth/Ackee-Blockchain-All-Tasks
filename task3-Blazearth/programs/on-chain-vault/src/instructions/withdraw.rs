use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// The authority who owns the vault and can withdraw
    #[account(mut)]
    pub vault_authority: Signer<'info>,

    /// The vault PDA account
    #[account(
        mut,
        seeds = [b"vault", vault.vault_authority.as_ref()],
        bump,
        has_one = vault_authority
    )]
    pub vault: Account<'info, Vault>,

    /// The Solana system program
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_authority = &ctx.accounts.vault_authority;

    // ✅ 1. Ensure vault is not locked
    if vault.locked {
        return err!(VaultError::VaultLocked);
    }

    // ✅ 2. Ensure vault has enough lamports
    if **vault.to_account_info().lamports.borrow() < amount {
        return err!(VaultError::InsufficientBalance);
    }

    // ✅ 3. Transfer lamports from vault (PDA) to vault_authority
    // Since the vault PDA has data, we can't use system_program::transfer
    // Instead, manually transfer lamports by modifying account balances
    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **vault_authority.to_account_info().try_borrow_mut_lamports()? += amount;

    // ✅ 4. Emit withdraw event
    emit!(WithdrawEvent {
        amount,
        vault_authority: vault_authority.key(),
        vault: vault.key(),
    });

    Ok(())
}
