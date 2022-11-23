use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::datamem::DataMem;
use crate::instrmem::InstrMem;
use crate::retaddrstack::RetAddrStack;
use crate::rtstack::RTStack;
use crate::stmt::Stmt;
use crate::strbuf::StrBuf;

pub struct VM {
    pc: i32,
    rel_path: String,
    bin_file: Option<File>,
    out_file: Option<File>,
    data_mem: Option<DataMem>,
    instr_mem: Option<InstrMem>,
    ret_addr_stack: RetAddrStack,
    rt_stack: RTStack,
    str_buf: Option<StrBuf>,
    fn_map_add: HashMap<i32, fn(&mut VM)>,
    fn_map_exec: HashMap<i32, fn(&mut VM)>,
}

impl VM {
    pub fn new(path: String) -> VM {
        VM {
            pc: 0,
            rel_path: path,
            bin_file: None,
            out_file: None,
            data_mem: None,
            instr_mem: None,
            ret_addr_stack: RetAddrStack::new(),
            rt_stack: RTStack::new(),
            str_buf: None,
            fn_map_add: HashMap::new(),
            fn_map_exec: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        self.fill_instr_mem();
        self.exec_instr_mem();
    }

    fn add_opcode(&mut self, exec: bool, opcode: i32, fptr: fn(&mut VM)) {
        if exec == false {
            self.fn_map_add.insert(opcode, fptr);
        } else {
            self.fn_map_exec.insert(opcode, fptr);
        }
    }

    fn exec_instr_mem(&mut self) {
        self.out_file = Some(File::create(self.get_out_filepath())
            .expect("error: invalid output path"));
        
        self.fill_fn_map_exec();
        println!();

        while self.pc < self.instr_mem.as_ref().unwrap().get_size() as i32 {
            println!("PC: {}", self.pc);
            self.handle_opcode(true, self.instr_mem.as_ref()
                .unwrap().get_instr(self.pc as usize).get_opcode());
        }
    }

    fn fill_fn_map_add(&mut self) {
        self.add_opcode(false, 0x00000017, VM::add_start_prog_instr);
        self.add_opcode(false, 0x00000018, VM::add_exit_instr);
        self.add_opcode(false, 0x00000022, VM::add_pushi_instr);
        self.add_opcode(false, 0x00000061, VM::add_print_tos_instr);
        self.add_opcode(false, 0x00000050, VM::add_add_instr);
        self.add_opcode(false, 0x00000060, VM::add_prints_instr);
        self.add_opcode(false, 0x00000053, VM::add_div_instr);
        self.add_opcode(false, 0x00000040, VM::add_dup_instr);
        self.add_opcode(false, 0x00000052, VM::add_mul_instr);
        self.add_opcode(false, 0x00000051, VM::add_negate_instr);
        self.add_opcode(false, 0x00000032, VM::add_pop_instr);
        self.add_opcode(false, 0x00000041, VM::add_swap_instr);
        self.add_opcode(false, 0x00000010, VM::add_jump_instr);
        self.add_opcode(false, 0x00000013, VM::add_gosub_instr);
        self.add_opcode(false, 0x00000014, VM::add_return_instr);
        self.add_opcode(false, 0x00000015, VM::add_enter_sub_instr);
        self.add_opcode(false, 0x00000030, VM::add_pop_scalar_instr);
    }

    fn fill_fn_map_exec(&mut self) {
        self.add_opcode(true, 0x00000017, VM::exec_start_prog_instr);
        self.add_opcode(true, 0x00000018, VM::exec_exit_prog_instr);
        self.add_opcode(true, 0x00000022, VM::exec_pushi_instr);
        self.add_opcode(true, 0x00000061, VM::exec_print_tos_instr);
        self.add_opcode(true, 0x00000050, VM::exec_add_instr);
        self.add_opcode(true, 0x00000060, VM::exec_prints_instr);
        self.add_opcode(true, 0x00000053, VM::exec_div_instr);
        self.add_opcode(true, 0x00000040, VM::exec_dup_instr);
        self.add_opcode(true, 0x00000052, VM::exec_mul_instr);
        self.add_opcode(true, 0x00000051, VM::exec_negate_instr);
        self.add_opcode(true, 0x00000032, VM::exec_pop_instr);
        self.add_opcode(true, 0x00000041, VM::exec_swap_instr);
        self.add_opcode(true, 0x00000010, VM::exec_jump_instr);
        self.add_opcode(true, 0x00000013, VM::exec_gosub_instr);
        self.add_opcode(true, 0x00000014, VM::exec_return_instr);
        self.add_opcode(true, 0x00000015, VM::exec_enter_sub_instr);
    }

    fn fill_instr_mem(&mut self) { 
        self.bin_file = Some(File::open(&self.rel_path)
            .expect("error: invalid file"));
        
        let mut buf = [0; 4];

        self.bin_file.as_ref().unwrap().read(&mut buf[..])
            .expect("error: issue reading data memory size");
        let data_mem_size = i32::from_le_bytes(buf) as usize;
        self.data_mem = Some(DataMem::new(data_mem_size));
        println!("Data memory size: {}", data_mem_size);

        self.bin_file.as_ref().unwrap().read(&mut buf[..])
            .expect("error: issue reading instruction memory size");
        let instr_mem_size = i32::from_le_bytes(buf) as usize;
        self.instr_mem = Some(InstrMem::new(instr_mem_size));
        println!("Instruction memory size: {}", instr_mem_size);
        
        self.fill_fn_map_add();

        let mut opcode: i32;
        for _idx in 0..self.instr_mem.as_ref().unwrap().get_size() {
            self.bin_file.as_ref().unwrap().read(&mut buf[..])
                .expect("error: issue reading opcode");
            opcode = i32::from_le_bytes(buf);
            self.handle_opcode(false, opcode);
        }

        self.bin_file.as_ref().unwrap().read(&mut buf[..])
            .expect("error: issue reading string buffer size");
        let str_buf_size = i32::from_le_bytes(buf) as usize;
        self.str_buf = Some(StrBuf::new(str_buf_size));

        if str_buf_size > 0 {
            let mut str_size: usize;
            let mut str: String;
            for idx in 0..self.str_buf.as_ref().unwrap().get_size() {
                let mut buf: [u8; 4] = [0; 4];
                self.bin_file.as_ref().unwrap().read(&mut buf[..])
                    .expect("error: issue reading string buffer size");
                str_size = i32::from_le_bytes(buf) as usize;
                let mut buf: Vec<u8> = Vec::new();
                buf.resize(str_size, 0);
                self.bin_file.as_ref().unwrap().read(&mut buf[..])
                    .expect("error: issue reading string into buffer");
                str = String::from_utf8(buf)
                    .expect("error: issue converting buffer to string");
                self.str_buf.as_mut().unwrap().insert_at(idx, str);
            }
        }
    }

    fn get_out_filepath(&self) -> String{
        let rel_path_str = self.rel_path.clone();
        let base_filename_pos = rel_path_str.rfind("/");
        let base_filename: &str;

        match base_filename_pos {
            Some(pos) => {
                base_filename = &rel_path_str[pos + 1..];
            } None => {
                base_filename = &rel_path_str[..];
            }
        }

        let vm_out_filename_pos = base_filename.rfind(".").unwrap();
        base_filename[0..vm_out_filename_pos].to_owned() + ".vout"
    }

    fn handle_opcode(&mut self, exec: bool, opcode: i32) {
        if exec == false {
            let fptr = self.fn_map_add.get(&opcode);
            match fptr {
                Some(f) => f(self),
                None => panic!("error: unsupported opcode {} provided", opcode),
            }
        } else {
            let fptr = self.fn_map_exec.get(&opcode);
            match fptr {
                Some(f) => f(self),
                None => panic!("error: unsupported opcode {} provided", opcode),
            }
        }
    }

    fn add_instr_with_operand(&mut self, opcode: i32, name: String) {
        let mut buf: [u8; 4] = [0; 4];
        self.bin_file.as_ref().unwrap().read(&mut buf[..])
            .expect("error: issue reading operand");
        let operand = i32::from_le_bytes(buf);
        let pos = self.instr_mem.as_ref().unwrap()
            .get_curr_pos() as usize;
        let mut instr = Box::new(Stmt::new());
        instr.set_opcode(opcode);
        instr.set_operand(operand);
        instr.set_name(name);
        self.instr_mem.as_mut().unwrap().insert_at(pos, instr);
        self.instr_mem.as_mut().unwrap().inc_curr_pos(1);
    }

    fn add_instr_without_operand(&mut self, opcode: i32, name: String) {
        let pos = self.instr_mem.as_ref().unwrap()
            .get_curr_pos() as usize;
        let mut instr = Box::new(Stmt::new());
        instr.set_opcode(opcode);
        instr.set_name(name);
        self.instr_mem.as_mut().unwrap().insert_at(pos, instr);
        self.instr_mem.as_mut().unwrap().inc_curr_pos(1);
    }

    fn add_start_prog_instr(&mut self) {
        self.add_instr_with_operand(0x00000017, 
            String::from("OP_START_PROGRAM"));
        println!("OP_START_PROGRAM added");
    }

    fn add_exit_instr(&mut self) {
        self.add_instr_without_operand(0x00000018, 
            String::from("OP_EXIT"));
        println!("OP_EXIT added");
    }

    fn add_pushi_instr(&mut self) {
        self.add_instr_with_operand(0x00000022, 
            String::from("OP_PUSHI"));
        println!("OP_PUSHI added");
    }

    fn add_print_tos_instr(&mut self) {
        self.add_instr_without_operand(0x00000061, 
            String::from("OP_PRINTTOS"));
        println!("OP_PRINTTOS added");
    }

    fn add_add_instr(&mut self) {
        self.add_instr_without_operand(0x00000050, 
            String::from("OP_ADD"));
        println!("OP_ADD added");
    }

    fn add_prints_instr(&mut self) {
        self.add_instr_with_operand(0x00000060, 
            String::from("OP_PRINTS"));
        println!("OP_PRINTS added");
    }

    fn add_div_instr(&mut self) {
        self.add_instr_without_operand(0x00000053, 
            String::from("OP_DIV"));
        println!("OP_DIV added");
    }

    fn add_dup_instr(&mut self) {
        self.add_instr_without_operand(0x00000040, 
            String::from("OP_DUP"));
        println!("OP_DUP added");
    }

    fn add_mul_instr(&mut self) {
        self.add_instr_without_operand(0x00000052, 
            String::from("OP_MUL"));
        println!("OP_MUL added");
    }

    fn add_negate_instr(&mut self) {
        self.add_instr_without_operand(0x00000051, 
            String::from("OP_NEGATE"));
        println!("OP_NEGATE added");
    }

    fn add_pop_instr(&mut self) {
        self.add_instr_without_operand(0x00000032, 
            String::from("OP_POP"));
        println!("OP_POP added");
    }

    fn add_swap_instr(&mut self) {
        self.add_instr_without_operand(0x00000041, 
            String::from("OP_SWAP"));
        println!("OP_SWAP added");
    }

    fn add_jump_instr(&mut self) {
        self.add_instr_with_operand(0x00000010, 
            String::from("OP_JUMP"));
        println!("OP_JUMP added");
    }

    fn add_gosub_instr(&mut self) {
        self.add_instr_with_operand(0x00000013, 
            String::from("OP_GOSUB"));
        println!("OP_GOSUB added");
    }

    fn add_return_instr(&mut self) {
        self.add_instr_without_operand(0x00000014, 
            String::from("OP_RETURN"));
        println!("OP_RETURN added");
    }

    fn add_enter_sub_instr(&mut self) {
        self.add_instr_with_operand(0x00000015, 
            String::from("OP_ENTER_SUBROUTINE"));
        println!("OP_ENTER_SUBROUTINE added");
    }

    fn add_pop_scalar_instr(&mut self) {
        self.add_instr_with_operand(0x00000030, 
            String::from("OP_POPSCALAR"));
        println!("OP_POPSCALAR added");
    }

    fn exec_start_prog_instr(&mut self) {
        self.pc += 1;
        println!("OP_START_PROGRAM executed");
    }

    fn exec_exit_prog_instr(&mut self) {
        self.pc += 1;
        println!("OP_EXIT executed");
    }

    fn exec_pushi_instr(&mut self) {
        let operand = self.instr_mem.as_ref().unwrap().
            get_instr(self.pc as usize).get_operand();
        self.rt_stack.push_val(operand);
        self.pc += 1;
        println!("OP_PUSHI executed");
    }

    fn exec_print_tos_instr(&mut self) {
        let top = self.rt_stack.top_val();
        self.out_file.as_ref().unwrap().write(&top.to_string().as_bytes())
            .expect("error: issue writing to output file");
        self.out_file.as_ref().unwrap().write("\n".as_bytes())
            .expect("error: issue writing to output file");
        self.rt_stack.pop_val();
        self.pc += 1;
        println!("OP_PRINTTOS executed");
    }

    fn exec_add_instr(&mut self) {
        let addend0 = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        let addend1 = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        self.rt_stack.push_val(addend0 + addend1);
        self.pc += 1;
        println!("OP_ADD executed");
    }

    fn exec_prints_instr(&mut self) {
        let operand = self.instr_mem.as_ref().unwrap()
            .get_instr(self.pc as usize).get_operand() as usize;
        let str = self.str_buf.as_ref().unwrap().get_str(operand);
        self.out_file.as_ref().unwrap().write(str.as_bytes())
            .expect("error: issue writing to output file");
        self.out_file.as_ref().unwrap().write("\n".as_bytes())
            .expect("error: issue writing to output file");
        self.pc += 1;
        println!("OP_PRINTS executed");
    }

    fn exec_div_instr(&mut self) {
        let dividend = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        let divisor = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        self.rt_stack.push_val(dividend / divisor);
        self.pc += 1;
        println!("OP_DIV executed");
    }

    fn exec_dup_instr(&mut self) {
        let top = self.rt_stack.top_val();
        self.rt_stack.push_val(top);
        self.pc += 1;
        println!("OP_DUP executed");
    }

    fn exec_mul_instr(&mut self) {
        let multiplier = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        let multiplicand = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        self.rt_stack.push_val(multiplier * multiplicand);
        self.pc += 1;
        println!("OP_MUL executed");
    }

    fn exec_negate_instr(&mut self) {
        let negate = 0 - self.rt_stack.top_val();
        self.rt_stack.pop_val();
        self.rt_stack.push_val(negate);
        self.pc += 1;
        println!("OP_NEGATE executed");
    }

    fn exec_pop_instr(&mut self) {
        self.rt_stack.pop_val();
        self.pc += 1;
        println!("OP_POP executed");
    }

    fn exec_swap_instr(&mut self) {
        let old_top = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        let new_top = self.rt_stack.top_val();
        self.rt_stack.pop_val();
        self.rt_stack.push_val(old_top);
        self.rt_stack.push_val(new_top);
        self.pc += 1;
        println!("OP_SWAP executed");
    }

    fn exec_jump_instr(&mut self) {
        let new_pc = self.instr_mem.as_ref().unwrap()
            .get_instr(self.pc as usize).get_operand();
        self.pc = new_pc;
        println!("OP_JUMP executed");
    }

    fn exec_gosub_instr(&mut self) {
        self.ret_addr_stack.push_addr(self.pc + 1);
        let new_pc = self.instr_mem.as_ref().unwrap()
            .get_instr(self.pc as usize).get_operand();
        self.pc = new_pc;
        println!("OP_GOSUB executed");
    }

    fn exec_return_instr(&mut self) {
        let new_pc = self.ret_addr_stack.top_addr();
        self.pc = new_pc;
        self.data_mem.as_mut().unwrap().pop_sub_mem();
        self.ret_addr_stack.pop_addr();
        println!("OP_RETURN executed");
    }

    fn exec_enter_sub_instr(&mut self) {
        let operand = self.instr_mem.as_ref().unwrap()
            .get_instr(self.pc as usize).get_operand();
        self.data_mem.as_mut().unwrap().push_sub_mem(operand as usize);
        self.pc += 1;
        println!("OP_ENTER_SUBROUTINE executed");
    }

    fn exec_pop_scalar_instr(&mut self) {
        let operand = self.instr_mem.as_ref().unwrap()
            .get_instr(self.pc as usize).get_operand();
        let top = self.rt_stack.top_val();
        self.data_mem.as_mut().unwrap().set_data(operand as usize, top);
        self.rt_stack.pop_val();
        self.pc += 1;
        println!("OP_POPSCALAR executed")
    }
}