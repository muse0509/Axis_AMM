/// Jupiter integration for G3M rebalancing.
///
/// Production pattern (two-step, used by most protocols):
///   1. Keeper bot calls Jupiter API off-chain to get optimal swap route
///   2. Keeper executes the Jupiter swap in a separate transaction
///   3. Keeper calls our Rebalance instruction with the vault accounts
///   4. On-chain: we read actual vault balances to verify the rebalance
///   5. On-chain: we verify the G3M invariant is maintained
///
/// This is safer than single-tx CPI because:
///   - No dynamic account allocation in no_std
///   - Jupiter's account layout varies per route (can't predict at compile time)
///   - We verify the RESULT (vault balances) not the PROCESS (swap route)
///
/// Jupiter V6 program ID (for reference/validation):
///   JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4

/// Jupiter V6 program ID bytes (for client-side validation)
pub const JUPITER_PROGRAM_ID: [u8; 32] = [
    0x04, 0x58, 0x99, 0x26, 0x88, 0x1e, 0xd7, 0x10,
    0x0e, 0x13, 0x14, 0x20, 0x8b, 0x1e, 0x54, 0x25,
    0x42, 0x79, 0x4f, 0x5d, 0x08, 0x16, 0x31, 0x76,
    0xa0, 0x73, 0xb2, 0x79, 0x81, 0x40, 0x2e, 0x72,
];

use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

/// Read token account balance from SPL token account data.
/// SPL token account layout: amount is at offset 64, 8 bytes LE.
pub fn read_vault_balance(account: &AccountInfo) -> Result<u64, ProgramError> {
    let data = account.try_borrow_data()?;
    if data.len() < 72 {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(u64::from_le_bytes(
        data[64..72]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    ))
}
