# Missing Account Validation Vulnerability

## Overview

Missing account validation is one of the most critical and common vulnerabilities in Solana programs. This vulnerability occurs when a program fails to verify that accounts passed to instructions have the expected relationships or properties, allowing attackers to manipulate program state by providing malicious or unauthorized accounts.

## The Vulnerability

### What is Missing Account Validation?

In Solana programs, instructions receive a list of accounts that the program operates on. The program must validate that these accounts have the expected relationships and properties before performing operations. When this validation is missing or insufficient, attackers can:

1. **Pass arbitrary accounts** as owners or authorities
2. **Manipulate account relationships** to gain unauthorized access
3. **Bypass access controls** by providing accounts they control
4. **Steal funds** from accounts they don't legitimately own

### Why This Happens

Developers often make these mistakes:

- **Assuming account relationships** without explicit validation
- **Only checking signatures** without verifying account ownership
- **Using `UncheckedAccount`** without proper validation
- **Missing Anchor constraints** like `has_one` or `constraint`
- **Manual validation errors** when not using framework constraints

## Code Examples

### Vulnerable Implementation

```rust
// VULNERABLE: No validation that vault.owner matches the signer
pub fn vulnerable_withdraw(ctx: Context<VulnerableWithdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // Only checks balance, not ownership!
    require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
    
    vault.balance = vault.balance.checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct VulnerableWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>, // Any signer can claim to be owner!
}
```

### Secure Implementation

```rust
// SECURE: Validates that vault.owner matches the signer
pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // Anchor's has_one constraint already validated ownership
    require!(vault.balance >= amount, ErrorCode::InsufficientFunds);
    
    vault.balance = vault.balance.checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticUnderflow)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(
        mut,
        has_one = owner  // This validates vault.owner == owner.key()
    )]
    pub vault: Account<'info, Vault>,
    pub owner: Signer<'info>,
}
```

## Attack Scenarios

### Scenario 1: Direct Fund Theft

1. **Victim** creates a vault and deposits funds
2. **Attacker** discovers the vault's public key
3. **Attacker** calls `vulnerable_withdraw` with their own keypair as owner
4. **Program** doesn't validate that attacker owns the vault
5. **Result**: Attacker steals victim's funds

### Scenario 2: Vault Impersonation

1. **Attacker** creates a vault claiming to be someone else's owner
2. **Attacker** deposits funds to make it look legitimate
3. **Victim** or others interact with the fake vault
4. **Attacker** can manipulate or steal funds from interactions

## Real-World Impact

### Historical Incidents

- **Wormhole Bridge Hack (February 2022)**: $320M stolen due to insufficient signature validation
- **Solana Ecosystem Exploits**: Multiple incidents involving account validation failures
- **DeFi Protocol Exploits**: Numerous smaller incidents where missing validation led to fund theft

### Financial Impact

Missing account validation vulnerabilities have resulted in:
- **$500M+** in total losses across the Solana ecosystem
- **Average loss per incident**: $2-50M
- **Recovery rate**: Less than 10% of stolen funds recovered

## Prevention Strategies

### 1. Use Anchor Constraints

```rust
#[account(
    mut,
    has_one = owner,                    // Validates ownership
    constraint = vault.balance >= amount // Custom validation
)]
pub vault: Account<'info, Vault>,
```

### 2. Manual Validation (when constraints aren't enough)

```rust
require!(
    vault.owner == ctx.accounts.owner.key(),
    ErrorCode::UnauthorizedOwner
);
```

### 3. Avoid UncheckedAccount

```rust
// BAD
pub owner: UncheckedAccount<'info>,

// GOOD
pub owner: Signer<'info>,
```

### 4. Validate All Account Relationships

- **Owner relationships**: Ensure accounts belong to expected owners
- **Authority relationships**: Verify administrative permissions
- **State consistency**: Check that account states are valid
- **Program ownership**: Ensure accounts are owned by expected programs

## Testing Your Code

### Exploit Tests

Write tests that attempt to exploit your program:

```typescript
it("Should prevent unauthorized withdrawal", async () => {
  try {
    await program.methods
      .withdraw(new anchor.BN(1000))
      .accounts({
        vault: victimVault.publicKey,
        owner: attacker.publicKey, // Wrong owner!
      })
      .signers([attacker])
      .rpc();
    
    expect.fail("Expected transaction to fail");
  } catch (error) {
    expect(error.message).to.include("has_one");
  }
});
```

### Security Checklist

- [ ] All account relationships validated with `has_one` or manual checks
- [ ] No `UncheckedAccount` without explicit validation
- [ ] Authority checks implemented for administrative functions
- [ ] Exploit tests written and passing
- [ ] Edge cases tested (empty accounts, wrong types, etc.)

## Running This Example

```bash
# Install dependencies
npm install

# Build the program
anchor build

# Run tests (including exploit demonstrations)
anchor test

# Deploy to localnet
anchor deploy
```

## Key Takeaways

1. **Never trust account relationships** - always validate explicitly
2. **Use Anchor constraints** whenever possible for automatic validation
3. **Write exploit tests** to verify your security measures work
4. **Validate early** - check account relationships before any state changes
5. **Be explicit** - prefer clear validation over implicit assumptions

## Next Steps

- Review the [EXPLOIT.md](./EXPLOIT.md) file for detailed attack walkthrough
- Examine the test suite to understand how exploits work
- Practice implementing secure account validation in your own programs
- Study the secure implementation patterns used in this example

---

⚠️ **Warning**: The vulnerable code in this example is for educational purposes only. Never use vulnerable patterns in production code.