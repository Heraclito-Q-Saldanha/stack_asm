use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("MAIN"),
            Instruction::Push(Value::LabelReference("RETURN")),
            Instruction::Push(Value::Inline(10)),
            Instruction::Push(Value::Inline(6)),
            Instruction::Push(Value::LabelReference("ADD_FN")),
            Instruction::Jmp,
            Instruction::LabelDeclaration("RETURN"),
            Instruction::Exit,

        Instruction::LabelDeclaration("ADD_FN"),
            Instruction::Add,
            Instruction::Swap,
            Instruction::Jmp
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
