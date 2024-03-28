use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("LOOP"),
        Instruction::Push(Value::LabelReference("LOOP")),
        Instruction::Jmp
    ];
    let program = compile(&Targets::X86_64, &code);
    fs::write("./out", program).unwrap();
}
