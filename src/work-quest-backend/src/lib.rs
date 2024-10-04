use crate::env::{CanisterEnvironment, Environment};
use crate::ledger::{LedgerCanister, Ledger};
use crate::model::jobs::Jobs;
use crate::model::clients::Clients;
use crate::model::freelancers::Freelancers;
use candid::{CandidType, Principal};
use serde::Deserialize;
use std::cell::RefCell;
use std::sync::Arc;

mod env;
mod ledger;
mod lifecycle;
// mod model;
// mod queries;
// mod updates;

thread_local! {
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

pub struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
    pub ledger: Arc<dyn Ledger>,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data, ledger: Arc<dyn Ledger>) -> RuntimeState {
        RuntimeState { env, data, ledger }
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(CanisterEnvironment {}),
            data: Data::default(),
            ledger: Arc::new(LedgerCanister {}),
        }
    }
}

#[derive(CandidType, Deserialize, Default)]
pub struct Data {
    jobs: Jobs,
    freelancers: Freelancers,
    clients: Clients,
}
