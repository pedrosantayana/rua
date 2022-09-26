use std::{fs::File, io::Read};

use super::state::State;

pub struct RuaVM {
  state: State,

}

impl RuaVM {
  pub fn new() -> RuaVM {
    RuaVM {
      state: State::new()
    }
  }

  pub fn run_bytecode_file(&mut self, path: &str) -> &mut RuaVM {
    let mut file = File::open(path).unwrap();

    let a = file.bytes();


    
    self
  }

  fn decode_bytecode() -> {
    unimplemented!()
  }
}