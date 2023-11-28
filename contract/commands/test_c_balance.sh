casper-client put-deploy -n http://46.4.20.60:7777 \
--secret-key /Users/constantine/Projects/Casper/acc/keys/secret_key.pem \
--chain-name casper-test \
--session-package-name "cep18_test_contract" \
--session-entry-point "check_balance_of" \
--session-arg "token_contract:key='hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7'" \
--session-arg "address:key='hash-4a31d64f1c5c09ce791083db791303f1f2cf83c208efc0a37ee1a3bf7422d9a7'" \
--payment-amount 1000000000