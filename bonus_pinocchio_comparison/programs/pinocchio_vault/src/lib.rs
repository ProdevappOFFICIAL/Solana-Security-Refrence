use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use std::mem;

// Program ID - same as Anchor version for comparison
pinocchio::declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Entry point for the Solana program
entrypoint!(process_instruction);

// ========================================
// INSTRUCTION DISCRIMINATORS
// ========================================
// Pinocchio requires manual instruction discrimination
// Unlike Anchor which handles this automatically

const VULNERABLE_INITIALIZE: u8 = 0;
const VULNERABLE_DEPOSIT: u8 = 1;
const VULNERABLE_WITHDRAW: u8 = 2;
const SECURE_INITIALIZE: u8 = 3;
const SECURE_DEPOSIT: u8 = 4;
const SECURE_WITHDRAW: u8 = 5;

// ========================================
// MAIN INSTRUCTION PROCESSOR
// ========================================
// COMPARISON: Anchor automatically routes instructions to handler functions
// Pinocchio requires manual instruction parsing and routing

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Manual instruction discrimination - Anchor does this automatically
    match instruction_data[0] {
        VULNERABLE_INITIALIZE => vulnerable_initialize(program_id, accounts, &instruction_data[1..]),
        VULNERABLE_DEPOSIT => vulnerable_deposit(program_id, accounts, &instruction_data[1..]),
        VULNERABLE_WITHDRAW => vulnerable_withdraw(program_id, accounts, &instruction_data[1..]),
        SECURE_INITIALIZE => secure_initialize(program_id, accounts, &instruction_data[1..]),
        SECURE_DEPOSIT => secure_deposit(program_id, accounts, &instruction_data[1..]),
        SECURE_WITHDRAW => secure_withdraw(program_id, accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// ========================================
// VULNERABLE IMPLEMENTATION
// ========================================
// COMPARISON: These functions show the same vulnerabilities as the Anchor version
// but with explicit, manual validation (or lack thereof)

/// VULNERABLE: Initialize a new vault without proper validation
/// 
/// PINOCCHIO vs ANCHOR COMPARISON:
/// - Anchor: Uses declarative constraints like `#[account(init)]`
/// - Pinocchio: Requires explicit manual validation of all account properties
/// 
/// Security Issue: This function doesn't validate the relationship between
/// the vault account and the owner, just like the Anchor version.
/// However, in Pinocchio, ALL validation must be done manually.
fn vulnerable_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // PINOCCHIO: Manual account parsing - Anchor does this automatically
    let accounts_iter = &mut accounts.iter();
    let vault_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let payer_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let system_program = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    // PINOCCHIO: Manual instruction data parsing - Anchor does this automatically
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let initial_balance = u64::from_le_bytes([
        instruction_data[0], instruction_data[1], instruction_data[2], instruction_data[3],
        instruction_data[4], instruction_data[5], instruction_data[6], instruction_data[7],
    ]);

    // VULNERABILITY: Same as Anchor version - no validation that the signer
    // is authorized to create a vault for the specified owner
    // PINOCCHIO: We must manually check account ownership, size, etc.
    if vault_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // VULNERABILITY: No validation that the vault is uninitialized
    // PINOCCHIO: Must manually check account data state
    let mut vault_data = vault_account.try_borrow_mut_data()?;
    if vault_data.len() < mem::size_of::<Vault>() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    // VULNERABILITY: Setting owner without validating authorization
    // PINOCCHIO: Manual serialization - Anchor handles this automatically
    let vault = Vault {
        owner: *owner_account.key,
        balance: initial_balance,
    };
    
    // Manual serialization to account data
    vault_data[0..32].copy_from_slice(&vault.owner.to_bytes());
    vault_data[32..40].copy_from_slice(&vault.balance.to_le_bytes());

    msg!("Vulnerable vault initialized with owner: {} and balance: {}", vault.owner, vault.balance);
    Ok(())
}

/// VULNERABLE: Deposit funds without proper account validation
/// 
/// PINOCCHIO vs ANCHOR COMPARISON:
/// - Anchor: Uses `#[account(mut)]` and automatic deserialization
/// - Pinocchio: Requires manual account validation and data parsing
fn vulnerable_deposit(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // PINOCCHIO: Manual account parsing
    let accounts_iter = &mut accounts.iter();
    let vault_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let _owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    // PINOCCHIO: Manual instruction data parsing
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes([
        instruction_data[0], instruction_data[1], instruction_data[2], instruction_data[3],
        instruction_data[4], instruction_data[5], instruction_data[6], instruction_data[7],
    ]);

    // PINOCCHIO: Manual data deserialization
    let mut vault_data = vault_account.try_borrow_mut_data()?;
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // VULNERABILITY: Same as Anchor version - no validation that vault.owner
    // matches the owner account. In Pinocchio, this validation must be explicit.
    vault.balance = vault.balance.checked_add(amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    // PINOCCHIO: Manual serialization back to account
    vault_data[32..40].copy_from_slice(&vault.balance.to_le_bytes());

    msg!("Deposited {} to vault. New balance: {}", amount, vault.balance);
    Ok(())
}

/// VULNERABLE: Withdraw funds without proper owner validation
/// 
/// PINOCCHIO vs ANCHOR COMPARISON:
/// - Anchor: Uses `pub owner: Signer<'info>` for automatic signer validation
/// - Pinocchio: Must manually check `is_signer` property
fn vulnerable_withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // PINOCCHIO: Manual account parsing
    let accounts_iter = &mut accounts.iter();
    let vault_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    // PINOCCHIO: Manual signer validation - Anchor does this automatically
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // PINOCCHIO: Manual instruction data parsing
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes([
        instruction_data[0], instruction_data[1], instruction_data[2], instruction_data[3],
        instruction_data[4], instruction_data[5], instruction_data[6], instruction_data[7],
    ]);

    // PINOCCHIO: Manual data deserialization
    let mut vault_data = vault_account.try_borrow_mut_data()?;
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // VULNERABILITY: Same critical flaw as Anchor version!
    // We check that someone signed, but NOT that the signer is the vault owner
    // PINOCCHIO: This validation must be explicit - we forgot to add it!
    if vault.balance < amount {
        return Err(VaultError::InsufficientFunds.into());
    }

    vault.balance = vault.balance.checked_sub(amount)
        .ok_or(VaultError::ArithmeticUnderflow)?;

    // PINOCCHIO: Manual serialization back to account
    vault_data[32..40].copy_from_slice(&vault.balance.to_le_bytes());

    msg!("Withdrew {} from vault. New balance: {}", amount, vault.balance);
    Ok(())
}

// ========================================
// SECURE IMPLEMENTATION
// ========================================
// COMPARISON: These functions show how to properly implement security
// in Pinocchio with explicit validation vs Anchor's declarative approach

/// SECURE: Initialize a new vault with proper validation
/// 
/// PINOCCHIO vs ANCHOR COMPARISON:
/// - Anchor: `#[account(init, has_one = owner)]` handles validation declaratively
/// - Pinocchio: Must explicitly validate account state, ownership, and relationships
fn secure_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // PINOCCHIO: Manual account parsing
    let accounts_iter = &mut accounts.iter();
    let vault_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let payer_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let system_program = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    // SECURITY: Explicit signer validation - Anchor's `Signer<'info>` does this automatically
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // PINOCCHIO: Manual instruction data parsing
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let initial_balance = u64::from_le_bytes([
        instruction_data[0], instruction_data[1], instruction_data[2], instruction_data[3],
        instruction_data[4], instruction_data[5], instruction_data[6], instruction_data[7],
    ]);

    // SECURITY: Explicit program ownership validation
    // PINOCCHIO: Must manually verify account is owned by our program
    if vault_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // SECURITY: Explicit initialization check - Anchor's `init` does this automatically
    // PINOCCHIO: Must manually verify account is uninitialized
    let vault_data = vault_account.try_borrow_data()?;
    if vault_data.len() < mem::size_of::<Vault>() {
        return Err(ProgramError::AccountDataTooSmall);
    }
    
    // Check if already initialized by looking at the data
    let existing_owner = Pubkey::try_from(&vault_data[0..32])
        .map_err(|_| ProgramError::InvalidAccountData)?;
    if existing_owner != Pubkey::default() {
        return Err(VaultError::AlreadyInitialized.into());
    }
    drop(vault_data);

    // SECURITY: Now we can safely initialize with validated owner
    let mut vault_data = vault_account.try_borrow_mut_data()?;
    let vault = Vault {
        owner: *owner_account.key,  // This is now validated as a signer
        balance: initial_balance,
    };
    
    // Manual serialization to account data
    vault_data[0..32].copy_from_slice(&vault.owner.to_bytes());
    vault_data[32..40].copy_from_slice(&vault.balance.to_le_bytes());

    msg!("Secure vault initialized with owner: {} and balance: {}", vault.owner, vault.balance);
    Ok(())
}

