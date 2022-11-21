pub struct Stmt {
    name: Option<String>,
    opcode: Option<i32>,
    operand: Option<i32>,
}

impl Stmt {
    pub fn new() -> Stmt {
        Stmt {
            name: None,
            opcode: None,
            operand: None,
        }
    }

    pub fn get_opcode(&self) -> i32 {
        self.opcode.unwrap()
    }

    pub fn get_operand(&self) -> i32 {
        self.operand.unwrap()
    }

    pub fn set_name(&mut self, _name: String) {
        self.name = Some(_name);
    }

    pub fn set_opcode(&mut self, _opcode: i32) {
        self.opcode = Some(_opcode);
    }

    pub fn set_operand(&mut self, _operand: i32) {
        self.operand = Some(_operand);
    }

    pub fn dbg_ser(&self) -> String {
        let mut dbg_str = self.name.as_ref().unwrap().clone();
        dbg_str.push_str(", ");
        dbg_str.push_str(self.operand.as_ref().unwrap().to_string()[..]
            .as_ref());
        dbg_str
    }
}