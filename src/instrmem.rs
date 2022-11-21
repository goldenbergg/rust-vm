use crate::stmt::Stmt;
pub struct InstrMem {
    curr_pos: i32,
    def_mem: Vec<Box<Stmt>>,
}

impl InstrMem {
    pub fn new(size: usize) -> InstrMem {
        InstrMem {
            curr_pos: 0,
            def_mem: Vec::with_capacity(size),
        }
    }

    pub fn get_instr(&self, pos: usize) -> &Box<Stmt> {
        &self.def_mem[pos]
    }

    pub fn get_curr_pos(&self) -> i32 {
        self.curr_pos.clone()
    }

    pub fn get_size(&self) -> usize {
        self.def_mem.capacity()
    }

    pub fn inc_curr_pos(&mut self, inc: i32) {
        self.curr_pos += inc;
    }

    pub fn insert_at(&mut self, pos: usize, instr: Box<Stmt>) {
        self.def_mem.insert(pos, instr);
    }
}