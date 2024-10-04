use crate::env::CanisterEnvironment;
use crate::ledger::LedgerCanister;
use crate::{Data, RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::init;
use std::sync::Arc;

#[init]
fn init() {
    // Setup canister runtime environment
    ic_cdk::setup();

    let env = CanisterEnvironment {};
    let data = Data::default();  // Initialize default data structure
    let ledger = LedgerCanister {};  // Initialize the ledger for transactions
    let runtime_state = RuntimeState::new(Box::new(env), data, Arc::new(ledger));

    // Store the runtime state for later use
    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}
