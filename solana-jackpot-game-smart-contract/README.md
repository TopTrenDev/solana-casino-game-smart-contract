# 🎰 SolBet Jackpot Game - Solana Smart Contract

A **secure and provably fair Jackpot game smart contract** built on **Solana** using the **Anchor framework** (v0.31.1) and **Rust**. This contract implements a time-based jackpot game where players deposit SOL into rounds, and winners are selected using **Orao VRF** (Verifiable Random Function) for provably fair randomness.

---

## ✨ Features

- 🎲 **Provably Fair Winner Selection** - Uses Orao VRF for on-chain verifiable randomness
- ⏱️ **Time-Based Rounds** - Configurable round duration with automatic expiration
- 💰 **Weighted Winner Selection** - Higher deposits = higher probability of winning
- 🔐 **PDA-Based Security** - All funds stored in Program Derived Addresses (PDAs)
- ⚙️ **Configurable Parameters** - Platform fees, round duration, and minimum deposits
- 🎯 **Admin-Controlled Flow** - Secure admin-only operations for game management
- 📊 **Multi-Round Support** - Sequential round system with automatic counter tracking

---

## 🎮 How It Works

### Game Flow

1. **Initialize** - Admin sets up the contract with configuration parameters
2. **Create Game** - Admin creates a new round and requests VRF randomness
3. **Join Game** - Players deposit SOL into the round (stored in vault PDA)
4. **Set Winner** - Admin finalizes the round by selecting winner using VRF randomness
5. **Claim Reward** - Winner claims their reward (total pool minus platform fee)
6. **Transfer Fees** - Admin transfers remaining fees to team wallet

### Winner Selection

The contract uses a **weighted random selection** mechanism where higher deposits increase the probability of winning. Winners are selected using provably fair on-chain randomness.

---

## 🏗️ Architecture

### Program Structure

The program follows standard Anchor framework structure with separate modules for instructions, state management, and utilities.

### Core Accounts

The contract uses Program Derived Addresses (PDAs) for secure account management:
- **Config Account** - Stores global configuration and settings
- **Vault Account** - Holds all player deposits for each round
- **Game Round Account** - Tracks round-specific data and state

---

## 📋 Instructions

### 1. `initialize`

Initializes the contract with configuration parameters.

**Parameters:**
- `team_wallet`: Pubkey - Wallet to receive platform fees
- `platform_fee`: u64 - Fee in basis points (10000 = 100%)
- `round_duration`: i64 - Round duration in seconds

**Accounts:**
- `admin`: Signer - Contract administrator
- `config`: Config account

**Access Control:** Admin only

---

### 2. `create_game`

Creates a new game round and requests VRF randomness.

**Parameters:**
- `force`: [u8; 32] - VRF force parameter
- `roundIndex`: u64 - Round number

**Accounts:**
- `admin`: Signer - Contract administrator
- `config`: Config account
- `round_acc`: Game round account
- VRF-related accounts (Orao VRF program)

**Access Control:** Admin only

**Requirements:**
- Previous round must be completed
- Round index must be sequential

---

### 3. `join_game`

Allows players to join the current round by depositing SOL.

**Parameters:**
- `roundIndex`: u64 - Current round number
- `amount`: u64 - Deposit amount in lamports

**Accounts:**
- `user`: Signer - Player joining the round
- `config`: Config account
- `vault`: Vault account
- `round_acc`: Game round account

**Requirements:**
- `amount > 0`
- Round must not be expired
- Round must be active

**Behavior:**
- First deposit starts the round timer
- SOL is transferred to vault
- Deposit is recorded in the round

---

### 4. `set_winner`

Selects the winner using Orao VRF randomness.

**Parameters:**
- `force`: [u8; 32] - VRF force parameter
- `roundIndex`: u64 - Round number

**Accounts:**
- `admin`: Signer - Contract administrator
- `config`: Config account
- `round_acc`: Game round account
- VRF randomness account

**Access Control:** Admin only

**Requirements:**
- VRF randomness must be fulfilled
- Round must have deposits
- Round must not be completed

---

### 5. `winner_payout`

Allows the winner to claim their reward.

**Parameters:**
- `roundIndex`: u64 - Round number

**Accounts:**
- `admin`: Signer - Contract administrator
- `winner`: Winner's wallet
- `config`: Config account
- `round_acc`: Game round account
- `vault`: Vault account

**Access Control:** Admin + Winner verification

**Note:** Winner receives the pool amount minus platform fees. Platform fees remain in vault for `transfer_fees` instruction.

