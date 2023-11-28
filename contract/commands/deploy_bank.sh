casper-client put-deploy \
  --node-address http://46.4.20.60:7777/rpc \
  --chain-name casper-test \
   --secret-key /Users/constantine/Projects/Casper/acc/user3/secret_key.pem \
  --payment-amount 50000000000 \
  --session-path /Users/constantine/Projects/Launchpad/contracts/airdrop/contract/target/wasm32-unknown-unknown/release/bank.wasm
