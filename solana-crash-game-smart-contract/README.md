# 🎰 Solana Crash Game Smart Contract

> **Solana crash-game smart contract** — House bankroll, commit–reveal fairness, auto & manual cashout. Built with **Anchor**.

[![Anchor](https://img.shields.io/badge/Anchor-0.32.x-8c8c8c?logo=rust)](https://www.anchor-lang.com/)
[![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?logo=solana)](https://solana.com/)

---

## ✨ Features

| Feature | Description |
|--------|-------------|
| **🏦 House & vault** | PDA-based bankroll; program-controlled, non-custodial from operator wallet |
| **🔄 Round lifecycle** | `Betting` → `Live` → `Revealed` with clear state transitions |
| **🎲 Commit–reveal** | Server seed commitment before round; reveal settles crash multiplier |
| **💰 Cashout** | Optional **auto cashout** (1.01x–100x) or **manual cashout** during live round |
| **📤 Claims** | Players claim payouts only after round is revealed and multiplier is set |
| **⏸️ Pause** | Authority can pause house for maintenance or emergencies |

---

## 📁 Project structure

```
programs/solana_crash_smart_contract/
├── src/
│   ├── lib.rs           # Program entry, seeds, instruction dispatch
│   ├── state.rs         # House, Round, Bet, RoundStatus
│   ├── errors.rs        # CrashError enum
│   ├── utils.rs         # Payout math, crash multiplier (keccak)
│   └── instructions/
│       ├── mod.rs
│       ├── house.rs     # initialize_house, set_paused, deposit, withdraw
│       ├── round.rs     # start_round, close_betting, reveal_and_settle_round
│       └── bet.rs       # place_bet, manual_cashout, claim
├── Cargo.toml
tests/
├── crash_house.ts       # TypeScript integration tests
migrations/
└── deploy.ts
```

---

## 🚀 Quick start

```bash
# Install JS dependencies
yarn install

# Build the program
anchor build

# Run tests (local validator + TypeScript)
anchor test
```

---

## 🎮 Flow (high level)

1. **House** — Authority runs `initialize_house(edge_bps)` and `house_deposit(amount)` to fund the vault.
2. **Round** — Authority runs `start_round(round_id, commit_hash, betting_ends_at, round_ends_at)`.
3. **Betting** — Before `betting_ends_at`, players call `place_bet(amount, auto_cashout_x100?)`.
4. **Live** — After `betting_ends_at`, authority calls `close_betting()`. Players can call `manual_cashout(cashout_x100)` during this phase.
5. **Reveal** — Authority calls `reveal_and_settle_round(server_seed, nonce)`. Commit must match `keccak(server_seed \|\| nonce \|\| round_id)`; crash multiplier is derived from the same seed.
6. **Claim** — Winners call `claim()` to receive `amount * (cashout_x100 / 100)` from the vault.

---

## 🌐 Devnet deploy

1. Set cluster to devnet:
   ```bash
   solana config set --url devnet
   ```
2. In **Anchor.toml**: set `[provider] cluster = "devnet"`.
3. Deploy (program id is assigned on first deploy):
   ```bash
   anchor deploy
   ```
4. Update **program id** in:
   - `programs/solana_crash_smart_contract/src/lib.rs` → `declare_id!(...)`
   - **Anchor.toml** → `[programs.devnet]` / `[programs.localnet]` as needed.
5. Rebuild and redeploy:
   ```bash
   anchor build
   anchor deploy
   ```

---

## ⚠️ Important (production checklist)

This is a **v1 reference implementation**. Before production:

- [ ] Integrate a **VRF oracle** (e.g. Switchboard, ORAO) instead of commit–reveal only.
- [ ] Add **exposure caps** and **emergency shutdown** controls.
- [ ] Use **multisig** (or similar) for house authority.
- [ ] Get a **security audit** and **legal/compliance** review.
