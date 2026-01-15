//! # Casino Jackpot Smart Contract
//!
//! A decentralized jackpot system built on Solana using Anchor framework.
//! This program implements a fair, verifiable random number-based winner selection
//! mechanism using ORAO Network's VRF (Verifiable Random Function).
//!
//! ## Overview
//!
//! The contract manages multiple independent game rounds where participants deposit SOL.
//! Winners are selected using weighted random selection based on deposit amounts,
//! ensuring proportional probability distribution.

use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
// pub mod events;
pub mod instructions;
pub mod misc;
pub mod state;
pub mod utils;

use instructions::{claim_reward::*, configure::*, create_game::*, join_game::*, set_winner::*};

use state::config::*;

declare_id!("CKaQ1zwbTdYoVjBfWMUiZGzTbf8wHfc2ExTRTM79kj7w");

/// Main program module containing all instruction handlers
#[program]
pub mod jackpot_smart_contract {

    use super::*;

    /// Initialize or update the global configuration
    ///
    /// # Arguments
    /// * `ctx` - Context containing config account and payer
    /// * `new_config` - Configuration object with all settings
    ///
    /// # Errors
    /// Returns error if authority validation fails or account initialization fails
    pub fn configure(ctx: Context<Configure>, new_config: Config) -> Result<()> {
        msg!("Configuring contract: {:#?}", new_config);
        ctx.accounts.handler(new_config, ctx.bumps.config)
    }

    /// Create a new jackpot game round
    ///
    /// # Arguments
    /// * `ctx` - Context containing game accounts and VRF accounts
    /// * `force` - 32-byte seed for VRF randomness request
    /// * `round_time` - Duration of the round in seconds
    /// * `min_deposit_amount` - Minimum deposit required to join (lamports)
    /// * `max_joiner_count` - Maximum number of participants allowed
    ///
    /// # Errors
    /// Returns error if validation fails or VRF request fails
    pub fn create_game(
        ctx: Context<CreateGame>,
        force: [u8; 32],
        round_time: i64,
        min_deposit_amount: u64,
        max_joiner_count: u64,
    ) -> Result<()> {
        ctx.accounts
            .handler(force, round_time, min_deposit_amount, max_joiner_count)
    }

    /// Select winner for a completed game round using VRF randomness
    ///
    /// # Arguments
    /// * `ctx` - Context containing game account and VRF account
    /// * `round_num` - Round number to process
    ///
    /// # Errors
    /// Returns error if game not completed, randomness not ready, or validation fails
    ///
    /// # Security
    /// Only callable by the configured authority
    pub fn set_winner(ctx: Context<SetWinner>, round_num: u64) -> Result<()> {
        ctx.accounts.handler(round_num)
    }

    /// Join an active game round by depositing SOL
    ///
    /// # Arguments
    /// * `ctx` - Context containing game account and participant wallet
    /// * `round_num` - Round number to join
    /// * `amount` - Deposit amount in lamports
    ///
    /// # Errors
    /// Returns error if game completed, amount too small, or max participants reached
    pub fn join_game(ctx: Context<JoinGame>, round_num: u64, amount: u64) -> Result<()> {
        ctx.accounts.handler(round_num, amount)
    }

    /// Claim reward for winning a game round
    ///
    /// # Arguments
    /// * `ctx` - Context containing game account and winner wallet
    /// * `round_num` - Round number to claim
    ///
    /// # Errors
    /// Returns error if caller is not winner, already claimed, or game not completed
    pub fn claim_reward(ctx: Context<ClaimReward>, round_num: u64) -> Result<()> {
        ctx.accounts.handler(round_num, ctx.bumps.global_vault)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
