# тут прописываем какому аккаунт сколько токенов полагается
# испольузуем hash-b7558c7308710e53b301883dd80e8d4ec6dab4e90c20994c0c683fcee079976d
# как с transfer токена

# session-hash - это инстанс контракта (не пакет)
casper-client put-deploy \
  --node-address http://46.4.20.60:7777/rpc \
  --chain-name casper-test \
  --secret-key /Users/constantine/Projects/Casper/acc/user3/secret_key.pem \
  --session-hash 0c12463b860230f3c11a4bda435485a4904d6f3362ff97db29fb6d7edf8696d3 \
  --payment-amount 100000000000 \
  --session-entry-point "set_drop_balances" \
  --session-args-json '[{"name": "balances","type":{"Map":{"key":"String","value":"I32"}},"value": [{"key":"hash-b7558c7308710e53b301883dd80e8d4ec6dab4e90c20994c0c683fcee079976d", "value":50},{"key":"hash-0291e78eeae003ba2703f154092b2018198009f0bbde44ecf6bdbace7802bd95", "value":50}] }]'
