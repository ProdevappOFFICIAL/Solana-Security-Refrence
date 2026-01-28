# Arithmetic Overflow/Underflow Vulnerability

## Overview

This example demonstrates one of the most dangerous and subtle vulnerabilities in Solana programs: **arithmetic overflow and underflow**. These vulnerabilities occur when mathematical operations exceed the bounds of their data types, leading to silent wraparound behavior that can corrupt program state and enable devastating exploits.

## The Vulnerability

### What is Arithmetic Overflow/Underflow?

**Arithmetic Overflow** occurs when a mathematical operation produces a result that exceeds the maximum value that can be stored in the data type. In Rust:
- `u64::MAX` is `18,446,744,073,709,551,615`
- Adding 1 to `u64::MAX` wraps around to `0`

**Arithmetic Underflow** occurs when subtraction produces a result below the minimum value:
- Subtracting 1 from `0u64` wraps around to `u64::MAX`

### Why This is Critical in Solana

1. **Silent Failures**: In Rust release mode, overflow/underflow wraps around silently without panicking
2. **Financial Impact**: Balance corruption can lead to fund theft or creation of funds from nothing
3. **State Corruption**: Wrapped values can break program logic and invariants
4. **Compound Effects**: Multiple operations can accumulate overflow risk

### Real-World Impact

Arithmetic vulnerabilities have caused some of the largest DeFi exploits:

- **Compound Protocol (2020)**: Integer overflow in interest calculation led to $80M+ at risk
- **bZx Protocol (2020)**: Arithmetic errors in flash loan calculations caused multiple exploits
- **Various Solana Programs**: Overflow in token calculations has led to unauthorized minting

## Vulnerable Code Patterns

### 1. Unchecked Addition (Balance Inflation)
```rust
// VULNERABLE: Can overflow silently
vault.balance = vault.balance + amount;
```

**Attack Scenario**: Attacker deposits `u64::MAX - current_balance + 1`, causing overflow that wraps the balance to a small number, effectively stealing funds.

### 2. Unchecked Subtraction (Balance Underflow)
```rust
// VULNERABLE: Can underflow silently  
vault.balance = vault.balance - amount;
```

**Attack Scenario**: Attacker withdraws more than the balance, causing underflow that wraps to `u64::MAX`, creating unlimited funds.

### 3. Unchecked Multiplication (Interest/Reward Calculation)
```rust
// VULNERABLE: Large balances * rates can overflow
let interest = vault.balance * rate / 10000;
vault.balance = vault.balance + interest;
```

**Attack Scenario**: With large balances, multiplication overflows to a small number, either stealing expected interest or causing unexpected behavior.

### 4. Batch Operations (Accumulated Risk)
```rust
// VULNERABLE: Multiple operations increase overflow probability
for amount in amounts {
    vault.balance = vault.balance + amount;
}
```

**Attack Scenario**: Even if individual amounts seem safe, their accumulation can cause overflow.

## Secure Implementation

### 1. Checked Arithmetic Operations
```rust
// SECURE: Use checked_add, returns None on overflow
vault.balance = vault.balance
    .checked_add(amount)
    .ok_or(ErrorCode::ArithmeticOverflow)?;
```

### 2. Pre-validation for Complex Operations
```rust
// SECURE: Validate intermediate results
let interest_numerator = vault.balance
    .checked_mul(rate_basis_points)
    .ok_or(ErrorCode::ArithmeticOverflow)?;
    
let interest = interest_numerator
    .checked_div(10000)
    .ok_or(ErrorCode::DivisionByZero)?;
```

### 3. Balance Validation Before Operations
```rust
// SECURE: Check sufficient funds before subtraction
if vault.balance < amount {
    return Err(ErrorCode::InsufficientFunds.into());
}

vault.balance = vault.balance
    .checked_sub(amount)
    .ok_or(ErrorCode::ArithmeticUnderflow)?;
```

