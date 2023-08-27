use std::{collections::BTreeMap, time::SystemTime};

use candid::Principal;

pub type InsertionDate = SystemTime;
pub type CanisterWasm = Vec<u8>;

#[derive(Default)]
pub struct WasmStore {
    pub canister_id: Option<Principal>,
    pub wasm_store: BTreeMap<InsertionDate, CanisterWasm>,
}
