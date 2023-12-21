#![no_std]
#![no_main]

mod error;

use core::convert::TryInto;

pub use error::NexfiDropError;

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

// Importing Rust types.
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::BTreeMap,
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
    account::AccountHash,
    api_error::ApiError,
    bytesrepr::ToBytes,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, ContractHash, Key, Parameter, RuntimeArgs, URef, U256,
    U512,
};

use casper_types::CLType::Map;

// Creating constants for values within the contract package.
const CONTRACT_PACKAGE_NAME: &str = "erc20_bank_package_name";
const CONTRACT_ACCESS_UREF: &str = "erc20_bank_access_uref";

const DATE_FROM_NAME: &str = "date_from";
const DATE_TO_NAME: &str = "date_to";
const STATUS_NAME: &str = "status";

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
            // Parameter::new("amount", CLType::U512),
            // Parameter::new("contract_hash", CLType::Key),
            // Parameter::new("address", CLType::Key),
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
        vec![Parameter::new(
            "balances",
            CLType::Map {
                key: Box::new(CLType::String),
                value: Box::new(CLType::U128),
            },
        )],
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(init_point);

    let mut named_keys = NamedKeys::new();

    let date_from_name = String::from(DATE_FROM_NAME);
    let date_to_name = String::from(DATE_TO_NAME);
    let status_name = String::from(DATE_TO_NAME);

    named_keys.insert(date_from_name, storage::new_uref("01.12.2023").into());
    named_keys.insert(date_to_name, storage::new_uref("05.12.2023").into());
    named_keys.insert(status_name, storage::new_uref("RUN").into());

    // новый контракт
    // let (stored_contract_hash, contract_version) = storage::new_contract(
    //     entry_points,
    //     Some(named_keys),
    //     Some(CONTRACT_PACKAGE_NAME.to_string()),
    //     Some(CONTRACT_ACCESS_UREF.to_string()),
    // );

    // или новая версия

    let bank_package_hash = runtime::get_key(CONTRACT_PACKAGE_NAME)
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();

    let (stored_contract_hash, contract_version) = storage::add_contract_version(
        bank_package_hash,
        entry_points,
        Some(named_keys).unwrap(), // Some(CONTRACT_PACKAGE_NAME.to_string()),
    );

    // Store the contract version in the context's named keys.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());

    let mut init_args = runtime_args! {};

    // сhagne context
    runtime::call_contract::<()>(stored_contract_hash, INIT_ENTRY_POINT_NAME, init_args);
}

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    // тут инициализируем карту балансов (не проверял)
    if !get_key(DROP_BALANCES).is_some() {
        // revert(NexfiDropError::AlreadyInitialized);

        let drop_balances_uref = storage::new_dictionary(DROP_BALANCES).unwrap_or_revert();

        // раздать кому-нибудь чето при деплое
        // let sender = runtime::get_caller().to_string();
        // storage::dictionary_put(drop_balances_uref, &sender, 99);

        // let my_acc = "f4c10aeb89a75d41b5545bb4183940dba4eb2e6071015da1f61e4c2a822d1c43";
        // storage::dictionary_put(drop_balances_uref, &my_acc, 299);
    }
}
#[no_mangle]
pub extern "C" fn set_drop_balances() {
    let balances = runtime::get_named_arg::<BTreeMap<String, i32>>("balances");

    let key = runtime::get_key(DROP_BALANCES)
        .ok_or(NexfiDropError::MissingKey)
        .unwrap_or_revert()
        .as_uref()
        .unwrap_or_revert()
        .to_owned();

    set_key(
        "participants",
        balances.keys().cloned().collect::<Vec<String>>(),
    );

    for (addr, balance) in &balances {
        storage::dictionary_put(key, &addr, balance.to_owned());
    }
}

/*
* TODO: добавпить проверки на баланс
*/
#[no_mangle]
pub extern "C" fn withdraw() {
    let participants_key = runtime::get_key("participants")
        .ok_or(NexfiDropError::MissingKey)
        .unwrap_or_revert()
        .as_uref()
        .unwrap_or_revert()
        .to_owned();

    // let dictionary_key = runtime::get_key("participants")
    //     .ok_or(NexfiDropError::CantReadUref)
    //     .unwrap_or_revert();

    // storage::dictionary_read::<String>(dictionary_key)

    // let list_key = runtime::get_key("participants")
    //     .unwrap_or_revert_with(NexfiDropError::CantReadUref);

    // let accounts_list =
    //     storage::read_from_key::<Vec::<String>>(list_key)
    //     .unwrap_or_revert_with(NexfiDropError::CantReadBalances).unwrap();

    /*
    for(addr) in accounts_list {
        let dropbalances_key = runtime::get_key(DROP_BALANCES)
            .ok_or(NexfiDropError::MissingKey)
            .unwrap_or_revert()
            .as_uref()
            .unwrap_or_revert()
            .to_owned();

        let balance = storage::dictionary_get::<i32>(dropbalances_key, &addr)
            .unwrap_or_revert_with(NexfiDropError::CantReadBalances)
            .unwrap_or_revert_with(NexfiDropError::CantReadBalances);

        let token_contract: ContractHash = ContractHash::new(
            runtime::get_named_arg::<Key>("contract_hash")
                .into_hash()
                .unwrap_or_revert(),
        );

        let args = runtime_args! {
            "amount" => balance,
            "recipient" => addr
        };

        let _: () = runtime::call_contract(token_contract, "transfer", args);
    }
    */

    /*
    let key = runtime::get_key(DROP_BALANCES)
        .ok_or(NexfiDropError::MissingKey)
        .unwrap_or_revert();

    let balances = storage::dictionary_read::<BTreeMap<String, i32>>(key)
        .unwrap_or_revert_with(NexfiDropError::CantReadBalances)
        .unwrap();

    let token_contract: ContractHash = ContractHash::new(
        runtime::get_named_arg::<Key>("contract_hash")
            .into_hash()
            .unwrap_or_revert(),
    );

    for (address, amount) in balances {
        let args = runtime_args! {
            "amount" => amount,
            "recipient" => address
        };

        let _: () = runtime::call_contract(token_contract, "transfer", args);
    }
    */

    // withdraw func
    // let amount = runtime::get_named_arg::<U512>("amount");
    // let token_contract: ContractHash = ContractHash::new(
    //     runtime::get_named_arg::<Key>("contract_hash")
    //         .into_hash()
    //         .unwrap_or_revert(),
    // );
    // let address: Key = runtime::get_named_arg("address");

    // let args = runtime_args! {
    //     "amount" => amount,
    //     "recipient" => address

    // };

    // let _: () = runtime::call_contract(token_contract, "transfer", args);
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}