/// SECURE: Deposit funds with proper account validation
/// 
/// PINOCCHIO vs ANCHOR COMPARISON:
/// - Anchor: `#[account(mut, has_one = owner)]` validates relationship declaratively
/// - Pinocchio: Must explicitly validate vault.owner == owner_account.key
fn secure_deposit(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // PINOCCHIO: Manual account parsing
    let accounts_iter = &mut accounts.iter();
    let vault_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    // SECURITY: Explicit signer validation
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // PINOCCHIO: Manual instruction data parsing
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes([
        instruction_data[0], instruction_data[1], instruction_data[2], instruction_data[3],
        instruction_data[4], instruction_data[5], instruction_data[6], instruction_data[7],
    ]);

    // PINOCCHIO: Manual data deserialization
    let mut vault_data = vault_account.try_borrow_mut_data()?;
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // SECURITY: Explicit ownership validation - this is the key fix!
    // PINOCCHIO: Must manually implement what Anchor's `has_one = owner` does
    if vault.owner != *owner_account.key {
        return Err(VaultError::UnauthorizedOwner.into());
    }

    vault.balance = vault.balance.checked_add(amount)
        .ok_or(VaultError::ArithmeticOverflow)?;

    // PINOCCHIO: Manual serialization back to account
    vault_data[32..40].copy_from_slice(&vault.balance.to_le_bytes());

    msg!("Securely deposited {} to vault. New balance: {}", amount, vault.balance);
    Ok(())
}

