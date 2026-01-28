use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod unsafe_cpi {
    use super::*;

    // ========================================
    // VULNERABLE IMPLEMENTATION
    // ========================================
    // This section contains INTENTIONALLY VULNERABLE code for educational purposes.
    // DO NOT use this code in production environments.

    /// VULNERABLE: Initialize a token vault without proper program validation
    /// 
    /// Security Issue: This function accepts any program as the token program
    /// without validating that it's actually the legitimate SPL Token program.
    /// An attacker could pass a malicious program that mimics the token program
    /// interface but performs unauthorized operations.
    pub fn vulnerable_initialize(ctx: Context<VulnerableInitialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // VULNERABILITY: We're setting up the vault without validating that
        // the token_program is actually the legitimate SPL Token program.
        // An attacker could pass any program ID here.
        vault.authority = ctx.accounts.authority.key();
        vault.token_account = ctx.accounts.token_account.key();
        vault.bump = ctx.bumps.vault;
        
        msg!("Vault initialized with authority: {}", vault.authority);
        msg!("Token account: {}", vault.token_account);
        msg!("WARNING: Token program not validated: {}", ctx.accounts.token_program.key());
        
        Ok(())
    }

    /// VULNERABLE: Transfer tokens using unchecked CPI call
    /// 
    /// Security Issue: This function performs a Cross-Program Invocation (CPI)
    /// to transfer tokens, but it doesn't validate that the token_program
    /// is actually the legitimate SPL Token program. An attacker could substitute
    /// a malicious program that appears to transfer tokens but actually steals them
    /// or performs other unauthorized operations.
    pub fn vulnerable_transfer(ctx: Context<VulnerableTransfer>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        
        // VULNERABILITY: We're making a CPI call to whatever program is passed
        // as token_program without validating its program ID. This is extremely
        // dangerous because:
        // 1. A malicious program could steal tokens instead of transferring them
        // 2. A malicious program could mint unlimited tokens
        // 3. A malicious program could manipulate account data
        // 4. The malicious program could perform any operation with the vault's authority
        
        let seeds = &[
            b"vault",
            vault.authority.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        
        // CRITICAL VULNERABILITY: This CPI call uses whatever program is passed
        // as token_program without any validation!
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        // An attacker could pass a malicious program here that:
        // - Transfers tokens to their own account instead
        // - Mints new tokens to themselves
        // - Drains the vault completely
        // - Performs any other malicious operation
        token::transfer(cpi_ctx, amount)?;
        
        msg!("VULNERABLE: Transferred {} tokens using unchecked program: {}", 
             amount, ctx.accounts.token_program.key());
        
        Ok(())
    }

    /// VULNERABLE: Withdraw tokens with unchecked CPI
    /// 
    /// Security Issue: Similar to vulnerable_transfer, this function makes
    /// CPI calls without validating the token program. This is particularly
    /// dangerous for withdrawal operations as it gives an attacker the ability
    /// to drain funds using a malicious program.
    pub fn vulnerable_withdraw(ctx: Context<VulnerableWithdraw>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        
        // VULNERABILITY: No validation that token_program is legitimate
        // An attacker could pass a malicious program that:
        // 1. Transfers more tokens than requested
        // 2. Transfers tokens to the attacker's account
        // 3. Manipulates the vault's token account balance
        
        let seeds = &[
            b"vault",
            vault.authority.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        
        // CRITICAL VULNERABILITY: Unchecked CPI call
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        msg!("VULNERABLE: Withdrew {} tokens using unchecked program: {}", 
             amount, ctx.accounts.token_program.key());
        
        Ok(())
    }

    // ========================================
    // SECURE IMPLEMENTATION
    // ========================================
    // This section contains the SECURE versions of the same functions
    // with proper program ID validation using Anchor constraints.

    /// SECURE: Initialize a token vault with proper program validation
    /// 
    /// Security Fix: Uses Anchor's Program<'info, Token> type which automatically
    /// validates that the program ID matches the expected SPL Token program.
    /// This prevents attackers from substituting malicious programs.
    pub fn secure_initialize(ctx: Context<SecureInitialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // SECURITY: The token_program is validated by Anchor's Program<'info, Token>
        // type, which ensures it matches the SPL Token program ID
        vault.authority = ctx.accounts.authority.key();
        vault.token_account = ctx.accounts.token_account.key();
        vault.bump = ctx.bumps.vault;
        
        msg!("Secure vault initialized with authority: {}", vault.authority);
        msg!("Token account: {}", vault.token_account);
        msg!("SECURE: Validated token program: {}", ctx.accounts.token_program.key());
        
        Ok(())
    }

    /// SECURE: Transfer tokens using validated CPI call
    /// 
    /// Security Fix: Uses Program<'info, Token> to ensure the token_program
    /// is actually the legitimate SPL Token program before making CPI calls.
    /// This prevents malicious program substitution attacks.
    pub fn secure_transfer(ctx: Context<SecureTransfer>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        
        // SECURITY: Anchor's Program<'info, Token> type has already validated
        // that token_program.key() == spl_token::ID before this code runs.
        // This prevents attackers from passing malicious programs.
        
        let seeds = &[
            b"vault",
            vault.authority.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        
        // SECURITY: This CPI call is safe because Anchor has validated
        // that token_program is actually the SPL Token program
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        msg!("SECURE: Transferred {} tokens using validated SPL Token program", amount);
        
        Ok(())
    }

    /// SECURE: Withdraw tokens with validated CPI
    /// 
    /// Security Fix: Uses Program<'info, Token> to ensure only legitimate
    /// SPL Token program can be used for withdrawal operations.
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        
        // SECURITY: Program<'info, Token> ensures token_program is legitimate
        let seeds = &[
            b"vault",
            vault.authority.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        
        // SECURITY: Safe CPI call with validated program
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        msg!("SECURE: Withdrew {} tokens using validated SPL Token program", amount);
        
        Ok(())
    }
}

// ========================================
// VULNERABLE ACCOUNT CONTEXTS
// ========================================
// These contexts accept any program without validation

#[derive(Accounts)]
pub struct VulnerableInitialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1, // discriminator + authority + token_account + bump
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_account: Account<'info, TokenAccount>,
    
    /// VULNERABILITY: Using UncheckedAccount instead of Program<'info, Token>
    /// This allows ANY program to be passed, including malicious ones
    /// CHECK: This is intentionally unsafe for demonstration
    pub token_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VulnerableTransfer<'info> {
    #[account(
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    
    /// VULNERABILITY: Accepts any program without validation
    /// CHECK: This is intentionally unsafe for demonstration
    pub token_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct VulnerableWithdraw<'info> {
    #[account(
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    
    /// VULNERABILITY: Accepts any program without validation
    /// CHECK: This is intentionally unsafe for demonstration
    pub token_program: UncheckedAccount<'info>,
}

// ========================================
// SECURE ACCOUNT CONTEXTS
// ========================================
// These contexts use proper program validation

#[derive(Accounts)]
pub struct SecureInitialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1, // discriminator + authority + token_account + bump
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_account: Account<'info, TokenAccount>,
    
    /// SECURITY: Program<'info, Token> validates that this is the SPL Token program
    pub token_program: Program<'info, Token>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SecureTransfer<'info> {
    #[account(
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    
    /// SECURITY: Program<'info, Token> ensures this is the legitimate SPL Token program
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(
        seeds = [b"vault", vault.authority.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, TokenVault>,
    
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    
    /// SECURITY: Program<'info, Token> ensures this is the legitimate SPL Token program
    pub token_program: Program<'info, Token>,
}

// ========================================
// ACCOUNT STRUCTURES
// ========================================

#[account]
pub struct TokenVault {
    /// The authority that can manage this vault (32 bytes)
    pub authority: Pubkey,
    /// The token account associated with this vault (32 bytes)
    pub token_account: Pubkey,
    /// The bump seed for PDA derivation (1 byte)
    pub bump: u8,
}

// ========================================
// ERROR DEFINITIONS
// ========================================

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token program - must be SPL Token program")]
    InvalidTokenProgram,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Insufficient token balance")]
    InsufficientBalance,
}