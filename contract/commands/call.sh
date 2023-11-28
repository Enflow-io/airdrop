casper-client put-deploy \
  --node-address http://46.4.20.60:7777/rpc \
  --chain-name casper-test \
  --secret-key /Users/constantine/Projects/Casper/acc/keys/secret_key.pem \
  --session-hash f81be3f0aa437e88b68120bf6b269f70a7320766e464c3a191a7d3d6ec6476fd \
  --payment-amount 5000000000000 \
  --session-entry-point "counter_inc"

