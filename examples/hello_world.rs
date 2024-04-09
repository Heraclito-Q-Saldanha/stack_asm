use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::LabelDeclaration("start"),
            Instruction::Push(Value::LabelReference("string")),
            Instruction::Push(Value::Inline(12)), // str len
            Instruction::StdOut,
            Instruction::Push(Value::Inline(0)), // exit code
            Instruction::Exit,
        
        Instruction::LabelDeclaration("string"),
        Instruction::Raw("hello world\n".as_bytes()),
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
