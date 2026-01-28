# Contributing to Solana Security Reference

Thank you for your interest in contributing to the Solana Security Reference! This guide will help you add new vulnerability examples, improve existing content, and maintain the high quality standards of this educational resource.

## 🎯 Contribution Types

We welcome several types of contributions:

- **New vulnerability examples** - Additional security patterns not currently covered
- **Improvements to existing examples** - Better code, tests, or documentation
- **Documentation enhancements** - Clearer explanations, additional context
- **Framework comparisons** - Examples using different Solana frameworks
- **Bug fixes** - Corrections to code or documentation errors

## 📋 Prerequisites

Before contributing, ensure you have:

- Strong understanding of Solana programming concepts
- Experience with Rust and the Anchor framework
- Familiarity with common security vulnerabilities
- Ability to write clear, educational documentation

## 🏗️ Adding a New Vulnerability Example

### Step 1: Choose a Vulnerability Pattern

New examples should:
- Address a real, documented security issue in Solana programs
- Have historical precedent (real-world exploits preferred)
- Be distinct from existing examples
- Be teachable with clear vulnerable vs. secure implementations

### Step 2: Create Directory Structure

Create a new directory following the naming convention:
```
XX_vulnerability_name/
├── README.md                    # Vulnerability overview and guide
├── EXPLOIT.md                   # Step-by-step attack walkthrough  
├── Anchor.toml                  # Anchor project configuration
├── Cargo.toml                   # Rust project configuration
├── package.json                 # Node.js dependencies for tests
├── programs/
│   └── vault/
│       ├── Cargo.toml          # Program-specific Cargo config
│       └── src/
│           └── lib.rs          # Vulnerable + secure implementations
└── tests/
    └── exploit.test.ts         # Automated exploit demonstrations
```

### Step 3: Implement Code Examples

#### lib.rs Structure
Your `lib.rs` file must contain:

```rust
use anchor_lang::prelude::*;

declare_id!("YourProgramIdHere");

#[program]
pub mod vault {
    use super::*;
    
    // VULNERABLE IMPLEMENTATION
    // Clear comments explaining the security flaw
    pub fn vulnerable_function(ctx: Context<VulnerableContext>) -> Result<()> {
        // Implementation with security vulnerability
        // Detailed comments explaining why this is dangerous
        Ok(())
    }
    
    // SECURE IMPLEMENTATION  
    // Clear comments explaining the security fix
    pub fn secure_function(ctx: Context<SecureContext>) -> Result<()> {
        // Fixed implementation with proper security measures
        // Detailed comments explaining the protection mechanism
        Ok(())
    }
}

// Account structures for vulnerable version
#[derive(Accounts)]
pub struct VulnerableContext<'info> {
    // Account definitions that allow the vulnerability
}

// Account structures for secure version
#[derive(Accounts)]  
pub struct SecureContext<'info> {
    // Account definitions with proper security constraints
}

// Shared account structures
#[account]
pub struct Vault {
    // Common data structures used by both versions
}

// Custom error types
#[error_code]
pub enum VaultError {
    // Security-related error definitions
}
```

#### Test Structure
Your `exploit.test.ts` file must include:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("vulnerability_name", () => {
  // Test setup
  
  describe("Exploit Demonstration", () => {
    it("successfully exploits vulnerable implementation", async () => {
      // Test that demonstrates the vulnerability
      // Should succeed and show the security flaw
    });
  });
  
  describe("Security Fix Verification", () => {
    it("blocks exploit attempt on secure implementation", async () => {
      // Test that shows the fix prevents the exploit
      // Should fail with appropriate error message
    });
  });
  
  describe("Legitimate Usage", () => {
    it("allows normal operations on secure implementation", async () => {
      // Test that confirms normal functionality works
      // Should succeed for legitimate use cases
    });
  });
});
```

### Step 4: Write Documentation

#### README.md Template
```markdown
# [Vulnerability Name]

## Overview
Brief description of the vulnerability and its impact.

## Vulnerability Details
- **Severity**: Critical/High/Medium/Low
- **Category**: [e.g., Account Validation, Authority, CPI, etc.]
- **Historical Impact**: Real-world examples and dollar amounts

## The Problem
Detailed explanation of what makes code vulnerable.

## The Solution  
Explanation of how to fix the vulnerability.

## Code Examples

### Vulnerable Implementation
```rust
// Code snippet showing the vulnerability
```

### Secure Implementation  
```rust
// Code snippet showing the fix
```

## Running the Example
```bash
anchor build
anchor test
```

## Key Takeaways
- Main lessons learned
- Best practices to follow
- Common mistakes to avoid
```

#### EXPLOIT.md Template
```markdown
# [Vulnerability Name] - Attack Walkthrough

