#[candid::candid_method(query)]
#[ic_cdk::query]
fn canister_1_greet() -> &'static str {
    "Hello from canister 1"
}

#[cfg(test)]
mod test;

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}
