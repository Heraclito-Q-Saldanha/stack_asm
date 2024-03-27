use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::Push(Value::Inline(0)),
        Instruction::Exit
    ];
    let program = compile(&Targets::X86_64, &code);
    fs::write("./out", program).unwrap();
}
