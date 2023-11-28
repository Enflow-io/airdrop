#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use core::str::FromStr;

// Importing Rust types.
use alloc::{
    string::{String, ToString},
};


use alloc::vec;


use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};

// Importing specific Casper types.
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, URef, ContractHash, RuntimeArgs, U512, U256, Key, runtime_args, Parameter,
};

// Creating constants for values within the contract package.
const CONTRACT_PACKAGE_NAME: &str = "erc20_bank_package_name";
const CONTRACT_ACCESS_UREF: &str = "erc20_bank_access_uref";

// Creating constants for the various contract entry points.
const ENTRY_POINT_WITHRDRAW: &str = "withdraw";

// Creating constants for values within the contract.
const CONTRACT_VERSION_KEY: &str = "bank_version";
const CONTRACT_KEY: &str = "erc20_bank";

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_WITHRDRAW,
        vec!(
            Parameter::new("amount", CLType::U512),
            Parameter::new("contract_hash", CLType::Key),
            Parameter::new("address", CLType::Key)
        ),
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

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
        Some(named_keys).unwrap()
        // Some(CONTRACT_PACKAGE_NAME.to_string()),
        // Some(CONTRACT_ACCESS_UREF.to_string()),
    );


      // Store the contract version in the context's named keys.
      let version_uref = storage::new_uref(contract_version);
      runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());
  
      // Create a named key for the contract hash.
      runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());


}

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
