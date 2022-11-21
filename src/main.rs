use std::env;
use vm::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("error: no input progam was provided");
    }

    let prog = args[1].clone();

    let mut vm = Box::new(VM::new(prog));
    
    vm.start();
}
