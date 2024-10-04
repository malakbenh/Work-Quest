use crate::{RUNTIME_STATE, Data};
use ic_cdk_macros::{pre_upgrade, post_upgrade};
use std::cell::RefCell;


#[pre_upgrade]
fn pre_upgrade() {
    // Store the current state to stable memory before upgrade
    RUNTIME_STATE.with(|state| {
        let state = state.borrow();
        
        // Serialize the current state
        let state_data = bincode::serialize(&state.data).expect("Failed to serialize state data");
        
        // Save to stable memory
        ic_cdk::api::stable::set_stable_memory(&state_data);
    });
}