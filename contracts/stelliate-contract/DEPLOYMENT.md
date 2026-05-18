# Stelliate Contract — Testnet Deployment

| Key | Value |
|---|---|
| Network | Stellar Testnet |
| Contract ID | `CBRG2KNBWFLWOOMFNGAHYIKWJT7CIBK4KK75RWVFHYWODVWH3WV5P64F` |
| TUSD Token (SAC) | `CDDZH25OZJSMVO4HM6HWCUR65YWTGW6XSHNSOKLJMAF4TM7AXLLMLMLL` |
| Deployer | `GCDF75X533CFNOCNZ262GDOS4OMQRHHVR2CGQVIHTA7CHZCX6TTB3P24` |
| Contract Explorer | [View on Stellar Lab](https://lab.stellar.org/r/testnet/contract/CBRG2KNBWFLWOOMFNGAHYIKWJT7CIBK4KK75RWVFHYWODVWH3WV5P64F) |

## Status

Contract is **initialized and active** on testnet with:
- Commission rate: 10% (1000 bps)
- 3 registered affiliates
- 400 TUSD remaining in escrow

## Build & Deploy

```bash
cd contracts/stelliate-contract
make deploy
```

## Initialize (after a fresh deploy)

```bash
stellar contract invoke \
  --id CBRG2KNBWFLWOOMFNGAHYIKWJT7CIBK4KK75RWVFHYWODVWH3WV5P64F \
  --source deployer \
  --network testnet \
  -- initialize \
  --merchant <MERCHANT_ADDRESS> \
  --oracle <ORACLE_ADDRESS> \
  --usdc_token <TOKEN_CONTRACT_ID> \
  --commission_bps 1000
```

## Verified Interactions (20/20 ✅)

| # | Call | Outcome |
|---|---|---|
| 1 | `initialize` | Contract configured |
| 2 | `balance` | 0 |
| 3–5 | `register_affiliate` ×3 | 3 affiliates active |
| 6 | `deposit` 500 TUSD | Escrow funded |
| 7 | `balance` | 500 TUSD |
| 8 | `pay_commission` affiliate1 (1000 TUSD sale → 100 TUSD) | Paid |
| 9 | `balance` | 400 TUSD |
| 10 | affiliate1 token balance | 100 TUSD |
| 11 | `pay_commission` affiliate2 (500 TUSD sale → 50 TUSD) | Paid |
| 12 | `balance` | 350 TUSD |
| 13 | `pay_commission` affiliate3 (200 TUSD sale → 20 TUSD) | Paid |
| 14 | `balance` | 330 TUSD |
| 15 | `deposit` 200 TUSD | Escrow topped up |
| 16 | `balance` | 530 TUSD |
| 17 | `deactivate_affiliate` affiliate2 | Deactivated |
| 18 | `pay_commission` affiliate1 (300 TUSD sale → 30 TUSD) | Paid |
| 19 | `withdraw` 100 TUSD | Returned to merchant |
| 20 | `balance` | 400 TUSD |
