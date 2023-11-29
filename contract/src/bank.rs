#![no_std]
#![no_main]

mod error;

pub use error::NexfiDropError;

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

// Importing Rust types.
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use alloc::vec;

use casper_contract::{
    contract_api::{
        runtime::{self, get_key, get_named_arg, put_key, revert},
        storage, system,
    },
    unwrap_or_revert::UnwrapOrRevert,
};

// Importing specific Casper types.
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLValue, ContractHash, Key, Parameter, RuntimeArgs, URef, U256, U512, account::AccountHash,
};

// Creating constants for values within the contract package.
const CONTRACT_PACKAGE_NAME: &str = "erc20_bank_package_name";
const CONTRACT_ACCESS_UREF: &str = "erc20_bank_access_uref";

// Creating constants for the various contract entry points.
const ENTRY_POINT_WITHRDRAW: &str = "withdraw";

// Creating constants for values within the contract.
const CONTRACT_VERSION_KEY: &str = "bank_version";
const CONTRACT_KEY: &str = "erc20_bank";
const DROP_BALANCES: &str = "drop_balances_cntrct";
const INIT_ENTRY_POINT_NAME: &str = "init";
pub const PACKAGE_HASH: &str = "erc20_bank_package_hash";

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_WITHRDRAW,
        vec![
            Parameter::new("amount", CLType::U512),
            Parameter::new("contract_hash", CLType::Key),
            Parameter::new("address", CLType::Key),
        ],
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let init_point = EntryPoint::new(
        INIT_ENTRY_POINT_NAME,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );


    
    entry_points.add_entry_point(EntryPoint::new(
        "set_drop_balances",
        vec![
            Parameter::new("balances", CLType::Map {
                key: Box::new(CLType::String),
                value: Box::new(CLType::U128),
            })
        ],
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));


    entry_points.add_entry_point(init_point);

    let mut named_keys = NamedKeys::new();

    let bank_package_hash = runtime::get_key(CONTRACT_PACKAGE_NAME)
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();

    // Create a new contract package that can be upgraded.
    let (stored_contract_hash, contract_version) = storage::add_contract_version(
        bank_package_hash,
        entry_points,
        Some(named_keys).unwrap(), // Some(CONTRACT_PACKAGE_NAME.to_string()),
                                   // Some(CONTRACT_ACCESS_UREF.to_string()),
    );

    // Store the contract version in the context's named keys.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());

    let mut init_args = runtime_args! {
    };

    // сhagne context
    runtime::call_contract::<()>(stored_contract_hash, INIT_ENTRY_POINT_NAME, init_args);
}

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    if get_key(DROP_BALANCES).is_some() {
        revert(NexfiDropError::AlreadyInitialized);
    }


    let drop_balances_uref = storage::new_dictionary(DROP_BALANCES).unwrap_or_revert();
    let sender = runtime::get_caller().to_string();
    storage::dictionary_put(drop_balances_uref, &sender, 99);
    
    let my_acc = "f4c10aeb89a75d41b5545bb4183940dba4eb2e6071015da1f61e4c2a822d1c43";
    storage::dictionary_put(drop_balances_uref, &my_acc, 299);
}
#[no_mangle]
pub extern "C" fn set_drop_balances() {

}


/* 
* TODO: добавпить проверки на баланс
*/
#[no_mangle]
pub extern "C" fn withdraw() {
    let amount = runtime::get_named_arg::<U512>("amount");
    let token_contract: ContractHash = ContractHash::new(
        runtime::get_named_arg::<Key>("contract_hash")
            .into_hash()
            .unwrap_or_revert(),
    );
    let address: Key = runtime::get_named_arg("address");

    let args = runtime_args! {
        "amount" => amount,
        "recipient" => address

    };

    let _: () = runtime::call_contract(token_contract, "transfer", args);
}
