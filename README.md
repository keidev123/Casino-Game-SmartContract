# 🎰 Casino Jackpot Smart Contract

> **Enterprise-grade decentralized jackpot system built on Solana blockchain with verifiable randomness**

A production-ready, auditable smart contract implementation for a decentralized casino jackpot game. This system leverages ORAO Network's Verifiable Random Function (VRF) for provably fair random number generation, ensuring transparency and trust in the gaming experience.

[![Solana](https://img.shields.io/badge/Solana-14F46C?style=flat&logo=solana&logoColor=white)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-0.30.1-000000?style=flat)](https://www.anchor-lang.com/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/License-ISC-blue.svg)](LICENSE)

---

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Features](#features)
- [Security](#security)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Smart Contract Details](#smart-contract-details)
- [Testing](#testing)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [Contact](#contact)

---

## 🎯 Overview

This smart contract implements a decentralized jackpot system where players can deposit SOL to participate in time-limited rounds. The winner is selected using a weighted random selection algorithm based on deposit amounts, ensuring fair distribution proportional to each participant's contribution.

### Key Highlights

- **Verifiable Randomness**: Uses ORAO Network VRF for cryptographically verifiable random number generation
- **Weighted Selection**: Winner selection is proportional to deposit amounts (larger deposits = higher probability)
- **Time-Limited Rounds**: Configurable round duration with automatic completion
- **Platform Fee System**: Configurable fee structure for sustainable operations
- **Gas Optimized**: Efficient account space management and minimal compute units

---

## 🏗️ Architecture

### System Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Casino Jackpot System                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐      ┌──────────────┐      ┌──────────┐ │
│  │   Config     │      │  GameGround  │      │  Global  │ │
│  │   Account    │◄────►│   Account    │◄────►│  Vault   │ │
│  │              │      │              │      │          │ │
│  └──────────────┘      └──────────────┘      └──────────┘ │
│         │                      │                           │
│         │                      │                           │
│         ▼                      ▼                           │
│  ┌──────────────────────────────────────────────┐          │
│  │         ORAO Network VRF Integration         │          │
│  │    (Verifiable Random Function Provider)     │          │
│  └──────────────────────────────────────────────┘          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Account Structure

#### Config Account
- **Purpose**: Global configuration and state management
- **Fields**: Authority, wallets, game round counter, fee structure, limits
- **PDA**: `[CONFIG_SEED]`

#### GameGround Account
- **Purpose**: Individual game round state
- **Fields**: Round metadata, deposits, winner, timestamps, randomness
- **PDA**: `[GAME_GROUND_SEED, round_number]`
- **Dynamic**: Account size grows with number of participants

#### Global Vault
- **Purpose**: Centralized SOL storage for all game rounds
- **Type**: PDA account with signer seeds
- **PDA**: `[GLOBAL_SEED]`

---

## ✨ Features

### Core Functionality

- ✅ **Decentralized Game Rounds**: Each round is an independent on-chain account
- ✅ **Weighted Random Selection**: Fair winner selection based on deposit amounts
- ✅ **Automatic Time Management**: Rounds complete automatically after time expires
- ✅ **Multi-Deposit Support**: Players can deposit multiple times in a single round
- ✅ **Platform Fee Collection**: Configurable fee percentage (basis points)
- ✅ **Maximum Joiner Limits**: Prevents account size overflow
- ✅ **Minimum Deposit Enforcement**: Ensures meaningful participation

### Security Features

- 🔒 **Authority Validation**: Strict access control for admin functions
- 🔒 **PDA-Based Storage**: All critical accounts use Program Derived Addresses
- 🔒 **Overflow Protection**: Safe arithmetic operations throughout
- 🔒 **State Validation**: Comprehensive checks before state transitions
- 🔒 **Reentrancy Protection**: Anchor framework built-in protections

### Developer Experience

- 🛠️ **TypeScript SDK**: Full type-safe client library
- 🛠️ **CLI Tools**: Command-line interface for all operations
- 🛠️ **Comprehensive Error Handling**: Detailed error messages
- 🛠️ **Event Logging**: Structured logging for debugging

---

## 🔐 Security

### Security Considerations

1. **Randomness Source**: ORAO Network VRF provides cryptographically verifiable randomness
2. **Access Control**: Admin functions require authority signature
3. **Account Validation**: All accounts validated via Anchor constraints
4. **Arithmetic Safety**: All calculations use checked arithmetic
5. **State Machine**: Enforced state transitions prevent invalid operations

### Known Limitations

- Randomness fulfillment depends on ORAO Network oracle response time
- Account resizing requires payer wallet for rent
- Maximum participants limited by account size constraints

### Audit Recommendations

Before production deployment, consider:
- Professional smart contract audit
- Formal verification of winner selection algorithm
- Load testing with maximum participants
- Economic attack vector analysis

---

## 🚀 Installation

### Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Solana CLI**: v1.18.18 or later
- **Anchor Framework**: v0.30.1
- **Node.js**: v16+ (v18+ recommended)
- **Yarn**: Package manager

### Setup Steps

1. **Clone the repository**
```bash
git clone https://github.com/keidev123/casino-game-smartContract
cd casino-game-smartContract
```

2. **Install dependencies**
```bash
yarn install
```

3. **Build the program**
```bash
anchor build
```

4. **Run tests** (optional)
```bash
anchor test
```

---

## ⚙️ Configuration

### Anchor.toml

The project is configured for Solana devnet by default. Key configuration:

```toml
[programs.devnet]
jackpot_smart_contract = "CKaQ1zwbTdYoVjBfWMUiZGzTbf8wHfc2ExTRTM79kj7w"

[provider]
cluster = "https://devnet.helius-rpc.com/?api-key=YOUR_API_KEY"
wallet = "../key/uu.json"
```

### Environment Variables

- Update `Anchor.toml` with your RPC endpoint
- Configure wallet path for transactions
- Set program ID if deploying new instance

### Initial Configuration

Default values (configurable via `configure` instruction):
- **Platform Fee**: 100 basis points (1%)
- **Minimum Deposit**: 100,000,000 lamports (0.1 SOL)
- **Maximum Joiners**: 100 participants per round

---

## 📖 Usage

### CLI Commands

#### 1. Configure Project
Initialize or update the global configuration:
```bash
yarn script config
```

#### 2. Create Game Round
Create a new jackpot round:
```bash
yarn script create -t 60 -d 100000000 -j 100
```
- `-t, --time`: Round duration in seconds
- `-d, --minDeposit`: Minimum deposit amount in lamports
- `-j, --maxJoiner`: Maximum number of participants

#### 3. Join Game
Participate in an active round:
```bash
yarn script join -a 100000000 -g 2
```
- `-a, --amount`: Deposit amount in lamports
- `-g, --roundNum`: Round number to join

#### 4. Set Winner
Select winner after round completion (admin only):
```bash
yarn script winner -g 2
```
- `-g, --roundNum`: Round number

#### 5. Claim Reward
Winner claims their reward:
```bash
yarn script claim -g 2
```
- `-g, --roundNum`: Round number

### Programmatic Usage

```typescript
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";
import { JackpotSmartContract } from "./target/types/jackpot_smart_contract";

// Initialize connection and program
const connection = new Connection("https://api.devnet.solana.com");
const provider = new AnchorProvider(connection, wallet, {});
const program = new Program<JackpotSmartContract>(IDL, programId, provider);

// Create a game round
await program.methods
  .createGame(force, new BN(60), new BN(100000000), new BN(100))
  .accounts({ creator: wallet.publicKey, payer: feePayer.publicKey })
  .rpc();

// Join a game
await program.methods
  .joinGame(new BN(roundNum), new BN(amount))
  .accounts({ joiner: wallet.publicKey, payer: feePayer.publicKey })
  .rpc();
```

---

## 📚 API Reference

### Instructions

#### `configure`
Initialize or update global configuration.

**Accounts:**
- `payer`: Signer, pays for account initialization
- `config`: Config PDA account
- `global_vault`: Global vault PDA

**Parameters:**
- `new_config: Config`: Configuration object

#### `create_game`
Create a new jackpot round.

**Accounts:**
- `creator`: Signer, game creator
- `payer`: Signer, pays for account creation
- `global_config`: Config account
- `game_ground`: GameGround PDA (initialized)
- `global_vault`: Global vault PDA
- `random`: ORAO VRF randomness account
- `treasury`: ORAO VRF treasury
- `config`: ORAO VRF network state
- `vrf`: ORAO VRF program

**Parameters:**
- `force: [u8; 32]`: Random seed for VRF request
- `round_time: i64`: Round duration in seconds
- `min_deposit_amount: u64`: Minimum deposit in lamports
- `max_joiner_count: u64`: Maximum participants

#### `join_game`
Join an active game round.

**Accounts:**
- `joiner`: Signer, participant wallet
- `payer`: Signer, pays for account resizing
- `global_config`: Config account
- `game_ground`: GameGround account
- `global_vault`: Global vault PDA
- `team_wallet`: Team fee recipient

**Parameters:**
- `round_num: u64`: Round number
- `amount: u64`: Deposit amount in lamports

#### `set_winner`
Select winner using VRF randomness (admin only).

**Accounts:**
- `creator`: Signer, must be authority
- `global_config`: Config account
- `game_ground`: GameGround account
- `random`: ORAO VRF randomness account

**Parameters:**
- `round_num: u64`: Round number

#### `claim_reward`
Winner claims their reward.

**Accounts:**
- `winner`: Signer, must be the selected winner
- `payer`: Signer, pays for transaction
- `global_config`: Config account
- `game_ground`: GameGround account
- `global_vault`: Global vault PDA

**Parameters:**
- `round_num: u64`: Round number

---

## 🔧 Smart Contract Details

### State Accounts

#### Config
```rust
pub struct Config {
    pub authority: Pubkey,
    pub payer_wallet: Pubkey,
    pub team_wallet: Pubkey,
    pub game_round: u64,
    pub platform_fee: u64,        // Basis points (100 = 1%)
    pub min_deposit_amount: u64,
    pub max_joiner_count: u64,
    pub initialized: bool,
}
```

#### GameGround
```rust
pub struct GameGround {
    pub creator: Pubkey,
    pub game_round: u64,
    pub create_date: i64,
    pub start_date: i64,          // Set when 2nd player joins
    pub end_date: i64,             // start_date + round_time
    pub round_time: i64,
    pub total_deposit: u64,
    pub rand: u64,                 // VRF randomness result
    pub winner: Pubkey,
    pub user_count: u64,
    pub min_deposit_amount: u64,
    pub max_joiner_count: u64,
    pub force: [u8; 32],           // VRF seed
    pub is_completed: bool,
    pub is_claimed: bool,
    pub deposit_list: Vec<DepositInfo>,
}
```

### Winner Selection Algorithm

The winner is selected using a weighted random algorithm:

1. Random number from VRF: `rand`
2. Calculate selection value: `selection = rand % total_deposit`
3. Iterate through deposits in order
4. First deposit where cumulative amount exceeds selection wins

This ensures proportional probability based on deposit size.

### Error Codes

| Code | Description |
|------|-------------|
| `ValueTooSmall` | Value below minimum threshold |
| `ValueTooLarge` | Value above maximum threshold |
| `IncorrectAuthority` | Unauthorized access attempt |
| `GameAlreadyCompleted` | Attempted operation on completed game |
| `GameNotCompleted` | Game not ready for operation |
| `StillProcessing` | VRF randomness not yet fulfilled |
| `DepositAmountError` | Deposit below minimum requirement |
| `UserCountOverError` | Maximum participants exceeded |

---

## 🧪 Testing

### Run Tests

```bash
# Run all tests
anchor test

# Run with specific cluster
anchor test --provider.cluster devnet
```

### Manual Testing Workflow

```bash
# 1. Configure project
yarn script config

# 2. Create a game round (60 seconds, 0.1 SOL min, 100 max)
yarn script create -t 60 -d 100000000 -j 100

# 3. Join the game
yarn script join -a 100000000 -g 2

# 4. Wait for round completion, then set winner
yarn script winner -g 2

# 5. Winner claims reward
yarn script claim -g 2
```

---

## 🚢 Deployment

### Build for Production

```bash
# Build optimized release
anchor build --release

# Deploy to mainnet
anchor deploy --provider.cluster mainnet-beta
```

### Deployment Checklist

- [ ] Update program ID in `Anchor.toml`
- [ ] Configure mainnet RPC endpoint
- [ ] Set up production wallet
- [ ] Initialize config account
- [ ] Verify ORAO VRF integration
- [ ] Test all instructions on devnet
- [ ] Perform security audit
- [ ] Deploy to mainnet
- [ ] Monitor initial transactions

---

## 📦 Dependencies

### Smart Contract
- `anchor-lang`: 0.30.1
- `anchor-spl`: 0.30.1
- `orao-solana-vrf`: 0.5.0
- `solana-program`: 1.18.18

### TypeScript Client
- `@coral-xyz/anchor`: ^0.30.1
- `@orao-network/solana-vrf`: ^0.4.0
- `@solana/web3.js`: ^1.68.0
- `commander`: ^13.0.0

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust formatting: `cargo fmt`
- Use TypeScript strict mode
- Add tests for new features
- Update documentation

---

## 📄 License

This project is licensed under the ISC License.

---

## 📞 Contact

For questions, support, or collaboration opportunities:

- **Telegram**: [@kei4650](https://t.me/kei4650)
- **Contract Address**: [View on Solscan](https://solscan.io/account/CKaQ1zwbTdYoVjBfWMUiZGzTbf8wHfc2ExTRTM79kj7w?cluster=devnet)

---

## 🙏 Acknowledgments

- [Anchor Framework](https://www.anchor-lang.com/) - Solana development framework
- [ORAO Network](https://orao.network/) - Verifiable Random Function provider
- [Solana Foundation](https://solana.org/) - Blockchain infrastructure

---

**Built with ❤️ for the Solana ecosystem**
