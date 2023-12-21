# --session-hash  - contract instance hash

casper-client put-deploy \ 
  --node-address http://46.4.20.60:7777/rpc \
  --chain-name casper-test \
  --secret-key /Users/constantine/Projects/Casper/acc/contract-user/secret_key.pem \
  --session-hash 178b2cb2dd25f26e646dc57d143ef7ac4a21b2c62aa719cbcafe3512453004f7 \ 
  --payment-amount 100000000000 \
  --session-entry-point "withdraw" \
  --session-arg "contract_hash:key='hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7'" \
  --session-arg "amount:u256='1'" \
  --session-arg "address:key=hash-a3caf9628418b3a35bd4a92d62d35e8a244898cdea26c269bf75a2d82ad221f0'" \

