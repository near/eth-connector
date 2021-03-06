.PHONY: compile test

NETWORK=ropsten
NEAR_EVM_ACCOUNT=develop.aurora
NEAR_ACCOUNT=
QUERY_NEAR_ACCOUNT=${NEAR_ACCOUNT}
RPC_ENDPOINT=https://rpc.testnet.near.org
ETH_RECIPIENT=
ETH_ADDRESS=
AMOUNT=0
FEE=0
PROOF=
TX_HASH=
RECEIPT_ID=
RELAYER_NEAR_ADDRESS=
RELAYER_ADDRESS_IN_AURORA=

NEAR_RECIPIENT=${NEAR_ACCOUNT}

compile:
	mkdir -p build/proofs && yarn hardhat compile

test:
	yarn hardhat test

eth-get-prover-address:
	yarn hardhat run --network ${NETWORK} scripts/eth_get_prover_address.js

eth-deploy-contracts:
	yarn hardhat run --network ${NETWORK} scripts/eth_deploy_contracts.js

eth-deposit-to-near:
	yarn hardhat --network ${NETWORK} eth-deposit-to-near --near-recipient ${NEAR_RECIPIENT} --amount ${AMOUNT} --fee ${FEE}

eth-deposit-to-evm:
	yarn hardhat --network ${NETWORK} eth-deposit-to-evm --eth-recipient-on-near ${ETH_RECIPIENT} --amount ${AMOUNT} --fee ${FEE}

eth-withdraw:
	yarn hardhat run --network ${NETWORK} scripts/eth_withdraw.js

eth-generate-deposit-proof:
	mkdir -p build/proofs && yarn hardhat --network ${NETWORK} eth-generate-deposit-proof --tx-hash ${TX_HASH}

eth-finalize-withdraw-from-near:
	yarn hardhat --network ${NETWORK} eth-finalize-withdraw-from-near --receipt-id ${RECEIPT_ID} --near-account ${NEAR_ACCOUNT}

near-finalize-deposit-from-eth:
	yarn hardhat --network ${NETWORK} near-finalize-deposit-from-eth --tx-hash ${TX_HASH} --near-account ${NEAR_ACCOUNT} --near-recipient ${NEAR_RECIPIENT} --deposited-to-near true

near-finalize-deposit-from-eth-to-evm:
	yarn hardhat --network ${NETWORK} near-finalize-deposit-from-eth --tx-hash ${TX_HASH} --near-account ${NEAR_ACCOUNT} --eth-recipient ${ETH_RECIPIENT}

near-ft-balance-of:
	yarn hardhat near-ft-balance-of --near-account ${NEAR_ACCOUNT} --query-near-account ${QUERY_NEAR_ACCOUNT}

near-ft-balance-of-eth:
	yarn hardhat near-ft-balance-of-eth --near-account ${NEAR_ACCOUNT} --eth-address ${ETH_ADDRESS}

near-withdraw-bridged-eth:
	yarn hardhat --network ${NETWORK} near-withdraw-bridged-eth \
		--near-account ${NEAR_ACCOUNT} --eth-recipient ${ETH_RECIPIENT} --amount ${AMOUNT} --fee ${FEE}

near-get-tx-status:
	/usr/bin/http post ${RPC_ENDPOINT} jsonrpc=2.0 method=EXPERIMENTAL_tx_status \
		params:='["${TX_HASH}", "${NEAR_ACCOUNT}"]' id=dontcare

_near-deposit:
	near call ${NEAR_EVM_ACCOUNT} deposit --account-id ${NEAR_ACCOUNT} --gas 300000000000000 '{"proof": ${PROOF}}'

_near-deposit-to-nostd:
	near call ${NEAR_EVM_ACCOUNT} deposit '${PROOF}' --account-id ${NEAR_ACCOUNT} --gas 300000000000000

_near-call-withdraw-from:
	near call ${NEAR_EVM_ACCOUNT} withdraw --account-id ${NEAR_ACCOUNT} \
		'{ "recipient_id": "${ETH_RECIPIENT}", "amount": "${AMOUNT}" }'

_near-call-withdraw-from-nostd:
	near call ${NEAR_EVM_ACCOUNT} withdraw --account-id ${NEAR_ACCOUNT} \
		'{ "recipient_id": "${ETH_RECIPIENT}", "amount": ${AMOUNT} }'

aurora-init-eth-connector:
	yarn hardhat aurora-init-eth-connector --near-account ${NEAR_ACCOUNT}

aurora-register-relayer:
	yarn hardhat aurora-register-relayer --near-account ${RELAYER_NEAR_ADDRESS} --relayer-address-in-aurora ${RELAYER_ADDRESS_IN_AURORA}
