# 🪙 Solana Coinflip — Anchor Smart Contract

A minimal, on-chain **coinflip game** for Solana built with Anchor. Players stake SOL, pick heads or tails, and the program resolves the flip deterministically. Winners get **2× stake** minus an optional house fee.

[![Telegram](https://img.shields.io/badge/Telegram-@toptrendev_66-2CA5E0?style=for-the-badge&logo=telegram)](https://t.me/TopTrenDev_66)
[![Twitter](https://img.shields.io/badge/Twitter-@toptrendev-1DA1F2?style=for-the-badge&logo=twitter)](https://x.com/toptrendev)

---

## 📋 Table of Contents

- [Overview](#-overview)
- [How It Works](#-how-it-works)
- [Instructions](#-instructions)
- [Project Structure](#-project-structure)
- [Getting Started](#-getting-started)
- [Client Integration](#-client-integration)
- [Security & Design Notes](#-security--design-notes)

---

## 🎯 Overview

| Feature        | Description                                                                                                                                           |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| **House**      | One house per authority; vault PDA holds all stakes; configurable `fee_bps` on winnings.                                                              |
| **Game**       | Each flip is a unique PDA: `[b"game", house, game_authority]`. Client uses a new keypair per game.                                                    |
| **Resolution** | Deterministic: `outcome = hash(game_pubkey \|\| created_at_slot \|\| resolve_slot) % 2`. Anyone can call `resolve_flip`; no house signature required. |

---

## ⚙️ How It Works

1. **House** — Authority initializes the house once: creates vault PDA and sets fee (0–10000 bps).
2. **Create flip** — Player chooses side (0 = heads, 1 = tails), stakes `amount` lamports; funds move to the house vault.
3. **Resolve** — Anyone can resolve. Outcome is derived from game key and slot; winner receives `2 × amount − fee`, loser’s stake remains in the vault.

Fairness relies on the **resolve slot** being unknown at creation time, so the outcome cannot be chosen in advance.

---

## 📜 Instructions

| Instruction        | Description                                                                                                                                                       |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `initialize_house` | Create house + vault for an authority. Set `fee_bps` (0–10000). Call once per house.                                                                              |
| `create_flip`      | Create a flip: player chooses `side` (0 = heads, 1 = tails), stakes `amount` lamports. Requires signer `game_authority` (e.g. new keypair) for a unique game PDA. |
| `resolve_flip`     | Resolve the flip. Outcome = `hash(game_key \|\| created_at \|\| clock.slot) % 2`. On win: player gets `2 × amount − fee`; on loss: stake stays in vault.          |

---

## 📁 Project Structure

```
solana-coinflip-smart-contract/
├── 📄 Anchor.toml
├── 📄 Cargo.toml
├── 📄 README.md
└── programs/
    └── solana-coinflip-smart-contract/
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Program entry & instruction routing
            ├── state.rs            # House, Game, enums
            ├── errors.rs           # CoinflipError
            ├── utils.rs            # compute_flip_outcome, compute_win_payout
            └── instructions/
                ├── mod.rs
                ├── initialize.rs   # InitializeHouse + handler
                ├── create_flip.rs  # CreateFlip + handler
                └── resolve_flip.rs # ResolveFlip + handler
```

---

## 🚀 Getting Started

### Build

```bash
# Install Anchor (if needed)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest && avm use latest

# Build (generates program ID and IDL)
anchor build
```

### Deploy

```bash
# Deploy to devnet
anchor deploy
```

After the first `anchor build`, you can run `anchor keys list` and update the program ID in `Anchor.toml` and in the program crate (`declare_id!` and `Cargo.toml` if applicable) when using a new keypair.

---

## 🔌 Client Integration

High-level flow for a client (e.g. TypeScript with `@coral-xyz/anchor`):

1. **Initialize house** (once)  
   `initialize_house(house_authority, fee_bps)`  
   Accounts: house (PDA), vault (PDA), authority, system_program.

2. **Create flip**  
   Generate a new keypair for `game_authority`.  
   `create_flip(player, side, amount)`  
   Accounts: house, game (PDA from house + game_authority), vault, player, game_authority, clock, system_program.  
   Player must send `amount` lamports (transfer in the same tx).

3. **Resolve**  
   `resolve_flip(house, game, vault, player, clock)`  
   Anyone can call; outcome is deterministic from game key and slots.

---

## 🔒 Security & Design Notes

- **Randomness** — Resolution uses `hash(game_pubkey || created_at_slot || resolve_slot) % 2`. For production-grade unpredictability, consider a commit–reveal scheme or an oracle/VRF; this design keeps the contract self-contained and simple.
- **Game uniqueness** — One game per `(house, game_authority)`. Reuse the same `game_authority` only after the previous game is resolved (and optionally closed if you add a `close_game` instruction).
