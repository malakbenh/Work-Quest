use crate::{RUNTIME_STATE, Data};
use ic_cdk_macros::{pre_upgrade, post_upgrade};
use std::cell::RefCell;

#[post_upgrade]
fn post_upgrade() {
    // Restore the state from stable memory after upgrade
    RUNTIME_STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        // Retrieve the state data from stable memory
        let state_data: Vec<u8> = ic_cdk::api::stable::get_stable_memory();
        
        // Deserialize the state data
        let restored_data: Data = bincode::deserialize(&state_data).expect("Failed to deserialize state data");
        
        // Update the runtime state with the restored data
        state.data = restored_data;
    });
}