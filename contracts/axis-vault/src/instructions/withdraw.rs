use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_token::instructions::{Burn, Transfer};

use crate::error::VaultError;
use crate::state::{load, load_mut, EtfState};

/// Withdraw — burn ETF tokens, return proportional basket tokens.
///
/// share = effective_burn / total_supply (after fee deduction)
/// For each token: output = vault_balance * share
///
/// Accounts:
///   0: [signer]    withdrawer
///   1: [writable]  etf_state PDA
///   2: [writable]  etf_mint
///   3: [writable]  withdrawer_etf_token_account (ETF tokens to burn)
///   4: []          token_program
///   5..5+N: [writable] vault token accounts (source)
///   5+N..5+2N: [writable] withdrawer's basket token accounts (destination)
///
/// Data: [burn_amount: u64][min_tokens_out: u64][name: bytes for PDA derivation]
pub fn process_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    burn_amount: u64,
    min_tokens_out: u64,
    name: &[u8],
) -> ProgramResult {
    if burn_amount == 0 {
        return Err(VaultError::ZeroDeposit.into());
    }

    let withdrawer = &accounts[0];
    let etf_state_ai = &accounts[1];
    let etf_mint_ai = &accounts[2];
    let withdrawer_etf_ata = &accounts[3];
    let _tok = &accounts[4];

    if !withdrawer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (tc, total_supply, authority, bump, fee_bps) = {
        let data = etf_state_ai.try_borrow_data()?;
        let etf = unsafe { load::<EtfState>(&data) }
            .ok_or(ProgramError::InvalidAccountData)?;
        if !etf.is_initialized() {
            return Err(VaultError::InvalidDiscriminator.into());
        }
        if etf.total_supply == 0 {
            return Err(VaultError::DivisionByZero.into());
        }
        (etf.token_count as usize, etf.total_supply, etf.authority, etf.bump, etf.fee_bps)
    };

    if burn_amount > total_supply {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Compute fee on burn
    let fee_amount = burn_amount
        .checked_mul(fee_bps as u64)
        .ok_or(VaultError::Overflow)?
        / 10_000;
    let effective_burn = burn_amount
        .checked_sub(fee_amount)
        .ok_or(VaultError::Overflow)?;

    // Burn ETF tokens from withdrawer
    Burn {
        account: withdrawer_etf_ata,
        mint: etf_mint_ai,
        authority: withdrawer,
        amount: burn_amount,
    }
    .invoke()?;

    // Transfer proportional basket tokens from vaults to withdrawer
    let bump_bytes = [bump];
    let vault_signer_seeds = [
        Seed::from(b"etf".as_ref()),
        Seed::from(authority.as_ref()),
        Seed::from(name),
        Seed::from(bump_bytes.as_ref()),
    ];

    let mut total_withdrawn: u64 = 0;

    for i in 0..tc {
        let vault = &accounts[5 + i];
        let dest = &accounts[5 + tc + i];

        // Read vault balance at offset 64 (SPL token account amount)
        let vault_balance = {
            let data = vault.try_borrow_data()?;
            if data.len() < 72 {
                return Err(ProgramError::InvalidAccountData);
            }
            u64::from_le_bytes(
                data[64..72].try_into().map_err(|_| ProgramError::InvalidAccountData)?
            )
        };

        // Proportional share: vault_balance * effective_burn / total_supply
        let withdraw_amount = (vault_balance as u128)
            .checked_mul(effective_burn as u128)
            .ok_or(VaultError::Overflow)?
            .checked_div(total_supply as u128)
            .ok_or(VaultError::DivisionByZero)? as u64;

        if withdraw_amount > 0 {
            Transfer {
                from: vault,
                to: dest,
                authority: etf_state_ai,
                amount: withdraw_amount,
            }
            .invoke_signed(&[Signer::from(&vault_signer_seeds)])?;
        }

        total_withdrawn = total_withdrawn
            .checked_add(withdraw_amount)
            .ok_or(VaultError::Overflow)?;
    }

    // Slippage check
    if total_withdrawn < min_tokens_out {
        return Err(VaultError::SlippageExceeded.into());
    }

    // Update total supply (checked_sub prevents underflow to u64::MAX)
    {
        let mut data = etf_state_ai.try_borrow_mut_data()?;
        let etf = unsafe { load_mut::<EtfState>(&mut data) }
            .ok_or(ProgramError::InvalidAccountData)?;
        etf.total_supply = etf.total_supply
            .checked_sub(burn_amount)
            .ok_or(VaultError::Overflow)?;
    }

    Ok(())
}
