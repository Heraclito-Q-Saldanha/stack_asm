use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("start"),
            Instruction::Push(Value::LabelReference("hello")),
            Instruction::Call,
            Instruction::Push(Value::LabelReference("exit")),
            Instruction::Call,

        Instruction::LabelDeclaration("hello"),
            Instruction::Push(Value::LabelReference("string")),
            Instruction::Push(Value::Inline(10)), // str len
            Instruction::StdOut,
            Instruction::Jmp,

        Instruction::LabelDeclaration("exit"),
            Instruction::Push(Value::Inline(0)),
            Instruction::Exit,
        
        Instruction::LabelDeclaration("string"),
        Instruction::Raw(b"hello :))\n")
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
