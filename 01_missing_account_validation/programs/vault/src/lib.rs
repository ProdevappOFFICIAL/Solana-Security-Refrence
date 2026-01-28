use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    // ========================================
    // VULNERABLE IMPLEMENTATION
    // ========================================
    // This section contains INTENTIONALLY VULNERABLE code for educational purposes.
    // DO NOT use this code in production environments.

    /// VULNERABLE: Initialize a new vault without proper validation
    /// 
    /// Security Issue: This function doesn't validate the relationship between
    /// the vault account and the owner. An attacker could potentially create
    /// a vault account with any owner they choose, leading to unauthorized access.
    pub fn vulnerable_initialize(ctx: Context<VulnerableInitialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: We're setting the owner without validating that the signer
        // is actually authorized to create a vault for this owner.
        // An attacker could pass any owner pubkey here.
        vault.owner = ctx.accounts.owner.key();
        vault.balance = initial_balance;
        
        msg!("Vault initialized with owner: {} and balance: {}", vault.owner, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Deposit funds without proper account validation
    /// 
    /// Security Issue: This function doesn't verify that the vault actually
    /// belongs to the claimed owner. An attacker could deposit to any vault
    /// and then potentially withdraw from it if they can manipulate the owner field.
    pub fn vulnerable_deposit(ctx: Context<VulnerableDeposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: No validation that the vault.owner matches the signer
        // An attacker could potentially deposit to any vault account
        vault.balance = vault.balance.checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Deposited {} to vault. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Withdraw funds without proper owner validation
    /// 
    /// Security Issue: This is the critical vulnerability. The function only
    /// checks that someone signed the transaction, but doesn't verify that
    /// the signer is actually the owner of the vault. An attacker could
    /// withdraw from any vault if they can pass the vault account.
    pub fn vulnerable_withdraw(ctx: Context<VulnerableWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: We check that vault.balance >= amount, but we DON'T
        // verify that ctx.accounts.owner.key() == vault.owner
        // This means ANY signer can withdraw from ANY vault!
        require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
        
        vault.balance = vault.balance.checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;
        
        msg!("Withdrew {} from vault. New balance: {}", amount, vault.balance);
        Ok(())
    }

    // ========================================
    // SECURE IMPLEMENTATION
    // ========================================
    // This section contains the SECURE versions of the same functions
    // with proper account validation using Anchor constraints.

    /// SECURE: Initialize a new vault with proper validation
    /// 
    /// Security Fix: Uses Anchor's `init` constraint which ensures the account
    /// is properly initialized and the `has_one` constraint validates ownership.
    pub fn secure_initialize(ctx: Context<SecureInitialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: The owner is automatically set and validated by Anchor constraints
        // The `init` constraint ensures this is a new account
        // The `has_one` constraint will validate the owner relationship
        vault.owner = ctx.accounts.owner.key();
        vault.balance = initial_balance;
        
        msg!("Secure vault initialized with owner: {} and balance: {}", vault.owner, vault.balance);
        Ok(())
    }

    /// SECURE: Deposit funds with proper account validation
    /// 
    /// Security Fix: Uses `has_one = owner` constraint to ensure the vault
    /// actually belongs to the signer before allowing deposits.
    pub fn secure_deposit(ctx: Context<SecureDeposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: Anchor's `has_one = owner` constraint has already validated
        // that vault.owner == ctx.accounts.owner.key() before this code runs
        vault.balance = vault.balance.checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Securely deposited {} to vault. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// SECURE: Withdraw funds with proper owner validation
    /// 
    /// Security Fix: Uses `has_one = owner` constraint to ensure only the
    /// actual vault owner can withdraw funds.
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: Anchor's `has_one = owner` constraint ensures that
        // vault.owner == ctx.accounts.owner.key() before this code runs
        require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
        
        vault.balance = vault.balance.checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;
        
        msg!("Securely withdrew {} from vault. New balance: {}", amount, vault.balance);
        Ok(())
    }
}

// ========================================
// VULNERABLE ACCOUNT CONTEXTS
// ========================================
// These contexts lack proper validation constraints

#[derive(Accounts)]
pub struct VulnerableInitialize<'info> {
    /// VULNERABILITY: Using `zero` constraint instead of `init` means we don't
    /// get proper initialization protection. An attacker could potentially
    /// reinitialize an existing account.
    #[account(zero)]
    pub vault: Account<'info, Vault>,
    
    /// VULNERABILITY: No validation that this owner is actually authorized
    /// to create this vault. Any pubkey can be passed as owner.
    /// CHECK: This is intentionally unsafe for demonstration
    pub owner: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VulnerableDeposit<'info> {
    /// VULNERABILITY: No `has_one` constraint to validate vault ownership
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    /// VULNERABILITY: We accept any account as owner without validation
    /// CHECK: This is intentionally unsafe for demonstration
    pub owner: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct VulnerableWithdraw<'info> {
    /// VULNERABILITY: No `has_one` constraint to validate vault ownership
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    /// VULNERABILITY: We require a signer but don't validate it's the vault owner
    pub owner: Signer<'info>,
}

// ========================================
// SECURE ACCOUNT CONTEXTS
// ========================================
// These contexts use proper Anchor constraints for security

#[derive(Accounts)]
pub struct SecureInitialize<'info> {
    /// SECURITY: `init` constraint ensures proper initialization and prevents
    /// reinitialization attacks. `payer` and `space` are required for init.
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8, // discriminator + owner + balance
        has_one = owner
    )]
    pub vault: Account<'info, Vault>,
    
    /// SECURITY: Must be a signer to prove ownership authorization
    pub owner: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SecureDeposit<'info> {
    /// SECURITY: `has_one = owner` constraint validates that vault.owner
    /// matches the owner account passed in the context
    #[account(
        mut,
        has_one = owner
    )]
    pub vault: Account<'info, Vault>,
    
    /// SECURITY: Must be a signer and must match vault.owner
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    /// SECURITY: `has_one = owner` constraint validates that vault.owner
    /// matches the owner account passed in the context
    #[account(
        mut,
        has_one = owner
    )]
    pub vault: Account<'info, Vault>,
    
    /// SECURITY: Must be a signer and must match vault.owner
    pub owner: Signer<'info>,
}

// ========================================
// ACCOUNT STRUCTURES
// ========================================

#[account]
pub struct Vault {
    /// The owner of this vault (32 bytes)
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
}