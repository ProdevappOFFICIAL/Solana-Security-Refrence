use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod admin_vault {
    use super::*;

    // ========================================
    // VULNERABLE IMPLEMENTATION
    // ========================================
    // This section contains INTENTIONALLY VULNERABLE code for educational purposes.
    // DO NOT use this code in production environments.

    /// VULNERABLE: Initialize a new admin vault without proper authority validation
    /// 
    /// Security Issue: This function sets up an admin vault but doesn't properly
    /// validate that the admin account has the authority to manage this vault.
    /// The vulnerability lies in the distinction between being a signer and having authority.
    pub fn vulnerable_initialize(ctx: Context<VulnerableInitialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: We're setting the admin without validating that this admin
        // is actually authorized to manage this specific vault. Any signer can
        // become an admin by simply signing the transaction.
        vault.admin = ctx.accounts.admin.key();
        vault.balance = initial_balance;
        vault.owner = ctx.accounts.owner.key();
        
        msg!("Vault initialized with admin: {}, owner: {}, balance: {}", 
             vault.admin, vault.owner, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Admin withdrawal that only checks signature, not authority
    /// 
    /// Security Issue: This is the critical vulnerability. The function checks that
    /// someone signed the transaction (Signer<'info>), but it doesn't verify that
    /// the signer is actually THE admin of THIS specific vault. Any signer can
    /// potentially withdraw funds if they can pass themselves as the admin.
    pub fn vulnerable_admin_withdraw(ctx: Context<VulnerableAdminWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: We check that someone signed (admin is Signer<'info>)
        // but we DON'T verify that ctx.accounts.admin.key() == vault.admin
        // This means ANY signer can withdraw as long as they're passed as "admin"!
        require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
        
        vault.balance = vault.balance.checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;
        
        msg!("Admin withdrew {} from vault. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Emergency drain function with signature-only validation
    /// 
    /// Security Issue: This function is supposed to be an emergency admin function
    /// but it only validates that someone signed the transaction, not that they
    /// are the actual admin of the vault. This is a common pattern where developers
    /// confuse "being a signer" with "having authority".
    pub fn vulnerable_emergency_drain(ctx: Context<VulnerableEmergencyDrain>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: We check that admin is a signer, but we don't validate
        // that this signer is actually THE admin for THIS vault.
        // Any signer can drain any vault if they can pass the vault account!
        let drained_amount = vault.balance;
        vault.balance = 0;
        
        msg!("Emergency drain executed by admin. Drained amount: {}", drained_amount);
        Ok(())
    }

    /// VULNERABLE: Change admin function without proper validation
    /// 
    /// Security Issue: This function allows changing the admin but only validates
    /// that someone signed the transaction. It doesn't verify that the current
    /// signer is actually the current admin of the vault.
    pub fn vulnerable_change_admin(ctx: Context<VulnerableChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: We don't verify that current_admin.key() == vault.admin
        // Any signer can change the admin of any vault!
        let old_admin = vault.admin;
        vault.admin = new_admin;
        
        msg!("Admin changed from {} to {}", old_admin, new_admin);
        Ok(())
    }

    // ========================================
    // SECURE IMPLEMENTATION
    // ========================================
    // This section contains the SECURE versions of the same functions
    // with proper authority validation using Anchor constraints.

    /// SECURE: Initialize a new admin vault with proper validation
    /// 
    /// Security Fix: Uses Anchor's `init` constraint for proper initialization
    /// and establishes the relationship between vault and admin that will be
    /// validated in subsequent operations.
    pub fn secure_initialize(ctx: Context<SecureInitialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: The admin and owner are set and will be validated by
        // Anchor constraints in future operations using `has_one`
        vault.admin = ctx.accounts.admin.key();
        vault.balance = initial_balance;
        vault.owner = ctx.accounts.owner.key();
        
        msg!("Secure vault initialized with admin: {}, owner: {}, balance: {}", 
             vault.admin, vault.owner, vault.balance);
        Ok(())
    }

    /// SECURE: Admin withdrawal with proper authority validation
    /// 
    /// Security Fix: Uses `has_one = admin` constraint to ensure that the
    /// signer is actually THE admin of THIS specific vault before allowing withdrawal.
    pub fn secure_admin_withdraw(ctx: Context<SecureAdminWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: Anchor's `has_one = admin` constraint has already validated
        // that vault.admin == ctx.accounts.admin.key() before this code runs
        require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
        
        vault.balance = vault.balance.checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;
        
        msg!("Secure admin withdrew {} from vault. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// SECURE: Emergency drain with proper authority validation
    /// 
    /// Security Fix: Uses `has_one = admin` constraint to ensure only the
    /// actual admin of the vault can execute emergency operations.
    pub fn secure_emergency_drain(ctx: Context<SecureEmergencyDrain>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: Anchor's `has_one = admin` constraint ensures that
        // vault.admin == ctx.accounts.admin.key() before this code runs
        let drained_amount = vault.balance;
        vault.balance = 0;
        
        msg!("Secure emergency drain executed by verified admin. Drained amount: {}", drained_amount);
        Ok(())
    }

    /// SECURE: Change admin with proper authority validation
    /// 
    /// Security Fix: Uses `has_one = admin` constraint to ensure only the
    /// current admin can change the admin of the vault.
    pub fn secure_change_admin(ctx: Context<SecureChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: Anchor's `has_one = admin` constraint has validated that
        // the current signer is actually the current admin before allowing the change
        let old_admin = vault.admin;
        vault.admin = new_admin;
        
        msg!("Admin securely changed from {} to {}", old_admin, new_admin);
        Ok(())
    }
}

// ========================================
// VULNERABLE ACCOUNT CONTEXTS
// ========================================
// These contexts lack proper authority validation constraints

#[derive(Accounts)]
pub struct VulnerableInitialize<'info> {
    /// VULNERABILITY: Using `zero` constraint instead of `init` for demonstration
    #[account(zero)]
    pub vault: Account<'info, AdminVault>,
    
    /// VULNERABILITY: Admin is a signer but we don't validate their authority
    /// to create vaults. Any signer can become an admin.
    pub admin: Signer<'info>,
    
    /// VULNERABILITY: Owner doesn't need to be a signer, which could lead to
    /// unauthorized vault creation for other users
    /// CHECK: This is intentionally unsafe for demonstration
    pub owner: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VulnerableAdminWithdraw<'info> {
    /// VULNERABILITY: No `has_one` constraint to validate admin authority
    #[account(mut)]
    pub vault: Account<'info, AdminVault>,
    
    /// VULNERABILITY: We require a signer but don't validate it's THE admin
    /// of THIS vault. Any signer can potentially act as admin.
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct VulnerableEmergencyDrain<'info> {
    /// VULNERABILITY: No `has_one` constraint to validate admin authority
    #[account(mut)]
    pub vault: Account<'info, AdminVault>,
    
    /// VULNERABILITY: Any signer can execute emergency drain on any vault
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct VulnerableChangeAdmin<'info> {
    /// VULNERABILITY: No `has_one` constraint to validate current admin authority
    #[account(mut)]
    pub vault: Account<'info, AdminVault>,
    
    /// VULNERABILITY: Any signer can change the admin of any vault
    pub current_admin: Signer<'info>,
}

// ========================================
// SECURE ACCOUNT CONTEXTS
// ========================================
// These contexts use proper Anchor constraints for authority validation

#[derive(Accounts)]
pub struct SecureInitialize<'info> {
    /// SECURITY: `init` constraint ensures proper initialization
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 8, // discriminator + admin + owner + balance
    )]
    pub vault: Account<'info, AdminVault>,
    
    /// SECURITY: Admin must be a signer to prove they authorize this vault creation
    pub admin: Signer<'info>,
    
    /// SECURITY: Owner should also be a signer to authorize vault creation
    pub owner: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SecureAdminWithdraw<'info> {
    /// SECURITY: `has_one = admin` constraint validates that vault.admin
    /// matches the admin account passed in the context
    #[account(
        mut,
        has_one = admin
    )]
    pub vault: Account<'info, AdminVault>,
    
    /// SECURITY: Must be a signer AND must match vault.admin
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureEmergencyDrain<'info> {
    /// SECURITY: `has_one = admin` constraint validates admin authority
    #[account(
        mut,
        has_one = admin
    )]
    pub vault: Account<'info, AdminVault>,
    
    /// SECURITY: Must be a signer AND must match vault.admin
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureChangeAdmin<'info> {
    /// SECURITY: `has_one = admin` constraint validates current admin authority
    #[account(
        mut,
        has_one = admin @ ErrorCode::UnauthorizedAdmin
    )]
    pub vault: Account<'info, AdminVault>,
    
    /// SECURITY: Must be a signer AND must match vault.admin
    pub admin: Signer<'info>,
}

// ========================================
// ACCOUNT STRUCTURES
// ========================================

#[account]
pub struct AdminVault {
    /// The admin who can perform administrative operations (32 bytes)
    pub admin: Pubkey,
    /// The owner of the vault (32 bytes)
    pub owner: Pubkey,
    /// The current balance in lamports (8 bytes)
    pub balance: u64,
}

// ========================================
// ERROR DEFINITIONS
// ========================================

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
    #[msg("Unauthorized admin - signer is not the vault admin")]
    UnauthorizedAdmin,
}