/// SECURE: Withdraw funds with proper owner validation
/// 
/// PINOCCHIO vs ANCHOR COMPARISON:
/// - Anchor: `#[account(mut, has_one = owner)]` + `pub owner: Signer<'info>`
/// - Pinocchio: Must manually validate both signer status AND ownership relationship
fn secure_withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // PINOCCHIO: Manual account parsing
    let accounts_iter = &mut accounts.iter();
    let vault_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    let owner_account = accounts_iter.next().ok_or(ProgramError::NotEnoughAccountKeys)?;

    // SECURITY: Explicit signer validation
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // PINOCCHIO: Manual instruction data parsing
    if instruction_data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes([
        instruction_data[0], instruction_data[1], instruction_data[2], instruction_data[3],
        instruction_data[4], instruction_data[5], instruction_data[6], instruction_data[7],
    ]);

    // PINOCCHIO: Manual data deserialization
    let mut vault_data = vault_account.try_borrow_mut_data()?;
    let mut vault = Vault::try_from_slice(&vault_data)?;

    // SECURITY: The critical fix - explicit ownership validation!
    // PINOCCHIO: Must manually implement what Anchor's `has_one = owner` does
    // This is the line that was missing in the vulnerable version
    if vault.owner != *owner_account.key {
        return Err(VaultError::UnauthorizedOwner.into());
    }

    // SECURITY: Balance check
    if vault.balance < amount {
        return Err(VaultError::InsufficientFunds.into());
    }

    vault.balance = vault.balance.checked_sub(amount)
        .ok_or(VaultError::ArithmeticUnderflow)?;

    // PINOCCHIO: Manual serialization back to account
    vault_data[32..40].copy_from_slice(&vault.balance.to_le_bytes());

    msg!("Securely withdrew {} from vault. New balance: {}", amount, vault.balance);
    Ok(())
}

