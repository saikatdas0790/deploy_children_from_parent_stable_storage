use self::canister_store::CanisterStore;

pub mod canister_store;
pub mod wasm_store;

#[derive(Default)]
pub struct CanisterData {
    pub canister_store: CanisterStore,
}
