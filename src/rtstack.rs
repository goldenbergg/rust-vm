pub struct RTStack {
    def_stack: Vec<i32>,
}

impl RTStack {
    pub fn new() -> RTStack {
        RTStack {
            def_stack: Vec::new(),
        }
    }

    pub fn push_val(&mut self, val: i32) {
        self.def_stack.push(val);
    }

    pub fn pop_val(&mut self) {
        self.def_stack.pop();
    }

    pub fn top_val(&self) -> i32 {
        self.def_stack.last().unwrap().clone()
    }
}