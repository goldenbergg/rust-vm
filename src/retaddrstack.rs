pub struct RetAddrStack {
    def_stack: Vec<i32>,
}

impl RetAddrStack {
    pub fn new() -> RetAddrStack {
        RetAddrStack { 
            def_stack: Vec::new() 
        }
    }

    pub fn push_addr(&mut self, addr: i32) {
        self.def_stack.push(addr);
    }

    pub fn pop_addr(&mut self) {
        self.def_stack.pop();
    }

    pub fn top_addr(&self) -> i32 {
        self.def_stack.last().unwrap().clone()
    }
}