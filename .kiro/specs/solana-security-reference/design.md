# Design Document

## Overview

The Solana Security Reference is designed as a comprehensive educational repository that demonstrates common Solana security vulnerabilities through practical, executable examples. The system follows a modular architecture where each vulnerability is self-contained with its own code, tests, and documentation, while sharing common patterns and infrastructure.

The design prioritizes hands-on learning through side-by-side code comparisons, automated exploit demonstrations, and comprehensive documentation that bridges the gap between theoretical security knowledge and practical implementation skills.

## Architecture

### Repository Structure

```
solana-security-reference/
├── README.md                          # Main entry point with quick start
├── DEEP_DIVE.md                       # Comprehensive technical analysis (3000+ words)
├── CONTRIBUTING.md                    # Contribution guidelines and templates
├── .github/workflows/test.yml         # CI/CD pipeline for automated testing
├── 01_missing_account_validation/     # Self-contained vulnerability example
├── 02_authority_check_failure/        # Self-contained vulnerability example
├── 03_unsafe_cpi/                     # Self-contained vulnerability example
├── 04_arithmetic_overflow/            # Self-contained vulnerability example
├── 05_reinitialization_attack/        # Self-contained vulnerability example
└── bonus_pinocchio_comparison/        # Framework comparison examples
```

### Modular Example Architecture

Each vulnerability example follows a consistent structure:

```
XX_vulnerability_name/
├── README.md                          # Vulnerability-specific guide
├── EXPLOIT.md                         # Step-by-step attack walkthrough
├── Anchor.toml                        # Anchor project configuration
├── Cargo.toml                         # Rust project configuration
├── programs/vault/src/lib.rs          # Vulnerable + secure implementations
└── tests/exploit.test.ts              # Automated exploit and fix tests
```

### Code Organization Pattern

Each `lib.rs` file contains:
1. **Vulnerable Implementation Section** - Clearly marked with security issues
2. **Secure Implementation Section** - Fixed version with explanations
3. **Account Structures** - Both vulnerable and secure variants
4. **State Definitions** - Shared data structures
5. **Error Definitions** - Custom error types for security violations

## Components and Interfaces

### Core Components

#### 1. Vulnerability Example Module
**Purpose**: Self-contained demonstration of a specific security vulnerability

**Interface**:
- `vulnerable_function()` - Demonstrates the security flaw
- `secure_function()` - Shows the corrected implementation
- Account validation structures for both versions
- Comprehensive inline documentation

**Dependencies**:
- Anchor framework for declarative security
- Solana program library for system interactions

#### 2. Test Suite Module
**Purpose**: Automated demonstration of exploits and fixes

**Interface**:
- `exploit_test()` - Demonstrates successful attack on vulnerable code
- `protection_test()` - Verifies fix prevents the attack
- `legitimate_usage_test()` - Confirms normal operations still work

**Dependencies**:
- Anchor testing framework
- Chai assertion library for test validation

#### 3. Documentation System
**Purpose**: Comprehensive educational content

**Components**:
- Main README with quick start and overview
- Per-example README with vulnerability details
- EXPLOIT.md files with attack scenarios
- DEEP_DIVE.md with technical analysis

#### 4. CI/CD Pipeline
**Purpose**: Automated quality assurance and testing

**Interface**:
- Automated compilation verification
- Test execution across all examples
- Documentation completeness checks
- Code quality validation

### Framework Integration

#### Anchor Framework Integration
- Utilizes Anchor's constraint system for declarative security
- Leverages account validation macros (`has_one`, `constraint`)
- Uses Anchor's initialization protection mechanisms
- Implements proper error handling with custom error types

#### Pinocchio Framework Integration (Bonus)
- Demonstrates explicit, low-level security validation
- Shows manual account relationship verification
- Illustrates performance vs. safety trade-offs
- Provides direct comparison with Anchor approaches

## Data Models

### Vulnerability Example Data Structure

```rust
#[account]
pub struct Vault {
    pub owner: Pubkey,      // Authorized owner of the vault
    pub balance: u64,       // Tracked balance in lamports
    pub admin: Pubkey,      // Administrative authority (where applicable)
    pub initialized: bool,  // Initialization state tracking
}
```

### Test Case Data Structure

```typescript
interface TestScenario {
    name: string;                    // Test case description
    expectation: 'success' | 'failure'; // Expected outcome
    accounts: AccountSetup;          // Account configuration
    parameters: InstructionParams;   // Function parameters
    validation: ValidationChecks;    // Post-execution validation
}
```

### Documentation Metadata

```markdown
# Per-example metadata structure
- Severity: Critical | High | Medium | Low
- Difficulty: Easy | Medium | Hard
- Historical Impact: Dollar amount and notable incidents
- Framework Compatibility: Anchor | Pinocchio | Both
- Prerequisites: Required knowledge for understanding
```

## Error Handling

### Security Error Categories

