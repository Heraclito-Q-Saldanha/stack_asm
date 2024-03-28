use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::Push(Value::LabelReference("STRING")),
        Instruction::Push(Value::Inline(12)),
        Instruction::StdOut,
        Instruction::Push(Value::Inline(0)),
        Instruction::Exit,
        Instruction::LabelDeclaration("STRING"),
        Instruction::Raw("hello world\n".as_bytes()),
    ];
    let program = compile(&Targets::X86_64, &code);
    fs::write("./out", program).unwrap();
}
