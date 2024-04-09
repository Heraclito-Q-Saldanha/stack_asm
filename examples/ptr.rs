use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("start"),
            Instruction::Push(Value::LabelReference("ptr")), //push ptr
            Instruction::Load, //load value from ptr
            Instruction::Push(Value::Inline(5)), //push 5
            Instruction::Add, //add
            Instruction::Exit, //exit
        
        Instruction::LabelDeclaration("ptr"), //ptr
        Instruction::Raw(&[10, 0, 0, 0, 0, 0, 0, 0]) //10
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
