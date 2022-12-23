use std::fs::File;

use super::state::State;

pub struct RuaVM {
    state: State,
}

impl RuaVM {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    pub fn load_bytecode_file(&mut self, path: &str) -> &mut RuaVM {
        match File::open(path) {
            Ok(file) => {
                
            }
            Err(e) => println!("{?}", e),
        }

        self
    }

    fn load_bytecode(&mut self, bytecode: &[u8]) -> &mut Self {
        unimplemented!()
    }
}
