use near_sdk_sim::{
    call, deploy, init_simulator, to_yocto, view, ContractAccount, UserAccount, DEFAULT_GAS,
    STORAGE_AMOUNT,
};

extern crate eth_connector;
use eth_connector::{EthConnectorContract, Proof};

near_sdk_sim::lazy_static! {
    static ref TOKEN_WASM_BYTES: &'static [u8] = include_bytes!("../res/eth_connector.wasm").as_ref();
}

fn init() -> (
    UserAccount,
    ContractAccount<EthConnectorContract>,
    UserAccount,
) {
    let master_account = init_simulator(None);
    let eth_ecc = "79183fdbd80e2d8AeA1aCaA2f67bFb8a36d40A80";
    let initial_balance = to_yocto("100000");

    let contract_account = deploy! {
        contract: EthConnectorContract,
        contract_id: "contract",
        bytes: &TOKEN_WASM_BYTES,
        signer_account: master_account,
        init_method: new(master_account.account_id(), eth_ecc.into())
    };
    let alice = master_account.create_user("alice".to_string(), initial_balance);
    (master_account, contract_account, alice)
}

#[test]
fn init_test() {
    let (_master_account, _contract_account, _alice) = init();
}

#[test]
fn test_sim_deposit() {
    let (master_account, contract, _alice) = init();
/*
    // Deplot Eth Verifier Contract 
    let status_id = "acc1".to_string();
    let status_amt = to_yocto("100");
    let res = call!(
        master_account,
        contract.deploy(status_id.clone(), status_amt.into()),
        STORAGE_AMOUNT * 1000,
        DEFAULT_GAS
    );
    println!("CALL: {:#?}", res);
    println!("#2: {:#?}", res.promise_results());
*/    
    let proof = Proof {
        log_index: 0,
        log_entry_data: vec![],
        receipt_index: 0,
        receipt_data: vec![],
        header_data: vec![],
        proof: vec![],
    };
    let res = call!(
        master_account,
        contract.deposit(proof.clone()),
        gas = DEFAULT_GAS * 3
    );

    let promise_outcomes = res.get_receipt_results();
    println!("CALL: {:#?}", res);
    println!("#1: {:#?}", promise_outcomes);
    println!("#2: {:#?}", res.promise_results());
}