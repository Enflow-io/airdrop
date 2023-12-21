## 1. Install / update contract

```
cargo build --release --target wasm32-unknown-unknown
./commands/deploy_bank.sh 
```


## 2. Fund contract

```
./commands/fund_bank.sh
```

## 3. Set user->token balance

```
./commands/set_balance.sh
```