use std::default;

use derive_builder::Builder;
use crate::vm::state::State;


#[derive(Builder, Default)]
pub struct Rua {
    state: State
}