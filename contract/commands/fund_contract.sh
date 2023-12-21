# send tokens to contract
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://46.4.20.60:7777/rpc \
    --secret-key /Users/constantine/Projects/Casper/acc/keys/secret_key.pem \
    --session-hash hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7 \
    --session-entry-point "transfer" \
    --session-arg "recipient:key='hash-051afb4ddc4783e2e52e208c05831f976b08a3fd98ad9e50e3b5e02bc08dc041'" \
    --session-arg "amount:u256='100000'" \
    --payment-amount "10000000000"
