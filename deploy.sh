casper-client put-deploy \
  --node-address http://46.4.20.60:7777/rpc \
  --chain-name casper-test \
  --secret-key /Users/constantine/Projects/Casper/acc/keys/secret_key.pem \
  --payment-amount 5000000000000 \
  --session-path contract/target/wasm32-unknown-unknown/release/contract.wasm