### 4. Batch Operation Safety
```rust
// SECURE: Calculate total first, then validate
let mut total_amount = 0u64;
for amount in &amounts {
    total_amount = total_amount
        .checked_add(*amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
}

// Only apply if total is safe
vault.balance = vault.balance
    .checked_add(total_amount)
    .ok_or(ErrorCode::ArithmeticOverflow)?;
```

## Testing the Vulnerability

### Running the Tests
```bash
cd 04_arithmetic_overflow
anchor test
```

### Key Test Cases

1. **Overflow Exploit**: Demonstrates balance corruption through deposit overflow
2. **Underflow Exploit**: Shows how withdrawal underflow creates unlimited funds  
3. **Interest Overflow**: Proves multiplication overflow in reward calculations
4. **Batch Overflow**: Shows accumulated overflow in batch operations
5. **Secure Prevention**: Verifies all exploits are blocked by checked arithmetic
6. **Boundary Testing**: Tests behavior at `u64::MAX` and zero boundaries

## Prevention Checklist

### Development Phase
- [ ] Use `checked_add()`, `checked_sub()`, `checked_mul()`, `checked_div()` for all arithmetic
- [ ] Validate inputs and intermediate results in complex calculations
- [ ] Check sufficient balance before any subtraction operations
- [ ] Consider overflow risk in batch and loop operations
- [ ] Use appropriate data types (avoid unnecessary large types that increase overflow risk)

### Code Review Phase
- [ ] Search for all arithmetic operators (`+`, `-`, `*`, `/`)
- [ ] Verify each operation uses checked arithmetic or has overflow protection
- [ ] Check that error handling properly propagates arithmetic errors
- [ ] Validate that balance checks occur before state modifications
- [ ] Review compound operations for accumulated overflow risk

### Testing Phase
- [ ] Test with maximum values (`u64::MAX`, `u64::MAX - 1`)
- [ ] Test with minimum values (`0`, `1`)
- [ ] Test boundary conditions where overflow/underflow is most likely
- [ ] Verify error messages are clear and appropriate
- [ ] Test legitimate operations still work correctly

## Compilation Considerations

### Debug vs Release Mode
```toml
# In Cargo.toml - Enable overflow checks in release mode
[profile.release]
overflow-checks = true
```

**Important**: Rust's default behavior differs between debug and release:
- **Debug mode**: Panics on overflow (helps catch bugs during development)
- **Release mode**: Wraps silently (can hide vulnerabilities in production)

Always enable `overflow-checks = true` in release profiles for Solana programs.

## Advanced Considerations

### Gas Optimization vs Security
While checked arithmetic has a small performance cost, the security benefit far outweighs the minimal gas increase. Never sacrifice security for micro-optimizations.

### Custom Error Types
```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("Arithmetic underflow occurred")]  
    ArithmeticUnderflow,
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
}
```

Provide clear, specific error messages that help with debugging while not revealing sensitive information to attackers.

### Integration with Anchor Constraints
```rust
#[derive(Accounts)]
pub struct SecureDeposit<'info> {
    #[account(mut, constraint = vault.balance.checked_add(amount).is_some())]
    pub vault: Account<'info, ArithmeticVault>,
    pub depositor: Signer<'info>,
}
```

Consider using Anchor constraints for additional validation layers, though explicit checks in instruction logic are often clearer.

## Summary

Arithmetic overflow/underflow vulnerabilities are among the most critical security issues in Solana programs. They can lead to:

- **Fund theft** through balance manipulation
- **Unlimited fund creation** through underflow exploits  
- **State corruption** that breaks program invariants
- **Compound effects** that amplify other vulnerabilities

**Key Takeaway**: Always use checked arithmetic operations (`checked_add`, `checked_sub`, `checked_mul`, `checked_div`) and validate intermediate results in complex calculations. The small performance cost is negligible compared to the catastrophic risk of arithmetic vulnerabilities.

Remember: In blockchain applications, mathematical correctness isn't just about program logic—it's about financial security and user trust.