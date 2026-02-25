# 🎲 Solana Dice Smart Contract

A Solana program (Anchor) for a provably fair dice-style casino game. Players bet on a target number (under/over) and the operator resolves each round. SOL is held in PDAs until the result is set.

[![Telegram](https://img.shields.io/badge/Telegram-@toptrendev_66-2CA5E0?style=for-the-badge&logo=telegram)](https://t.me/TopTrenDev_66)
[![Twitter](https://img.shields.io/badge/Twitter-@toptrendev-1DA1F2?style=for-the-badge&logo=twitter)](https://x.com/toptrendev)
[![Discord](https://img.shields.io/badge/Discord-toptrendev-5865F2?style=for-the-badge&logo=discord)](https://discord.com/users/648385188774019072)

## Features

- **Under/Over dice**: Player picks a target number (e.g. 50) and bets “under” or “over”; payout is derived from probability and configurable RTP.
- **Multi-role admin**: Separate authorities for operations (resolve games), finance (withdraw from vault), and config (RTP, limits).
- **Per-session vaults**: Each game session has its own vault PDA; player funds are locked until the operator sets win/lose.
- **Configurable limits**: Min/max bet, max win amount, and valid target number range are set at init and updatable by the update authority.

## Program Overview

### Instructions

| Instruction           | Signers / Authority        | Description |
|-----------------------|----------------------------|-------------|
| `initialize`          | Admin (payer)              | Create global config PDA and set super_admin, operation_authority, finance_authority, update_authority and default RTP/limits. |
| `play_game`           | Player + Operator          | Player bets (target_number, is_under, bet_amount, game_session_id). SOL moves to game vault; `PlayerPool` created. |
| `set_result`          | Operator                   | Resolve round: win → pay player from game vault (casino tops up if needed); lose → send game vault to casino. Closes `PlayerPool`. |
| `withdraw`            | Finance authority          | Transfer SOL from casino vault to a recipient. |
| `set_rtp`             | Update authority           | Set global RTP (0–99). |
| `set_max_win_amount`  | Update authority           | Set max win amount per bet. |
| `set_min_bet_amount`  | Update authority           | Set minimum bet. |
| `set_min_num` / `set_max_num` | Update authority | Set allowed target number range (e.g. 9–90). |
| `set_operation_authority` / `set_finance_authority` / `set_update_authority` | Super admin | Change the respective authority pubkeys. |

### Main Accounts (PDAs)

- **GlobalPool** (`global-authority`): Admins + RTP, max_win_amount, min_bet_amount, min_num, max_num.
- **Casino vault** (`vault-authority`): Main SOL treasury (receives lost bets; pays wins and withdrawals).
- **PlayerPool** (per player + `game_session_id`): One active bet per session (bet, status, is_under, target_num, player).
- **Game vault** (per player + `game_session_id`): Holds the player’s bet until `set_result`.

### Default Constants (from `constants.rs`)

- RTP: 95%
- Max win amount: 10 SOL (lamports in code)
- Min bet: 0.1 SOL
- Target number range: 9–90 (exclusive)

## Configuration

- **Cluster / RPC**: Set in `Anchor.toml` under `[provider]` or via env (e.g. `ANCHOR_PROVIDER_URL`).
- **Wallet**: Set `[provider] wallet` in `Anchor.toml` or use `ANCHOR_WALLET`.

Avoid committing API keys. Prefer environment variables or a local override for RPC URLs and wallet paths.

## Testing

```bash
yarn run test
```

Requires a `tests/` directory and test files (e.g. TypeScript with `@coral-xyz/anchor`). Add at least one test that runs `initialize`, then `play_game` and `set_result`, to guard against regressions.
