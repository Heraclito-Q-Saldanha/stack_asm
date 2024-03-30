use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("LOOP"),
        
        Instruction::Push(Value::LabelReference("STRING")),
        Instruction::Push(Value::Inline(12)), // str len
        Instruction::StdOut,

        Instruction::Push(Value::LabelReference("LOOP")),
        Instruction::Jmp,
        
        Instruction::LabelDeclaration("STRING"),
        Instruction::Raw("hello world\n".as_bytes()),
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
