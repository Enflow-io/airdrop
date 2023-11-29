# casper-client put-deploy \
#     --chain-name casper-test \
#      --node-address http://46.4.20.60:7777/rpc \
#     --secret-key /Users/constantine/Projects/Casper/acc/user3/secret_key.pem \
#     --session-hash hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7 \
#     --session-entry-point "transfer" \
#     --session-arg "recipient:key='hash-460285f06e1727dec045c2f5acbe81f561fda05025c6c6713d89f7df68cbd8d5'" \
#     --session-arg "amount:u256='10'" \
#     --payment-amount "10000000000"

# --session-hash hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7 \


# send tokens to contract
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://46.4.20.60:7777/rpc \
    --secret-key /Users/constantine/Projects/Casper/acc/keys/secret_key.pem \
    --session-hash hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7 \
    --session-entry-point "transfer" \
    --session-arg "recipient:key='hash-4853156363e9e627f10ce74f81255b4f655378cb3dc584b0da63887511c4a982'" \
    --session-arg "amount:u256='1000'" \
    --payment-amount "10000000000"



# casper-client put-deploy \
#     --chain-name casper-test \
#     --node-address http://46.4.20.60:7777/rpc \
#     --secret-key /Users/constantine/Projects/Casper/acc/keys/secret_key.pem \
#     --session-hash hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7 \
#     --session-entry-point "transfer_from" \
#     --session-arg "owner:key='hash-2d7914257740a8101563776b0b1575b7776bebbc38a97a0d5a986552d76668e5'" \
#     --session-arg "recipient:key='account-hash-460285f06e1727dec045c2f5acbe81f561fda05025c6c6713d89f7df68cbd8d5'" \
#     --session-arg "amount:u256='100'" \
#     --payment-amount "10000000000"