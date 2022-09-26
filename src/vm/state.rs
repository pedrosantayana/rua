trait TValue {
    
}

struct Stack {
  vec: Vec<Box<dyn TValue>>,

}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            vec: Vec::new()
        }
    }
}

struct Callinfo;

impl Callinfo {
    fn new() -> Callinfo {
        unimplemented!()
    }
}

pub struct State {
  stack: Stack,
  ci: Callinfo
}

impl State {
  pub fn new() -> State {
    State {
      stack: Stack::new(),
        ci: Callinfo::new()
    }
  }


}