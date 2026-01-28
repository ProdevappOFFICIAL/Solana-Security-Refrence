# Unsafe Cross-Program Invocation (CPI) Vulnerability

## Overview

This example demonstrates one of the most critical security vulnerabilities in Solana programs: **Unsafe Cross-Program Invocation (CPI)**. This vulnerability occurs when a program makes CPI calls to other programs without properly validating the target program's identity, allowing attackers to substitute malicious programs that can steal funds, manipulate data, or perform unauthorized operations.

## Vulnerability Description

### What is Cross-Program Invocation (CPI)?

Cross-Program Invocation (CPI) is Solana's mechanism for programs to call other programs. It's similar to making API calls between different services, but in the blockchain context. When your program needs to interact with another program (like transferring SPL tokens), it uses CPI to invoke functions in that target program.

### The Security Risk

The critical vulnerability occurs when programs accept **any program ID** for CPI calls without validation. An attacker can:

1. **Deploy a malicious program** that mimics the interface of a legitimate program (like SPL Token)
2. **Substitute the malicious program ID** in place of the legitimate program
3. **Execute unauthorized operations** using the calling program's authority

### Why This is Extremely Dangerous

When a program makes a CPI call, it often delegates its authority to the target program. If the target program is malicious, it can:

- **Steal tokens** instead of transferring them legitimately
- **Mint unlimited tokens** to the attacker's accounts
- **Manipulate account data** in unauthorized ways
- **Drain entire vaults** or protocol treasuries
- **Perform any operation** that the calling program has authority to do

## Code Examples

### Vulnerable Implementation

```rust
/// VULNERABLE: Transfer tokens using unchecked CPI call
pub fn vulnerable_transfer(ctx: Context<VulnerableTransfer>, amount: u64) -> Result<()> {
    // VULNERABILITY: No validation of token_program identity
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    
    // CRITICAL VULNERABILITY: This accepts ANY program!
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
    token::transfer(cpi_ctx, amount)?; // Calls whatever program was passed!
    Ok(())
}

#[derive(Accounts)]
pub struct VulnerableTransfer<'info> {
    // ... other accounts ...
    
    /// VULNERABILITY: Accepts ANY program without validation
    /// CHECK: This is intentionally unsafe for demonstration
    pub token_program: UncheckedAccount<'info>,
}
```

### Secure Implementation

```rust
/// SECURE: Transfer tokens using validated CPI call
pub fn secure_transfer(ctx: Context<SecureTransfer>, amount: u64) -> Result<()> {
    // SECURITY: Anchor has validated token_program is legitimate SPL Token program
    let cpi_accounts = Transfer {
        from: ctx.accounts.from_token_account.to_account_info(),
        to: ctx.accounts.to_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    
    // SECURITY: Safe CPI call with validated program
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
    token::transfer(cpi_ctx, amount)?; // Only calls legitimate SPL Token program
    Ok(())
}

#[derive(Accounts)]
pub struct SecureTransfer<'info> {
    // ... other accounts ...
    
    /// SECURITY: Program<'info, Token> validates program ID
    pub token_program: Program<'info, Token>,
}
```

## Key Security Differences

| Aspect | Vulnerable | Secure |
|--------|------------|--------|
| **Program Validation** | None - accepts any program | Validates program ID matches expected |
| **Account Type** | `UncheckedAccount<'info>` | `Program<'info, Token>` |
| **Attack Surface** | Any malicious program can be substituted | Only legitimate SPL Token program accepted |
| **Risk Level** | **CRITICAL** - Complete compromise possible | **SAFE** - Protected against substitution attacks |

## Attack Scenarios

### Scenario 1: Token Theft
1. Attacker deploys malicious program that mimics SPL Token interface
2. Attacker calls vulnerable function with malicious program ID
3. Instead of transferring tokens to intended recipient, malicious program transfers to attacker
4. Vault is drained, users lose funds

### Scenario 2: Unlimited Minting
1. Malicious program implements mint function instead of transfer
2. Attacker calls vulnerable function requesting small transfer
3. Malicious program mints unlimited tokens to attacker's account
4. Token supply is inflated, protocol is compromised

### Scenario 3: Data Manipulation
1. Malicious program manipulates account data during "transfer"
2. Vault balances are corrupted or set to attacker's advantage
3. Protocol accounting becomes inconsistent
4. Further exploits become possible due to corrupted state

## Prevention Strategies

### 1. Use Anchor's Program Types
```rust
// ✅ SECURE: Use Program<'info, Token> for SPL Token
pub token_program: Program<'info, Token>,

// ❌ VULNERABLE: Never use UncheckedAccount for programs
pub token_program: UncheckedAccount<'info>,
```

### 2. Manual Program ID Validation
```rust
// If you must use UncheckedAccount, validate manually:
require_keys_eq!(
    ctx.accounts.token_program.key(),
    spl_token::ID,
    ErrorCode::InvalidTokenProgram
);
```

### 3. Constraint-Based Validation
```rust
#[derive(Accounts)]
pub struct SecureContext<'info> {
    #[account(
        constraint = token_program.key() == spl_token::ID @ ErrorCode::InvalidTokenProgram
    )]
    pub token_program: UncheckedAccount<'info>,
}
```

## Best Practices

### ✅ Do's
- **Always validate program IDs** before making CPI calls
- **Use Anchor's Program types** when available (e.g., `Program<'info, Token>`)
- **Implement explicit checks** if using `UncheckedAccount`
- **Test with malicious programs** during development
- **Document expected program IDs** in your code

### ❌ Don'ts
- **Never use `UncheckedAccount`** for program accounts without validation
- **Don't assume** client-side validation is sufficient
- **Don't trust** program IDs passed by users
- **Don't skip** program ID validation for "internal" functions
- **Don't rely** on program interface similarity for security

## Testing Your Implementation

Run the test suite to see the vulnerability in action:

```bash
anchor test
```

The tests demonstrate:
1. **Vulnerable functions** accepting malicious program IDs
2. **Secure functions** rejecting invalid programs
3. **Attack scenarios** showing potential exploitation
4. **Protection verification** confirming security measures work

## Real-World Impact

Unsafe CPI vulnerabilities have led to some of the largest DeFi exploits in Solana's history:

- **Wormhole Bridge Exploit (2022)**: $320M stolen due to signature verification bypass
- **Cashio Exploit (2022)**: $52M drained through malicious mint authority
- **Crema Finance Exploit (2022)**: $8.8M stolen via program substitution

These incidents highlight the critical importance of proper CPI validation in production systems.

## Conclusion

Unsafe CPI is one of the most dangerous vulnerabilities in Solana development. The ability for attackers to substitute malicious programs can lead to complete protocol compromise. Always validate program IDs before making CPI calls, and use Anchor's type system to enforce these validations automatically.

Remember: **Trust but verify** - even if a program has the right interface, ensure it has the right program ID before delegating your authority to it.