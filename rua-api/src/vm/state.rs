trait TValue {}

#[derive(Default, Clone)]
struct Stack {
    stack: Vec<Box<dyn TValue>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: Vec::new() }
    }
}


struct CallInfo{
    top: Box<dyn TValue>,
    base: Box<dyn TValue>,
    func: Box<dyn TValue>,
    prev: Box<CallInfo>
}

#[derive(Default, Clone)]
pub struct State {
    stack: Stack,
    ci: Vec<Box<CallInfo>>,
}

impl State {
    pub fn new() -> State {
        State {
            stack: Stack::new(),
            ci: Vec::new(),
        }
    }
}
