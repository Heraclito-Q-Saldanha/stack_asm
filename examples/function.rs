use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("start"),
            Instruction::Push(Value::Inline(10)),
            Instruction::Push(Value::Inline(6)),
            Instruction::Push(Value::LabelReference("add")),
            Instruction::Call,
            Instruction::Exit,

        Instruction::LabelDeclaration("add"),
            Instruction::Push(Value::LabelReference("return_ptr")),
            Instruction::Store,
            Instruction::Add,
            Instruction::Push(Value::LabelReference("return_ptr")),
            Instruction::Load,
            Instruction::Jmp,
        Instruction::LabelDeclaration("return_ptr"),
        Instruction::Raw(&[0, 0, 0, 0, 0, 0, 0, 0])
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
