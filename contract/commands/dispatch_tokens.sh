# --session-hash  - contract instance hash
casper-client put-deploy \
  --node-address http://46.4.20.60:7777/rpc \
  --chain-name casper-test \
  --secret-key /Users/constantine/Projects/Casper/acc/contract-user/secret_key.pem \
  --session-hash 178b2cb2dd25f26e646dc57d143ef7ac4a21b2c62aa719cbcafe3512453004f7 \
  --payment-amount 100000000000 \
  --session-entry-point "withdraw"