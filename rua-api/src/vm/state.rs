use super::gc::GCObject;



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

enum StateStatus {

}

pub struct RuaState {
    header: Header,
    status: unimplemented!("enum with different status"),
    call_info_stack: Vec<CallInfo>,
    register_stack: Vec<Register>,
    gc_list: Vec<GCObject>,

}

impl State {
    pub fn new() -> State {
        State {
            stack: Stack::new(),
            ci: Vec::new(),
        }
    }
}