## Attack Scenario
Step-by-step description of how an attacker would exploit this vulnerability.

## Prerequisites
What an attacker needs to execute this exploit.

## Attack Steps
1. **Step 1**: Detailed description
2. **Step 2**: Detailed description  
3. **Step 3**: Detailed description

## Impact
What the attacker achieves and potential damage.

## Real-World Examples
Historical incidents using this attack pattern.

## Detection
How to identify this vulnerability in code reviews.

## Prevention
Specific measures to prevent this attack.
```

### Step 5: Update Configuration Files

#### Anchor.toml
```toml
[features]
seeds = false
skip-lint = false

[programs.localnet]
vault = "YourProgramIdHere"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "Localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

#### Cargo.toml (Program)
```toml
[package]
name = "vault"
version = "0.1.0"
description = "Vulnerability example: [Description]"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]
name = "vault"

[dependencies]
anchor-lang = "0.29.0"
```

## 🧪 Testing Requirements

### Test Coverage
All examples must include:
- **Exploit test** - Proves vulnerability exists
- **Protection test** - Proves fix works  
- **Legitimate usage test** - Confirms normal operations

### Test Quality Standards
- Tests should run in under 1 minute
- Clear, descriptive test names
- Comprehensive assertions
- Proper error message validation
- No flaky or intermittent failures

### Running Tests
```bash
# Individual example
cd XX_vulnerability_name
anchor test

# All examples  
npm test

# CI/CD validation
npm run ci
```

## 📝 Documentation Standards

### Writing Style
- **Clear and educational** - Assume readers are learning
- **Practical focus** - Emphasize real-world applicability  
- **Security-first** - Always explain the security implications
- **Code-heavy** - Show, don't just tell

### Required Sections
Every example must have:
- Vulnerability overview and severity assessment
- Historical context and real-world impact
- Clear code examples (vulnerable vs. secure)
- Step-by-step exploit walkthrough
- Prevention and detection guidance

### Code Comments
- **Vulnerable code** - Explain why it's dangerous
- **Secure code** - Explain how the fix works
- **Account structures** - Document security constraints
- **Error handling** - Explain security error conditions

## 🔍 Review Process

### Submission Checklist
Before submitting a pull request:

- [ ] Code compiles without warnings
- [ ] All tests pass consistently  
- [ ] Documentation is complete and accurate
- [ ] Follows established naming conventions
- [ ] Includes both vulnerable and secure implementations
- [ ] Has comprehensive test coverage
- [ ] Vulnerability has real-world precedent
- [ ] Code includes detailed security comments

### Review Criteria
Contributions are evaluated on:

1. **Educational Value** - Does it teach important security concepts?
2. **Code Quality** - Is the implementation clear and correct?
3. **Documentation Quality** - Are explanations clear and comprehensive?
4. **Test Coverage** - Do tests adequately demonstrate the concepts?
5. **Uniqueness** - Does it add new value not covered elsewhere?

### Review Process
1. **Automated Checks** - CI/CD pipeline validates compilation and tests
2. **Technical Review** - Maintainers review code and security concepts
3. **Documentation Review** - Verify clarity and educational value
4. **Integration Testing** - Ensure compatibility with existing examples

## 🚀 Advanced Contributions

### Framework Comparisons
We welcome examples showing the same vulnerability in different frameworks:
- Anchor vs. Pinocchio implementations
- Native Solana program comparisons
- Framework-specific security trade-offs

### Tooling Improvements
- Better testing utilities
- Documentation generation tools
- Security analysis automation
- CI/CD enhancements

### Educational Enhancements
- Visual diagrams and flowcharts
- Interactive tutorials
- Video content integration
- Workshop materials

## 📞 Getting Help

### Questions and Discussion
- **GitHub Issues** - For bugs, feature requests, and questions
- **GitHub Discussions** - For general discussion and ideas
- **Discord** - Real-time chat with maintainers and contributors

### Maintainer Contact
- Create an issue for technical questions
- Tag maintainers in pull requests for review
- Use discussions for broader architectural questions

## 🏆 Recognition

Contributors are recognized through:
- **Contributors section** in README.md
- **Author attribution** in example documentation  
- **Maintainer status** for significant ongoing contributions
- **Community recognition** in project announcements

## 📄 Legal

By contributing, you agree that:
- Your contributions will be licensed under the project's MIT license
- You have the right to submit the contribution
- Your contribution is original work or properly attributed
- You understand this is educational content with security implications

---

**Thank you for helping make Solana development more secure! 🛡️**

*Every contribution helps developers build safer applications.*