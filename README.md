# Solana Security Reference

A comprehensive educational repository demonstrating common Solana security vulnerabilities through practical, executable examples. Learn to identify, understand, and prevent critical security issues in Solana programs.

## 🚨 Educational Purpose Only

**WARNING**: This repository contains intentionally vulnerable code for educational purposes. Never use vulnerable examples in production environments.

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18.0+)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) (v0.29.0+)
- [Node.js](https://nodejs.org/) (v16+)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/solana-security-reference.git
cd solana-security-reference

# Install dependencies
npm install

# Set up Solana for local development
solana config set --url localhost
solana-keygen new --no-bip39-passphrase
```

### Running Examples

Each vulnerability example is self-contained. To run an example:

```bash
# Navigate to any example directory
cd 01_missing_account_validation

# Build the program
anchor build

# Run tests (including exploit demonstrations)
anchor test
```

## 🔍 Vulnerability Examples

This repository covers the five most critical Solana security vulnerabilities:

### 1. Missing Account Validation
**Severity**: Critical | **Directory**: `01_missing_account_validation/`

Learn how missing account relationship validation can allow unauthorized access to user funds.

- **Vulnerable Pattern**: Programs that don't verify account ownership or relationships
- **Real-world Impact**: Multiple DeFi protocols exploited for millions
- **Fix**: Proper use of Anchor constraints (`has_one`, `constraint`)

### 2. Authority Check Failure  
**Severity**: Critical | **Directory**: `02_authority_check_failure/`

Understand the difference between signature verification and authority validation.

- **Vulnerable Pattern**: Checking if account is a signer vs. checking if signer has authority
- **Real-world Impact**: Admin function bypasses leading to protocol takeovers
- **Fix**: Explicit authority validation in addition to signature checks

### 3. Unsafe Cross-Program Invocation (CPI)
**Severity**: High | **Directory**: `03_unsafe_cpi/`

Discover how malicious programs can be substituted in CPI calls.

- **Vulnerable Pattern**: CPI calls without program ID validation
- **Real-world Impact**: Token program substitution attacks
- **Fix**: Explicit program ID constraints and validation

### 4. Arithmetic Overflow/Underflow
**Severity**: High | **Directory**: `04_arithmetic_overflow/`

See how integer overflow can corrupt program state and balances.

- **Vulnerable Pattern**: Unchecked arithmetic operations
- **Real-world Impact**: Balance manipulation and infinite minting
- **Fix**: Checked arithmetic operations (`checked_add`, `checked_sub`, etc.)

### 5. Reinitialization Attack
**Severity**: Medium | **Directory**: `05_reinitialization_attack/`

Learn how attackers can reset program state by reinitializing accounts.

- **Vulnerable Pattern**: Missing initialization state checks
- **Real-world Impact**: Account state reset and fund drainage
- **Fix**: Proper initialization protection with Anchor's `init` constraint

## 📚 Documentation Structure

- **README.md** (this file) - Project overview and quick start
- **[DEEP_DIVE.md](./DEEP_DIVE.md)** - Comprehensive technical analysis (3000+ words)
- **Each example directory contains**:
  - `README.md` - Vulnerability-specific guide
  - `EXPLOIT.md` - Step-by-step attack walkthrough
  - `programs/vault/src/lib.rs` - Vulnerable and secure implementations
  - `tests/exploit.test.ts` - Automated exploit demonstrations

## 🧪 Testing Philosophy

Each example includes three types of tests:

1. **Exploit Tests** - Demonstrate successful attacks on vulnerable code
2. **Protection Tests** - Verify that secure implementations block attacks  
3. **Legitimate Usage Tests** - Confirm normal operations work correctly

Run all tests across examples:
```bash
npm test
```

## 🔧 Framework Comparison

This repository primarily uses the Anchor framework for its declarative security features. For advanced users, the `bonus_pinocchio_comparison/` directory shows the same vulnerabilities implemented in the Pinocchio framework, highlighting the trade-offs between explicit control and safety.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for:

- How to add new vulnerability examples
- Code style and documentation standards
- Testing requirements
- Review process

## 📖 Learning Path

**Recommended order for beginners**:

1. Start with `01_missing_account_validation` - most common and critical
2. Progress through `02_authority_check_failure` - builds on account concepts
3. Continue with `03_unsafe_cpi` - introduces cross-program security
4. Study `04_arithmetic_overflow` - covers state corruption
5. Finish with `05_reinitialization_attack` - advanced state management

**For security auditors**: Read [DEEP_DIVE.md](./DEEP_DIVE.md) first for systematic review methodology.

## ⚡ Performance

- Complete repository build: < 2 minutes
- Individual example tests: < 1 minute  
- Full test suite: < 5 minutes

## 🔗 Additional Resources

- [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security)
- [Anchor Security Guidelines](https://www.anchor-lang.com/docs/security)
- [Solana Program Security Reviews](https://github.com/coral-xyz/sealevel-attacks)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This repository is for educational purposes only. The authors are not responsible for any misuse of the information or code provided. Always conduct thorough security audits before deploying any Solana program to mainnet.

---

**Happy Learning! 🚀**

*Remember: The best defense is understanding how attacks work.*