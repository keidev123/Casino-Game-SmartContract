use crate::{
    constants::{CONFIG, GAME_GROUND, GLOBAL},
    errors::*,
    misc::*,
    state::{config::*, gameground::*},
};
use anchor_lang::{prelude::*, system_program};
use orao_solana_vrf::RANDOMNESS_ACCOUNT_SEED;

#[derive(Accounts)]
#[instruction(round_num: u64)]
pub struct SetWinner<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    /// CHECK: global vault pda which stores SOL
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    pub global_vault: AccountInfo<'info>,

    #[account(mut,
    constraint = global_config.authority == creator.key() @ ContractError::IncorrectAuthority)]
    creator: Signer<'info>,

    #[account(
        mut,
        seeds = [GAME_GROUND.as_bytes(), round_num.to_le_bytes().as_ref()],
        bump
    )]
    game_ground: Box<Account<'info, GameGround>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED, &game_ground.force],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,
}

impl<'info> SetWinner<'info> {
    /// Handles winner selection using VRF randomness
    ///
    /// This function:
    /// 1. Validates game is completed (time expired)
    /// 2. Checks VRF randomness is fulfilled
    /// 3. Selects winner using weighted random algorithm
    /// 4. Marks game as completed
    ///
    /// # Arguments
    /// * `round_num` - The round number to process
    ///
    /// # Returns
    /// * `Result<()>` - Success if randomness is ready and winner selected
    ///
    /// # Security
    /// Only callable by the configured authority. Winner selection is deterministic
    /// based on VRF output and cannot be manipulated.
    pub fn handler(&mut self, round_num: u64) -> Result<()> {
        require!(
            round_num < self.global_config.game_round,
            ContractError::RoundNumberError
        );

        let game_ground = &mut self.game_ground;
        let timestamp = Clock::get()?.unix_timestamp;

        require!(
            game_ground.end_date > 0 && game_ground.end_date <= timestamp,
            ContractError::GameNotCompleted
        );
        
        require!(
            !game_ground.is_completed,
            ContractError::SetWinnerCompleted
        );
        
        require!(
            game_ground.total_deposit > 0,
            ContractError::InvalidAmount
        );
        
        require!(
            !game_ground.deposit_list.is_empty(),
            ContractError::InvalidAmount
        );

        let rand_acc = crate::misc::get_account_data(&self.random)?;

        let randomness = current_state(&rand_acc);
        if randomness == 0 {
            return err!(ContractError::StillProcessing);
        }

        game_ground.set_winner(randomness)?;
        game_ground.is_completed = true;

        Ok(())
    }
}
