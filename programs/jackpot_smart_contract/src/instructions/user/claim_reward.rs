use crate::{
    constants::{CONFIG, GAME_GROUND, GLOBAL},
    errors::*,
    state::{config::*, gameground::*},
    utils::*,
};
use anchor_lang::{prelude::*, system_program};

#[derive(Accounts)]
#[instruction(round_num: u64)]
pub struct ClaimReward<'info> {
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
        constraint = game_ground.winner == winner.key() @ ContractError::IncorrectAuthority)]
    winner: Signer<'info>,

    #[account(mut,
        constraint = global_config.payer_wallet == payer.key() @ ContractError::IncorrectPayerAuthority)]
    payer: Signer<'info>,

    #[account(
        mut,
        seeds = [GAME_GROUND.as_bytes(), round_num.to_le_bytes().as_ref()],
        bump
    )]
    game_ground: Box<Account<'info, GameGround>>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,
}

impl<'info> ClaimReward<'info> {
    /// Handles reward claiming by the winner
    ///
    /// This function:
    /// 1. Validates caller is the selected winner
    /// 2. Validates game is completed and not yet claimed
    /// 3. Transfers total deposit from global vault to winner
    /// 4. Marks reward as claimed
    ///
    /// # Arguments
    /// * `round_num` - The round number to claim
    /// * `global_vault_bump` - Bump seed for global vault PDA
    ///
    /// # Returns
    /// * `Result<()>` - Success if all validations pass and transfer succeeds
    ///
    /// # Note
    /// Winner receives the full total_deposit amount (fees already deducted on join)
    pub fn handler(&mut self, round_num: u64, global_vault_bump: u8) -> Result<()> {
        require!(
            round_num < self.global_config.game_round,
            ContractError::RoundNumberError
        );

        let game_ground = &mut self.game_ground;

        require!(
            !game_ground.is_claimed,
            ContractError::WinnerClaimed
        );

        require!(
            game_ground.is_completed,
            ContractError::GameNotCompleted
        );
        
        require!(
            game_ground.winner == self.winner.key(),
            ContractError::IncorrectAuthority
        );
        
        require!(
            game_ground.total_deposit > 0,
            ContractError::InvalidAmount
        );

        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL.as_bytes(), &[global_vault_bump]]];

        sol_transfer_with_signer(
            self.global_vault.to_account_info(),
            self.winner.to_account_info(),
            &self.system_program,
            signer_seeds,
            game_ground.total_deposit,
        )?;

        game_ground.is_claimed = true;

        Ok(())
    }
}