// ========================================
// DATA STRUCTURES
// ========================================
// COMPARISON: Anchor uses `#[account]` macro for automatic serialization
// Pinocchio requires manual implementation of serialization traits

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vault {
    pub owner: Pubkey,    // 32 bytes
    pub balance: u64,     // 8 bytes
}

impl Vault {
    /// PINOCCHIO: Manual deserialization - Anchor does this automatically
    pub fn try_from_slice(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < mem::size_of::<Self>() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        let owner_bytes: [u8; 32] = data[0..32].try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let owner = Pubkey::from(owner_bytes);

        let balance_bytes: [u8; 8] = data[32..40].try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        let balance = u64::from_le_bytes(balance_bytes);

        Ok(Self { owner, balance })
    }
}

// ========================================
// ERROR DEFINITIONS
// ========================================
// COMPARISON: Anchor uses `#[error_code]` macro for automatic error handling
// Pinocchio requires manual error type implementation

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VaultError {
    InsufficientFunds,
    ArithmeticOverflow,
    ArithmeticUnderflow,
    UnauthorizedOwner,
    AlreadyInitialized,
}

impl From<VaultError> for ProgramError {
    fn from(error: VaultError) -> Self {
        match error {
            VaultError::InsufficientFunds => ProgramError::Custom(0),
            VaultError::ArithmeticOverflow => ProgramError::Custom(1),
            VaultError::ArithmeticUnderflow => ProgramError::Custom(2),
            VaultError::UnauthorizedOwner => ProgramError::Custom(3),
            VaultError::AlreadyInitialized => ProgramError::Custom(4),
        }
    }
}

// ========================================
// FRAMEWORK COMPARISON SUMMARY
// ========================================
/*
KEY DIFFERENCES BETWEEN ANCHOR AND PINOCCHIO:

1. INSTRUCTION HANDLING:
   - Anchor: Automatic instruction discrimination and routing via macros
   - Pinocchio: Manual instruction parsing and routing in process_instruction

2. ACCOUNT VALIDATION:
   - Anchor: Declarative constraints like `#[account(has_one = owner)]`
   - Pinocchio: Explicit validation code that must be written manually

3. DATA SERIALIZATION:
   - Anchor: Automatic serialization/deserialization with `#[account]` macro
   - Pinocchio: Manual implementation of serialization logic

4. ERROR HANDLING:
   - Anchor: `#[error_code]` macro generates error handling automatically
   - Pinocchio: Manual error type implementation and conversion

5. SECURITY IMPLICATIONS:
   - Anchor: Harder to forget security checks due to declarative constraints
   - Pinocchio: Easy to forget security checks - must remember every validation

6. DEVELOPMENT SPEED:
   - Anchor: Faster development due to automatic code generation
   - Pinocchio: Slower development but more explicit control

7. PERFORMANCE:
   - Anchor: Slight overhead from generated code and runtime checks
   - Pinocchio: Potentially more efficient with careful manual optimization

8. DEBUGGING:
   - Anchor: Can be harder to debug generated code
   - Pinocchio: Easier to debug since all code is explicit

The vulnerability in both frameworks is the same: missing ownership validation.
However, in Anchor it's easier to prevent with declarative constraints,
while in Pinocchio it requires remembering to add explicit checks.
*/