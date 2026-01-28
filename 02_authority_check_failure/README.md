# Authority Check Failure - Solana Security Example

## Overview

This example demonstrates one of the most critical and common security vulnerabilities in Solana programs: **Authority Check Failure**. This vulnerability occurs when a program validates that someone signed a transaction (signature validation) but fails to verify that the signer has the actual authority to perform the requested operation (authority validation).

## The Critical Distinction: Signature vs Authority

### Signature Validation ✅ vs Authority Validation ❌

Many developers mistakenly believe that checking `Signer<'info>` is sufficient for security. However, this only proves that:
- Someone has the private key for an account
- They signed the transaction

It does **NOT** prove that:
- They are authorized to perform this specific operation
- They have the right to access this specific resource
- They are the designated admin/owner of this particular account

### Real-World Impact

This vulnerability has led to millions of dollars in losses across the Solana ecosystem:
- **Wormhole Bridge Hack (2022)**: $320M loss due to signature validation without proper authority checks
- **Cashio Hack (2022)**: $52M loss from similar authority bypass vulnerabilities
- **Numerous smaller protocols**: Hundreds of thousands in losses from admin function bypasses

## Vulnerability Examples

### 1. Admin Withdrawal Without Authority Check

```rust
// ❌ VULNERABLE: Only checks signature, not authority
pub fn vulnerable_admin_withdraw(ctx: Context<VulnerableAdminWithdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // VULNERABILITY: We check that someone signed (admin is Signer<'info>)
    // but we DON'T verify that ctx.accounts.admin.key() == vault.admin
    // This means ANY signer can withdraw as long as they're passed as "admin"!
    require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
    
    vault.balance = vault.balance.checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct VulnerableAdminWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, AdminVault>,
    pub admin: Signer<'info>, // ❌ Only validates signature, not authority!
}
```

### 2. Secure Implementation with Authority Validation

```rust
// ✅ SECURE: Validates both signature AND authority
pub fn secure_admin_withdraw(ctx: Context<SecureAdminWithdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // SECURITY: Anchor's `has_one = admin` constraint has already validated
    // that vault.admin == ctx.accounts.admin.key() before this code runs
    require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
    
    vault.balance = vault.balance.checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct SecureAdminWithdraw<'info> {
    #[account(
        mut,
        has_one = admin  // ✅ Validates that vault.admin == admin.key()
    )]
    pub vault: Account<'info, AdminVault>,
    pub admin: Signer<'info>, // ✅ Must be signer AND must match vault.admin
}
```

## Attack Scenarios

### Scenario 1: Unauthorized Withdrawal
1. Attacker discovers a vault with admin functions
2. Attacker calls `vulnerable_admin_withdraw` passing their own key as `admin`
3. Program only checks that attacker signed the transaction
4. Program doesn't verify attacker is the actual admin of the vault
5. Attacker successfully withdraws funds from any vault

### Scenario 2: Admin Takeover
1. Attacker finds a vault with `vulnerable_change_admin` function
2. Attacker calls the function passing their key as both current and new admin
3. Program only validates that someone signed the transaction
4. Attacker becomes the new admin of the vault
5. Attacker can now perform all admin operations legitimately

### Scenario 3: Emergency Drain Exploit
1. Attacker identifies vaults with emergency functions
2. Attacker calls `vulnerable_emergency_drain` with their signature
3. Program drains the entire vault without validating admin authority
4. Attacker repeats across multiple vaults for maximum damage

## Prevention Strategies

### 1. Use Anchor's `has_one` Constraint

```rust
#[derive(Accounts)]
pub struct SecureOperation<'info> {
    #[account(
        mut,
        has_one = admin @ ErrorCode::UnauthorizedAdmin
    )]
    pub vault: Account<'info, AdminVault>,
    pub admin: Signer<'info>,
}
```

### 2. Manual Authority Validation

```rust
pub fn secure_operation(ctx: Context<Operation>) -> Result<()> {
    // Manual validation when constraints aren't sufficient
    require!(
        ctx.accounts.vault.admin == ctx.accounts.admin.key(),
        ErrorCode::UnauthorizedAdmin
    );
    
    // Rest of the function logic...
    Ok(())
}
```

### 3. Role-Based Access Control

```rust
#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub owner: Pubkey,
    pub operators: Vec<Pubkey>, // Multiple authorized operators
}

// Validate against multiple possible authorities
require!(
    ctx.accounts.vault.admin == ctx.accounts.signer.key() ||
    ctx.accounts.vault.operators.contains(&ctx.accounts.signer.key()),
    ErrorCode::UnauthorizedAccess
);
```

## Testing and Validation

### Running the Tests

```bash
# Install dependencies
npm install

# Build the program
anchor build

# Run the exploit demonstration tests
anchor test
```

### Test Categories

1. **Exploit Tests**: Demonstrate successful attacks on vulnerable code
2. **Protection Tests**: Verify secure implementations block attacks
3. **Legitimate Tests**: Confirm authorized operations work correctly

### Expected Test Output

```
✅ EXPLOIT SUCCESS: Attacker withdrew funds without being real admin
✅ PROTECTION SUCCESS: Secure function blocked unauthorized withdrawal  
✅ LEGITIMATE: Real admin successfully performed operations
```

## Security Checklist

When reviewing Solana programs for authority vulnerabilities:

- [ ] **Signature vs Authority**: Does the program distinguish between having a signature and having authority?
- [ ] **Account Relationships**: Are account relationships properly validated using `has_one` constraints?
- [ ] **Admin Functions**: Do administrative functions verify the signer is the actual admin?
- [ ] **State Changes**: Are state-changing operations protected by proper authority checks?
- [ ] **Error Handling**: Are authority failures handled with clear, specific error messages?
- [ ] **Test Coverage**: Are both exploit and protection scenarios tested?

## Common Pitfalls

### ❌ Assuming Signer Equals Authority
```rust
// Wrong assumption: If someone can sign, they have authority
pub admin: Signer<'info>, // Only proves they have the private key
```

### ❌ Client-Side Validation Only
```rust
// Wrong: Trusting client to pass correct admin account
// Attacker can pass any account as admin
```

### ❌ Inconsistent Authority Patterns
```rust
// Wrong: Some functions check authority, others don't
// Creates confusion and potential vulnerabilities
```

## Framework-Specific Guidance

### Anchor Framework
- **Strengths**: Declarative constraints make authority validation explicit
- **Best Practice**: Always use `has_one` constraints for authority relationships
- **Pitfall**: Forgetting to add constraints to account contexts

### Pinocchio Framework  
- **Strengths**: Explicit validation gives full control
- **Best Practice**: Manual validation of all authority relationships
- **Pitfall**: Easy to forget validation steps in complex functions

## Related Vulnerabilities

- **Missing Account Validation**: Broader category including this vulnerability
- **Privilege Escalation**: Often combined with authority check failures
- **Access Control Bypass**: General pattern of circumventing security checks

## Additional Resources

- [Anchor Security Guidelines](https://www.anchor-lang.com/docs/account-constraints)
- [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security)
- [Real-World Exploit Analysis](./EXPLOIT.md)

---

**⚠️ Security Warning**: The vulnerable code in this example is for educational purposes only. Never use vulnerable patterns in production code. Always validate authority relationships using proper constraints and manual checks.