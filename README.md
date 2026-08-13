# CreditFlow Atlas

Interactive stakeholder operating model for a bank-grade credit-builder rewards and hardship-intervention platform.

## Prior art

The cached LUXERON repository index was searched for credit, loan, bank, loyalty, reward, airline, and retail concepts. `SOLVENCY` and the Transaction Buffering Protocol family are adjacent bank-liquidity work; no credit-builder rewards orchestration product was found. This application is new ground and keeps those systems outside its initial product boundary.

## Stack

- Svelte 5 + TypeScript frontend
- Axum + Rust API
- rusqlite scenario persistence
- Tauri 2 desktop wrapper
- Docker/Render web deployment

## Run locally

```powershell
cd frontend
npm install
npm run build
cd ..\backend
cargo run
```

Open http://localhost:3000. Set `STATIC_DIR=../frontend/dist` when running from another working directory.

## Product boundary

This is an educational and commercial operating-model application, not a lending, securities, custody, or credit-decision system. It is not legal, financial, or regulatory advice.
