use crate::{util::get_current_system_time_from_ic, CANISTER_DATA};

#[candid::candid_method(update)]
#[ic_cdk::update]
fn save_canister_wasm(canister_wasm: Vec<u8>) -> Result<(), &'static str> {
    let current_time = get_current_system_time_from_ic();

    CANISTER_DATA.with(|canister_data_ref_cell| {
        let mut canister_data = canister_data_ref_cell.borrow_mut();
        canister_data
            .canister_store
            .user_index
            .wasm_store
            .insert(current_time, canister_wasm);
    });

    Ok(())
}
