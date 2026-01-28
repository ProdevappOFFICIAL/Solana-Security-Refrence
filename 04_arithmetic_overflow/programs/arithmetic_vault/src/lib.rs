use anchor_lang::prelude::*;

declare_id!("ArithmeticVault11111111111111111111111111111");

#[program]
pub mod arithmetic_vault {
    use super::*;

    // ========================================
    // VULNERABLE IMPLEMENTATION
    // ========================================
    // This section contains INTENTIONALLY VULNERABLE code for educational purposes.
    // DO NOT use this code in production environments.

    /// VULNERABLE: Initialize vault with unchecked arithmetic
    /// 
    /// Security Issue: This function uses unchecked arithmetic operations that can
    /// silently overflow or underflow in release mode, leading to corrupted state.
    /// In Rust, integer overflow panics in debug mode but wraps around silently
    /// in release mode, which can be exploited by attackers.
    pub fn vulnerable_initialize(ctx: Context<VulnerableInitialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        vault.owner = ctx.accounts.owner.key();
        // VULNERABILITY: Direct assignment without overflow checks
        // If initial_balance is near u64::MAX, any subsequent additions will overflow
        vault.balance = initial_balance;
        vault.total_deposits = 0;
        vault.total_withdrawals = 0;
        
        msg!("Vault initialized with balance: {}", vault.balance);
        Ok(())
    }

