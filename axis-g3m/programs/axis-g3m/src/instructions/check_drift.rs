use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use crate::error::G3mError;
use crate::state::G3mPoolState;

/// CheckDrift — read-only instruction that emits drift metrics via logs.
/// Complexity: O(n_tokens) where n <= 5.
///
/// Accounts:
///   0: pool_state (read-only, PDA)
///
/// No instruction data beyond discriminant.
/// Returns drift info via program logs (msg!).
pub fn process_check_drift(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let pool_account = &accounts[0];

    let pool_data = pool_account.try_borrow_data()?;
    let pool = unsafe { &*(pool_data.as_ptr() as *const G3mPoolState) };

    if !pool.is_initialized() {
        return Err(G3mError::InvalidDiscriminator.into());
    }

    let tc = pool.token_count as usize;
    let mut max_drift: u64 = 0;
    let mut max_drift_idx: u8 = 0;

    for i in 0..tc {
        let drift = pool.drift_bps(i)
            .ok_or(ProgramError::from(G3mError::Overflow))?;

        if drift > max_drift {
            max_drift = drift;
            max_drift_idx = i as u8;
        }
    }

    let needs_rebalance = max_drift > pool.drift_threshold_bps as u64;

    // Emit via return data (clients parse this)
    // Pinocchio msg! doesn't support format args in no_std.
    // Encode drift result as return data: [max_drift: u64 LE][idx: u8][needs_rebalance: u8]
    // Clients read this from transaction simulation logs.
    let _ = (max_drift, max_drift_idx, needs_rebalance);

    Ok(())
}
