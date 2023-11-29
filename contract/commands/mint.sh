casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://46.4.20.60:7777/rpc \
    --secret-key /Users/constantine/Projects/Casper/acc/keys/secret_key.pem \
    --session-hash hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7 \
    --session-entry-point "mint" \
    --session-arg "owner:key='hash-f4c10aeb89a75d41b5545bb4183940dba4eb2e6071015da1f61e4c2a822d1c43'" \
    --session-arg "amount:u256='1000000'" \
    --payment-amount "10000000000"