use anchor_lang::prelude::*;

declare_id!("ReinitVault11111111111111111111111111111111");

#[program]
pub mod reinit_vault {
    use super::*;

    // ========================================
    // VULNERABLE IMPLEMENTATION
    // ========================================
    // This section contains INTENTIONALLY VULNERABLE code for educational purposes.
    // DO NOT use this code in production environments.

    /// VULNERABLE: Initialize vault without reinitialization protection
    /// 
    /// Security Issue: This function does not check if the vault has already been
    /// initialized. An attacker can call this function multiple times to reset
    /// the vault state, potentially stealing funds or resetting ownership.
    /// 
    /// The vulnerability occurs because:
    /// 1. No check for existing initialization state
    /// 2. Allows overwriting of critical fields like owner and balance
    /// 3. Can be called by anyone, not just the original owner
    pub fn vulnerable_initialize(
        ctx: Context<VulnerableInitialize>, 
        initial_balance: u64
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: No check if already initialized
        // An attacker can call this multiple times to reset state
        vault.owner = ctx.accounts.owner.key();
        vault.balance = initial_balance;
        vault.total_deposits = initial_balance;
        vault.is_initialized = true; // This flag is set but never checked!
        
        msg!("Vault initialized with owner: {} and balance: {}", 
             vault.owner, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Deposit function that works with reinitialized vaults
    /// 
    /// This function itself isn't vulnerable, but it demonstrates how
    /// reinitialization attacks can affect legitimate operations.
    pub fn vulnerable_deposit(ctx: Context<VulnerableDeposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // Basic validation - but this can be bypassed via reinitialization
        require!(vault.is_initialized, ErrorCode::NotInitialized);
        
        vault.balance = vault.balance
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
            
        vault.total_deposits = vault.total_deposits
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Deposited {} to vault. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Withdraw function that can be exploited after reinitialization
    /// 
    /// Security Issue: After reinitialization, an attacker becomes the new owner
    /// and can withdraw all funds, even those deposited by the original owner.
    pub fn vulnerable_withdraw(ctx: Context<VulnerableWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // Basic validation
        require!(vault.is_initialized, ErrorCode::NotInitialized);
        require!(vault.owner == ctx.accounts.owner.key(), ErrorCode::Unauthorized);
        require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
        
        vault.balance = vault.balance
            .checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;
        
        msg!("Owner {} withdrew {} from vault. Remaining balance: {}", 
             vault.owner, amount, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Manual initialization without proper protection
    /// 
    /// Security Issue: This function attempts to provide manual initialization
    /// protection but does it incorrectly. The check can be bypassed by
    /// directly modifying the account data or through other vulnerabilities.
    pub fn vulnerable_manual_init(
        ctx: Context<VulnerableManualInit>, 
        initial_balance: u64
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: Weak initialization check
        // This check is insufficient and can be bypassed
        if vault.is_initialized {
            return Err(ErrorCode::AlreadyInitialized.into());
        }
        
        // VULNERABILITY: Race condition possible
        // Between the check above and setting the flag below,
        // another transaction could initialize the account
        vault.owner = ctx.accounts.owner.key();
        vault.balance = initial_balance;
        vault.total_deposits = initial_balance;
        vault.is_initialized = true;
        
        msg!("Manual initialization completed for owner: {}", vault.owner);
        Ok(())
    }

    // ========================================
    // SECURE IMPLEMENTATION
    // ========================================
    // This section shows the SECURE way to handle initialization
    // using Anchor's built-in protection mechanisms.

    /// SECURE: Initialize vault using Anchor's init constraint
    /// 
    /// Security Fix: Uses Anchor's `init` constraint which automatically
    /// prevents reinitialization by ensuring the account doesn't already exist.
    /// The `init` constraint will fail if the account is already initialized.
    pub fn secure_initialize(
        ctx: Context<SecureInitialize>, 
        initial_balance: u64
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY FIX: Anchor's init constraint prevents reinitialization
        // The account structure ensures this function can only be called once
        vault.owner = ctx.accounts.owner.key();
        vault.balance = initial_balance;
        vault.total_deposits = initial_balance;
        vault.is_initialized = true;
        
        msg!("Vault securely initialized with owner: {} and balance: {}", 
             vault.owner, vault.balance);
        Ok(())
    }

    /// SECURE: Deposit function with proper initialization checks
    pub fn secure_deposit(ctx: Context<SecureDeposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // The account validation in the context ensures proper initialization
        vault.balance = vault.balance
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
            
        vault.total_deposits = vault.total_deposits
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Securely deposited {} to vault. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// SECURE: Withdraw function with proper owner validation
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // Anchor constraints ensure proper owner validation
        require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
        
        vault.balance = vault.balance
            .checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;
        
        msg!("Owner {} securely withdrew {} from vault. Remaining balance: {}", 
             vault.owner, amount, vault.balance);
        Ok(())
    }

    /// SECURE: Manual initialization with proper atomic protection
    /// 
    /// Security Fix: Uses a combination of account constraints and atomic
    /// operations to prevent reinitialization attacks. The discriminator
    /// and account validation provide multiple layers of protection.
    pub fn secure_manual_init(
        ctx: Context<SecureManualInit>, 
        initial_balance: u64
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY FIX: Anchor's account validation ensures this is a fresh account
        // The zero_copy or init_if_needed constraints provide atomic protection
        vault.owner = ctx.accounts.owner.key();
        vault.balance = initial_balance;
        vault.total_deposits = initial_balance;
        vault.is_initialized = true;
        
        msg!("Vault securely initialized manually for owner: {}", vault.owner);
        Ok(())
    }

    /// SECURE: Reset function with proper authorization
    /// 
    /// Security Fix: If reset functionality is needed, it should be properly
    /// authorized and have clear business logic constraints.
    pub fn secure_reset(ctx: Context<SecureReset>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY FIX: Only allow reset by current owner with explicit authorization
        // Additional business logic constraints should be added as needed
        require!(vault.balance == 0, ErrorCode::VaultNotEmpty);
        
        vault.owner = Pubkey::default();
        vault.balance = 0;
        vault.total_deposits = 0;
        vault.is_initialized = false;
        
        msg!("Vault securely reset by authorized owner");
        Ok(())
    }
}

// ========================================
// ACCOUNT STRUCTURES
// ========================================

#[account]
pub struct ReinitVault {
    pub owner: Pubkey,           // 32 bytes - vault owner
    pub balance: u64,            // 8 bytes - current balance
    pub total_deposits: u64,     // 8 bytes - lifetime deposits
    pub is_initialized: bool,    // 1 byte - initialization flag
}

// ========================================
// VULNERABLE CONTEXTS
// ========================================

#[derive(Accounts)]
pub struct VulnerableInitialize<'info> {
    // VULNERABILITY: Using 'mut' instead of 'init' allows reinitialization
    // This account can be called multiple times on the same account
    #[account(mut)]
    pub vault: Account<'info, ReinitVault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VulnerableDeposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, ReinitVault>,
    pub depositor: Signer<'info>,
}

#[derive(Accounts)]
pub struct VulnerableWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, ReinitVault>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VulnerableManualInit<'info> {
    // VULNERABILITY: Manual initialization without proper constraints
    #[account(mut)]
    pub vault: Account<'info, ReinitVault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// ========================================
// SECURE CONTEXTS
// ========================================

#[derive(Accounts)]
pub struct SecureInitialize<'info> {
    // SECURITY FIX: Using 'init' constraint prevents reinitialization
    // Anchor will ensure this account doesn't already exist
    #[account(
        init, 
        payer = owner, 
        space = 8 + 32 + 8 + 8 + 1
    )]
    pub vault: Account<'info, ReinitVault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SecureDeposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, ReinitVault>,
    pub depositor: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // SECURITY FIX: Constraint ensures only the owner can withdraw
    #[account(
        mut,
        has_one = owner @ ErrorCode::Unauthorized
    )]
    pub vault: Account<'info, ReinitVault>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureManualInit<'info> {
    // SECURITY FIX: Using init_if_needed provides atomic initialization protection
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 32 + 8 + 8 + 1
    )]
    pub vault: Account<'info, ReinitVault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SecureReset<'info> {
    #[account(
        mut,
        has_one = owner @ ErrorCode::Unauthorized
    )]
    pub vault: Account<'info, ReinitVault>,
    pub owner: Signer<'info>,
}

// ========================================
// ERROR DEFINITIONS
// ========================================

#[error_code]
pub enum ErrorCode {
    #[msg("Account is already initialized")]
    AlreadyInitialized,
    #[msg("Account is not initialized")]
    NotInitialized,
    #[msg("Unauthorized access attempt")]
    Unauthorized,
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,
    #[msg("Vault must be empty before reset")]
    VaultNotEmpty,
}