---

### 6. `transfer_fees`

Transfers remaining fees from vault to team wallet.

**Parameters:**
- `roundIndex`: u64 - Round number

**Accounts:**
- `admin`: Signer - Contract administrator
- `config`: Config account
- `round_acc`: Game round account
- `vault`: Vault account
- `team_wallet`: Team wallet

**Access Control:** Admin only

**Behavior:**
- Transfers remaining fees from vault to team wallet
- Marks round as completed to allow next round

---

## 🎲 Randomness (Orao VRF)

The contract uses **Orao Network's VRF** for provably fair, verifiable on-chain randomness. Randomness is requested when creating a game round and must be fulfilled before selecting a winner.

> ⚠️ **Important**: VRF fulfillment is asynchronous. Admin must wait for fulfillment before calling `set_winner`.

---

## 💰 Fee Structure

- **Platform Fee**: Configurable percentage (set during initialization)
- **Winner Reward**: Total pool minus platform fees
- **Team Wallet**: Receives platform fees via `transfer_fees` instruction

---

## 🧪 Development

### Prerequisites

- **Rust** >= 1.70.0
- **Solana CLI** >= 1.18.0
- **Anchor CLI** >= 0.31.1
- **Node.js** >= 18.0.0
- **Yarn** (package manager)

---

## 🔐 Security Considerations

### Implemented Security Features

- ✅ **PDA-based vault** - Funds stored in program-controlled accounts
- ✅ **Admin-only operations** - Critical functions require admin signature
- ✅ **Checked arithmetic** - Overflow protection
- ✅ **Account validation** - Strict account ownership and type checks
- ✅ **Sequential round tracking** - Prevents round manipulation
- ✅ **VRF verification** - Ensures randomness is fulfilled before use
- ✅ **Time-based expiration** - Rounds automatically expire

### Security Notes

- ⚠️ **Admin Key Security**: Admin key controls critical operations. Use a multisig or hardware wallet.
- ⚠️ **VRF Timing**: Admin must ensure VRF is fulfilled before calling `set_winner`
- ⚠️ **Round Expiration**: Expired rounds cannot accept new deposits
- ⚠️ **Not Audited**: This contract has **not been professionally audited**. Use at your own risk on mainnet.

### Best Practices

1. **Admin Key Management**: Use a multisig wallet for admin operations
2. **Testing**: Thoroughly test on devnet before mainnet deployment
3. **Monitoring**: Monitor VRF fulfillment status
4. **Rate Limiting**: Consider implementing rate limits for `join_game`
5. **Emergency Pause**: Consider adding pause functionality for emergencies

---

## 📊 Program ID

```
3x4QwXtG3mCjYe6WYHuHacmHqkJjcNo3znkmprCESxzB
```

---

## 🐛 Error Codes

| Error Code | Description |
|------------|-------------|
| `InvalidAuthority` | Caller is not the admin |
| `RoundDurationIsOver` | Round has expired |
| `RoundAlreadyCompleted` | Round is already completed |
| `InvalidRoundCounter` | Round index mismatch |
| `InvalidAmount` | Invalid deposit amount |
| `Overflow` | Arithmetic overflow |
| `WinnerAlreadySet` | Winner already selected |
| `RoundIsCompleted` | Round is completed |
| `NotWinner` | Caller is not the winner |
| `WinnerNotSet` | Winner has not been set yet |
| `StillProcessing` | VRF randomness not yet fulfilled |

---

## 📝 Configuration Example

```rust
// Initialize with:
team_wallet: <team_pubkey>
platform_fee: 500        
round_duration: 60
min_deposit_amount: 10_000
```

---

## 🔄 Game Round Lifecycle

1. **INACTIVE** - Previous round completed
2. **CREATED** - Admin creates new round, VRF randomness requested
3. **ACTIVE** - Players join and deposit SOL
4. **EXPIRED** - Round duration ends, no new deposits
5. **FINALIZED** - Admin selects winner using VRF
6. **CLAIMED** - Winner claims reward
7. **COMPLETED** - Admin transfers fees, round marked complete
8. **INACTIVE** - Ready for next round

---

## 📚 Dependencies

### Rust Dependencies
- `anchor-lang`: 0.31.1
- `orao-solana-vrf`: 0.6.1
- `solana-program`: 2.1.20

### TypeScript Dependencies
- `@coral-xyz/anchor`: ^0.31.1
- `@orao-network/solana-vrf`: ^0.6.1
