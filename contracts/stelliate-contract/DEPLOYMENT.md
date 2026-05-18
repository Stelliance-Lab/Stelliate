# Stelliate Contract — Testnet Deployment

| Key | Value |
|---|---|
| Network | Stellar Testnet |
| Contract ID | `CDHKWCARDV6HPI7O2UV32LHZGPWHGYBAY6FLBJMVWOEBG4VITMAGOGZ2` |
| WASM Hash | `477f00db875793250e191b575748c69165bc802729176f6c99ea5be9160e57a5` |
| Deployer | `GCDF75X533CFNOCNZ262GDOS4OMQRHHVR2CGQVIHTA7CHZCX6TTB3P24` |
| Deploy Tx | [View on Stellar Expert](https://stellar.expert/explorer/testnet/tx/5bedfc30408aeb25257378dbafe37176ac5330768835bac173bc5ab0ae00fd83) |
| Contract Explorer | [View on Stellar Lab](https://lab.stellar.org/r/testnet/contract/CDHKWCARDV6HPI7O2UV32LHZGPWHGYBAY6FLBJMVWOEBG4VITMAGOGZ2) |

## Build & Deploy

```bash
cd contracts/stelliate-contract
make deploy
```

## Initialize (after deploy)

```bash
stellar contract invoke \
  --id CDHKWCARDV6HPI7O2UV32LHZGPWHGYBAY6FLBJMVWOEBG4VITMAGOGZ2 \
  --source deployer \
  --network testnet \
  -- initialize \
  --merchant <MERCHANT_ADDRESS> \
  --oracle <ORACLE_ADDRESS> \
  --usdc_token <USDC_CONTRACT_ID> \
  --commission_bps 1000
```
