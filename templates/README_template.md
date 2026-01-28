# [Vulnerability Name]

## Overview
[Brief description of the vulnerability and its impact]

## Vulnerability Details
- **Severity**: [Critical/High/Medium/Low]
- **Category**: [e.g., Account Validation, Authority, CPI, Arithmetic, Initialization]
- **Historical Impact**: [Real-world examples and dollar amounts if available]
- **Difficulty**: [Easy/Medium/Hard to exploit]

## The Problem
[Detailed explanation of what makes code vulnerable. Include:]
- Root cause of the vulnerability
- Why developers commonly make this mistake
- What attackers can achieve by exploiting it

## The Solution  
[Explanation of how to fix the vulnerability. Include:]
- Specific code changes required
- Security mechanisms that prevent the exploit
- Best practices to follow

## Code Examples

### Vulnerable Implementation
```rust
// Show the problematic code pattern
// Include detailed comments explaining the vulnerability
```

### Secure Implementation  
```rust
// Show the fixed code pattern
// Include detailed comments explaining the security measures
```

## Running the Example
```bash
# Build the program
anchor build

# Run all tests (exploit + protection + legitimate usage)
anchor test

# Run specific test categories
anchor test --grep "Exploit"
anchor test --grep "Protection" 
anchor test --grep "Legitimate"
```

## Test Results
[Describe what users should expect to see when running tests:]
- Exploit tests should succeed on vulnerable code
- Protection tests should fail on vulnerable code but succeed on secure code
- Legitimate usage tests should succeed on secure code

## Key Takeaways
- [Main lesson 1]
- [Main lesson 2] 
- [Main lesson 3]

## Common Mistakes to Avoid
- [Mistake 1 and why it's problematic]
- [Mistake 2 and why it's problematic]
- [Mistake 3 and why it's problematic]

## Additional Resources
- [Link to relevant Solana documentation]
- [Link to Anchor security guidelines]
- [Link to related security research or blog posts]

## Real-World Context
[If applicable, include information about:]
- Historical exploits using this pattern
- Dollar amounts lost to this vulnerability type
- Notable protocols that were affected
- Timeline of when this vulnerability was discovered/publicized