    /// VULNERABLE: Deposit with unchecked addition
    /// 
    /// Security Issue: Uses unchecked addition which can overflow silently in release mode.
    /// An attacker could deposit a large amount that causes the balance to wrap around
    /// to a small number, effectively stealing funds from the vault.
    pub fn vulnerable_deposit(ctx: Context<VulnerableDeposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: Unchecked arithmetic - can overflow silently
        // If vault.balance + amount > u64::MAX, this will wrap around to a small number
        vault.balance = vault.balance + amount;
        vault.total_deposits = vault.total_deposits + amount;
        
        msg!("Deposited {}. New balance: {} (unchecked)", amount, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Withdraw with unchecked subtraction
    /// 
    /// Security Issue: Uses unchecked subtraction which can underflow silently.
    /// An attacker could withdraw more than the balance, causing underflow that
    /// wraps around to a very large number, creating funds out of thin air.
    pub fn vulnerable_withdraw(ctx: Context<VulnerableWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: No balance check and unchecked subtraction
        // If amount > vault.balance, this will underflow and wrap to a huge number
        vault.balance = vault.balance - amount;
        vault.total_withdrawals = vault.total_withdrawals + amount;
        
        msg!("Withdrew {}. New balance: {} (unchecked)", amount, vault.balance);
        Ok(())
    }

    /// VULNERABLE: Compound interest calculation with unchecked multiplication
    /// 
    /// Security Issue: Multiplication can easily overflow with large balances or rates.
    /// This could be exploited to either drain funds (overflow to small number) or
    /// create excessive funds (if overflow detection is inconsistent).
    pub fn vulnerable_apply_interest(ctx: Context<VulnerableApplyInterest>, rate_basis_points: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: Unchecked multiplication can overflow
        // Large balance * rate could overflow, wrapping to unexpected values
        let interest = vault.balance * rate_basis_points / 10000;
        vault.balance = vault.balance + interest;
        
        msg!("Applied interest. New balance: {} (unchecked)", vault.balance);
        Ok(())
    }

    /// VULNERABLE: Batch operation with accumulated overflow risk
    /// 
    /// Security Issue: Multiple unchecked operations compound the overflow risk.
    /// Even if individual operations seem safe, the accumulation can cause overflow.
    pub fn vulnerable_batch_deposit(ctx: Context<VulnerableBatchDeposit>, amounts: Vec<u64>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        for amount in amounts {
            // VULNERABILITY: Multiple unchecked additions increase overflow probability
            vault.balance = vault.balance + amount;
            vault.total_deposits = vault.total_deposits + amount;
        }
        
        msg!("Batch deposit completed. New balance: {} (unchecked)", vault.balance);
        Ok(())
    }

    // ========================================
    // SECURE IMPLEMENTATION
    // ========================================
    // This section shows the SECURE way to handle arithmetic operations
    // using checked arithmetic to prevent overflow/underflow vulnerabilities.

    /// SECURE: Initialize vault with proper validation
    pub fn secure_initialize(ctx: Context<SecureInitialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        vault.owner = ctx.accounts.owner.key();
        vault.balance = initial_balance;
        vault.total_deposits = 0;
        vault.total_withdrawals = 0;
        
        msg!("Vault securely initialized with balance: {}", vault.balance);
        Ok(())
    }

    /// SECURE: Deposit with checked addition
    /// 
    /// Security Fix: Uses checked_add() which returns None on overflow,
    /// allowing us to handle the error gracefully instead of silent corruption.
    pub fn secure_deposit(ctx: Context<SecureDeposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY FIX: Use checked arithmetic to prevent overflow
        vault.balance = vault.balance
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
            
        vault.total_deposits = vault.total_deposits
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Securely deposited {}. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// SECURE: Withdraw with balance validation and checked subtraction
    /// 
    /// Security Fix: Validates sufficient balance and uses checked_sub()
    /// to prevent underflow attacks.
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY FIX: Validate sufficient balance before withdrawal
        if vault.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        
        // SECURITY FIX: Use checked subtraction to prevent underflow
        vault.balance = vault.balance
            .checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticUnderflow)?;
            
        vault.total_withdrawals = vault.total_withdrawals
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Securely withdrew {}. New balance: {}", amount, vault.balance);
        Ok(())
    }

    /// SECURE: Interest calculation with checked multiplication and division
    /// 
    /// Security Fix: Uses checked arithmetic for all operations and validates
    /// intermediate results to prevent overflow in compound calculations.
    pub fn secure_apply_interest(ctx: Context<SecureApplyInterest>, rate_basis_points: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY FIX: Use checked multiplication and validate intermediate results
        let interest_numerator = vault.balance
            .checked_mul(rate_basis_points)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
            
        let interest = interest_numerator
            .checked_div(10000)
            .ok_or(ErrorCode::DivisionByZero)?;
            
        vault.balance = vault.balance
            .checked_add(interest)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Securely applied interest. New balance: {}", vault.balance);
        Ok(())
    }

    /// SECURE: Batch operation with overflow protection
    /// 
    /// Security Fix: Each operation uses checked arithmetic, and we validate
    /// the total before applying to prevent accumulated overflow.
    pub fn secure_batch_deposit(ctx: Context<SecureBatchDeposit>, amounts: Vec<u64>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY FIX: Calculate total first to check for overflow
        let mut total_amount = 0u64;
        for amount in &amounts {
            total_amount = total_amount
                .checked_add(*amount)
                .ok_or(ErrorCode::ArithmeticOverflow)?;
        }
        
        // SECURITY FIX: Check if adding total would overflow before applying
        vault.balance = vault.balance
            .checked_add(total_amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
            
        vault.total_deposits = vault.total_deposits
            .checked_add(total_amount)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
        
        msg!("Securely completed batch deposit. New balance: {}", vault.balance);
        Ok(())
    }
}

// ========================================
// ACCOUNT STRUCTURES
// ========================================

#[account]
pub struct ArithmeticVault {
    pub owner: Pubkey,           // 32 bytes
    pub balance: u64,            // 8 bytes - main balance that can overflow
    pub total_deposits: u64,     // 8 bytes - running total of deposits
    pub total_withdrawals: u64,  // 8 bytes - running total of withdrawals
}

// ========================================
// VULNERABLE CONTEXTS
// ========================================

#[derive(Accounts)]
pub struct VulnerableInitialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub vault: Account<'info, ArithmeticVault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VulnerableDeposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub depositor: Signer<'info>,
}

#[derive(Accounts)]
pub struct VulnerableWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub withdrawer: Signer<'info>,
}

#[derive(Accounts)]
pub struct VulnerableApplyInterest<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct VulnerableBatchDeposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub depositor: Signer<'info>,
}

// ========================================
// SECURE CONTEXTS
// ========================================

#[derive(Accounts)]
pub struct SecureInitialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]
    pub vault: Account<'info, ArithmeticVault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SecureDeposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub depositor: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub withdrawer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureApplyInterest<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureBatchDeposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, ArithmeticVault>,
    pub depositor: Signer<'info>,
}

// ========================================
// ERROR DEFINITIONS
// ========================================

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
}