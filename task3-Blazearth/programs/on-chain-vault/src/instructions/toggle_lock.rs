use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    /// The vault authority who is allowed to toggle the lock
    #[account(mut)]
    pub vault_authority: Signer<'info>,

    /// The vault whose locked state will be toggled
    #[account(
        mut,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // ✅ Only vault authority can toggle — enforced by PDA seeds check above

    // 🔄 Toggle locked state
    vault.locked = !vault.locked;

    // 📡 Emit event after successful state change
    emit!(ToggleLockEvent {
        vault: vault.key(),
        vault_authority: ctx.accounts.vault_authority.key(),
        locked: vault.locked,
    });

    Ok(())
}
