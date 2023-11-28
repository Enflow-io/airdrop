casper-client put-deploy \
  --node-address http://localhost:11101/rpc \
  --chain-name casper-net-1 \
   --secret-key /Users/constantine/Projects/Casper/acc/local/secret_key.pem \
  --payment-amount 50000000000 \
  --session-path /Users/constantine/Projects/Launchpad/contracts/airdrop/contract/target/wasm32-unknown-unknown/release/contract.wasm

# --secret-key /Users/constantine/Projects/Casper/acc/user3/secret_key.pem \
  # --node-address http://46.4.20.60:7777/rpc \