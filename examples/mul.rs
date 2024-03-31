use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::Push(Value::Inline(10)),
        Instruction::Push(Value::Inline(2)),
        Instruction::Mul,
        Instruction::Exit
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
