# 🎰 Solana Casino Games – Smart Contract Suite

A **production‑grade collection of Solana casino game smart contracts**, built with **Anchor** and designed for **fairness, security, and composability**.

This repository contains multiple on‑chain casino games such as **Plinko, Jackpot, Coinflip, Dice, and Roulette**, written in Rust and optimized for real‑world deployment on Solana.

Whether you're building a full casino platform, experimenting with on‑chain randomness, or learning advanced Solana patterns (PDAs, CPI, VRF, escrow, vaults), this repo is designed to be a strong foundation.

[![Telegram](https://img.shields.io/badge/Telegram-@toptrendev_66-2CA5E0?style=for-the-badge&logo=telegram)](https://t.me/TopTrenDev_66)
[![Twitter](https://img.shields.io/badge/Twitter-@toptrendev-1DA1F2?style=for-the-badge&logo=x)](https://x.com/intent/follow?screen_name=toptrendev)
[![Discord](https://img.shields.io/badge/Discord-toptrendev-5865F2?style=for-the-badge&logo=discord)](https://discord.com/users/648385188774019072)

---

## ✨ Key Features

- 🧠 **Provably Fair Game Logic**
- 🔐 **Secure Vault & Treasury Management**
- 🎲 **On‑chain Randomness (VRF‑ready)**
- ⚡ **Low‑latency, high‑throughput Solana execution**
- 🧩 **Modular & Extensible Architecture**
- 🧪 **Test‑driven with Anchor test suite**
- 🛠 **Production‑oriented account design (PDAs, seeds, bumps)**

---

## 🎮 Games Included

| Game         | Description                                                  |
| ------------ | ------------------------------------------------------------ |
| **Plinko**   | Ball drop game with configurable risk and payout multipliers |
| **Coinflip** | Simple 50/50 wager game                                      |
| **Dice**     | Roll‑based game with adjustable win probability              |
| **Roulette** | Classic roulette mechanics adapted for on‑chain execution    |
| **Jackpot**  | Pooled betting game with winner‑takes‑all logic              |

> Each game is implemented as an **independent Anchor program** or **isolated module**, allowing easy reuse or selective deployment.

---

## 🎲 Randomness Strategy

This repo is designed to support **secure randomness**, including:

- ⚠️ Pseudo‑random fallback (for local testing)
- 🔮 **VRF integration ready** (e.g. Orao, Switchboard)
- ⛓ Seed‑based entropy using blockhash + user input (non‑VRF modes)

> **Important:** For mainnet deployment, **VRF is strongly recommended** to prevent manipulation.

---

## 💰 Betting & Payout Flow

1. Player submits bet + parameters
2. Funds transferred to game vault PDA
3. Random outcome is generated
4. Win/loss is calculated on‑chain
5. Payout (if any) sent back to player
6. House edge retained in treasury

All calculations are performed **fully on‑chain**.

---

## 🔐 Security Notes

- ✔ Checked arithmetic (no overflows)
- ✔ PDA‑only vault ownership
- ✔ Explicit signer checks
- ✔ Configurable max bet & limits

> ⚠️ **This repo has not been audited.** Use at your own risk for mainnet deployments.

---

## 🧩 Extending the Repo

You can easily add new games by:

1. Creating a new Anchor program
2. Reusing the shared vault & randomness utilities
3. Defining game‑specific payout logic

Examples:

- Blackjack
- Baccarat
- Crash game
- Slots

---

## ⭐ Support

If you find this project useful, consider **starring the repo** ⭐

Happy building on Solana 🚀
