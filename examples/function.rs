use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("start"),
            Instruction::Push(Value::LabelReference("return")),
            Instruction::Push(Value::Inline(10)),
            Instruction::Push(Value::Inline(6)),
            Instruction::Push(Value::LabelReference("add")),
            Instruction::Jmp,
            Instruction::LabelDeclaration("return"),
            Instruction::Exit,
        
        Instruction::LabelDeclaration("add"),
            Instruction::Add,
            Instruction::Swap,
            Instruction::Jmp
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
