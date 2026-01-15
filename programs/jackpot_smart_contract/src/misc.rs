use std::mem::size_of;

use anchor_lang::{
    solana_program::{account_info::AccountInfo, program_error::ProgramError},
    AccountDeserialize,
};

use orao_solana_vrf::state::RandomnessAccountData;

/// Deserialize randomness account data from account info
pub fn get_account_data(account_info: &AccountInfo) -> Result<RandomnessAccountData, ProgramError> {
    if account_info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    RandomnessAccountData::try_deserialize(&mut &account_info.data.borrow()[..])
        .map_err(|_| ProgramError::InvalidAccountData)
}

/// Extracts the current fulfilled randomness value as u64
/// Returns 0 if randomness is not yet fulfilled
pub fn current_state(randomness: &RandomnessAccountData) -> u64 {
    if let Some(fulfilled_randomness) = randomness.fulfilled_randomness() {
        if fulfilled_randomness.len() >= size_of::<u64>() {
            let value: [u8; 8] = fulfilled_randomness[0..size_of::<u64>()]
                .try_into()
                .unwrap_or([0u8; 8]);
            return u64::from_le_bytes(value);
        }
    }
    0
}
