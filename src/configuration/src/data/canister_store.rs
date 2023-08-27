use super::wasm_store::WasmStore;

#[derive(Default)]
pub struct CanisterStore {
    pub user_index: WasmStore,
}
