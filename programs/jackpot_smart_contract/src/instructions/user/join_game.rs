use crate::{
    constants::{CONFIG, GAME_GROUND, GLOBAL},
    errors::*,
    state::{config::*, gameground::*},
    utils::*,
};
use anchor_lang::{prelude::*, system_program};

#[derive(Accounts)]
#[instruction(round_num: u64)]
pub struct JoinGame<'info> {
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

    /// CHECK: should be same with the address in the global_config
    #[account(
        mut,
        constraint = global_config.team_wallet == team_wallet.key() @ContractError::IncorrectTeamWalletAuthority
    )]
    pub team_wallet: AccountInfo<'info>,

    #[account(mut)]
    joiner: Signer<'info>,

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

impl<'info> JoinGame<'info> {
    /// Handles player joining a game round
    ///
    /// This function:
    /// 1. Validates deposit amount and game state
    /// 2. Calculates and transfers platform fee to team wallet
    /// 3. Transfers remaining deposit to global vault
    /// 4. Resizes game account if needed for new participant
    /// 5. Updates game state (starts timer when 2nd player joins)
    ///
    /// # Arguments
    /// * `round_num` - The round number to join
    /// * `amount` - Total deposit amount in lamports
    ///
    /// # Returns
    /// * `Result<()>` - Success if all validations pass and transfers succeed
    ///
    /// # Note
    /// Platform fee is deducted from the deposit amount before adding to vault
    pub fn handler(&mut self, round_num: u64, amount: u64) -> Result<()> {
        if amount <= 0 {
            return err!(ContractError::InvalidAmount);
        }

        require!(
            round_num < self.global_config.game_round,
            ContractError::RoundNumberError
        );

        let game_ground = &mut self.game_ground;
        require!(
            amount >= game_ground.min_deposit_amount,
            ContractError::DepositAmountError
        );

        let timestamp = Clock::get()?.unix_timestamp;

        // Check if game has started (after 2 players joined)
        if game_ground.user_count >= 2 {
            require!(
                game_ground.end_date > timestamp,
                ContractError::GameAlreadyCompleted
            );

            require!(
                !game_ground.is_completed,
                ContractError::GameAlreadyCompleted
            );
        }

        let global_config = &mut self.global_config;
        let team_wallet = &mut self.team_wallet;
        let source = &mut self.global_vault.to_account_info();

        // Calculate platform fee and deposit amount
        let platform_fee_lamports = bps_mul(global_config.platform_fee, amount, 10_000)
            .ok_or(ContractError::ArithmeticError)?;
        
        let deposit_amount_applied = amount
            .checked_sub(platform_fee_lamports)
            .ok_or(ContractError::InvalidAmount)?;

        require!(
            deposit_amount_applied > 0,
            ContractError::InvalidAmount
        );

        // Check if user already exists to determine if we need to resize
        let user_exists = game_ground.deposit_list.iter().any(|d| d.user == self.joiner.key());
        let new_list_len = if user_exists {
            game_ground.deposit_list.len()
        } else {
            game_ground.deposit_list.len() + 1
        };

        // Resize account before appending to accommodate new entry
        resize_account(
            game_ground.to_account_info().clone(),
            GameGround::space(new_list_len),
            self.payer.to_account_info().clone(),
            self.system_program.to_account_info().clone(),
        )?;

        // Transfer SOL to global vault (deposit amount after fee)
        sol_transfer_from_user(
            &self.joiner,
            source.clone(),
            &self.system_program,
            deposit_amount_applied,
        )?;

        // Transfer platform fee to team wallet
        if platform_fee_lamports > 0 {
            sol_transfer_from_user(
                &self.joiner,
                team_wallet.clone(),
                &self.system_program,
                platform_fee_lamports,
            )?;
        }

        // Append deposit after transfers are successful
        game_ground.append(self.joiner.key(), deposit_amount_applied);

        if game_ground.user_count == 2 {
            game_ground.start_date = timestamp;
            game_ground.end_date = game_ground.start_date + game_ground.round_time;
        }

        require!(
            game_ground.max_joiner_count >= game_ground.user_count,
            ContractError::UserCountOverError
        );

        Ok(())
    }
}