#### 1. Account Validation Errors
- `UnauthorizedOwner` - Account relationship validation failure
- `InvalidAccountData` - Account data integrity issues
- `MissingRequiredSignature` - Signature validation failure

#### 2. Arithmetic Errors
- `Overflow` - Integer overflow detection
- `Underflow` - Integer underflow detection
- `DivisionByZero` - Division safety checks

#### 3. CPI Security Errors
- `InvalidProgramId` - Program ID validation failure
- `UnauthorizedCPI` - Cross-program invocation security violation

#### 4. Initialization Errors
- `AlreadyInitialized` - Reinitialization attempt prevention
- `NotInitialized` - Uninitialized account access prevention

### Error Handling Strategy

1. **Fail Fast Principle**: Security violations immediately terminate execution
2. **Clear Error Messages**: Each error provides specific context about the security violation
3. **Consistent Error Codes**: Standardized error types across all examples
4. **Educational Error Context**: Error messages explain why the security check exists

## Testing Strategy

### Test Categories

#### 1. Exploit Demonstration Tests
**Purpose**: Prove that vulnerable code can be exploited
- Execute attack scenarios against vulnerable implementations
- Verify that exploits succeed and cause intended damage
- Document attack vectors and success conditions

#### 2. Security Fix Verification Tests
**Purpose**: Confirm that secure implementations prevent exploits
- Execute same attack scenarios against secure implementations
- Verify that security measures block malicious attempts
- Confirm appropriate error messages are returned

#### 3. Legitimate Usage Tests
**Purpose**: Ensure security fixes don't break normal functionality
- Test normal user workflows with secure implementations
- Verify that legitimate operations complete successfully
- Confirm performance impact is acceptable

#### 4. Edge Case Tests
**Purpose**: Validate security under boundary conditions
- Test with maximum/minimum values
- Verify behavior with unusual but valid inputs
- Confirm security holds under stress conditions

### Test Execution Strategy

#### Automated Testing Pipeline
1. **Compilation Verification**: All code compiles without warnings
2. **Unit Test Execution**: Individual function security tests
3. **Integration Testing**: End-to-end exploit scenarios
4. **Performance Validation**: Ensure tests complete within time limits
5. **Documentation Sync**: Verify code matches documentation

#### Manual Testing Procedures
1. **Security Review Checklist**: Systematic vulnerability assessment
2. **Code Quality Review**: Adherence to security best practices
3. **Documentation Review**: Accuracy and completeness verification
4. **User Experience Testing**: Clarity and educational value assessment

### Test Data Management

#### Test Account Generation
- Deterministic keypair generation for reproducible tests
- Separate account sets for legitimate users and attackers
- Proper cleanup and isolation between test cases

#### Test Environment Setup
- Local validator configuration for consistent testing
- Airdrop management for test account funding
- Program deployment and initialization procedures

## Implementation Phases

### Phase 1: Foundation (Day 1)
1. Repository structure setup
2. CI/CD pipeline configuration
3. First two vulnerability examples (Missing Account Validation, Authority Check Failure)
4. Basic documentation framework

### Phase 2: Core Examples (Day 2)
1. Remaining three vulnerability examples (Unsafe CPI, Arithmetic Overflow, Reinitialization)
2. Comprehensive test suite completion
3. DEEP_DIVE.md technical documentation
4. End-to-end testing and validation

### Phase 3: Polish and Bonus Features (Day 3)
1. Pinocchio framework comparison implementation
2. Advanced documentation features (diagrams, visual aids)
3. Final quality assurance and testing
4. Submission preparation and review

## Security Considerations

### Code Security
- All example code undergoes security review
- Vulnerable examples are clearly marked and isolated
- Secure implementations follow industry best practices
- No production secrets or sensitive data in repository

### Educational Safety
- Clear warnings about vulnerable code usage
- Explicit guidance on when and how to use examples
- Proper context for learning vs. production environments
- Responsible disclosure principles for new vulnerabilities

### Repository Security
- Automated security scanning in CI/CD pipeline
- Dependency vulnerability monitoring
- Code signing and integrity verification
- Access control for repository modifications

## Performance Requirements

### Build Performance
- Complete repository build in under 2 minutes
- Individual example compilation in under 30 seconds
- Parallel build support for multiple examples

### Test Performance
- Full test suite execution in under 5 minutes
- Individual example tests in under 1 minute
- Automated timeout handling for long-running tests

### Documentation Performance
- Fast navigation between examples and sections
- Efficient search and discovery mechanisms
- Responsive design for various devices and screen sizes

## Maintenance and Evolution

### Content Updates
- Regular review of vulnerability examples for accuracy
- Updates to reflect new Solana features and security patterns
- Community contribution integration and review process

### Framework Compatibility
- Tracking of Anchor and Pinocchio framework updates
- Compatibility testing with new framework versions
- Migration guides for breaking changes

### Educational Effectiveness
- User feedback collection and analysis
- Learning outcome measurement and improvement
- Content optimization based on usage patterns