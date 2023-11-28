casper-client put-deploy \
  --node-address http://46.4.20.60:7777/rpc \
  --chain-name casper-test \
  --secret-key /Users/constantine/Projects/Casper/acc/user3/secret_key.pem \
  --session-hash 19aa6060f0c6d29f0931787a89bcd4381ba59d790252809d1dddab2df5bc6dea \
  --payment-amount 100000000000 \
  --session-entry-point "withdraw" \
  --session-arg "contract_hash:key='hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7'" \
  --session-arg "amount:u256='100'" \
  --session-arg "address:key=hash-a3caf9628418b3a35bd4a92d62d35e8a244898cdea26c269bf75a2d82ad221f0'" \

