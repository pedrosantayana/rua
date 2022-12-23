use std::default;

use derive_builder::Builder;
use crate::vm::state::State;

pub struct RuaState {
    
}

impl RuaState {
    pub fn new() -> Self {
        Self {
            state: State::new()
        }
    }
}