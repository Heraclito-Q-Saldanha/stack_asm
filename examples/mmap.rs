use stack_asm::*;
use std::fs;

fn main() {
    #[rustfmt::skip]
    let code = [
        Instruction::Push(Value::Inline(0x00)), //addr = 0
        Instruction::Push(Value::Inline(4096)), //length = 4096 bytes (1 page)
        Instruction::Push(Value::Inline(0x07)), //prot = PROT_READ | PROT_WRITE | PROT_EXEC
        Instruction::Push(Value::Inline(0x22)), //flags = MAP_PRIVATE | MAP_ANONYMOUS
        Instruction::Push(Value::Inline(0x00)), //fd = 0 (ignored because of MAP_ANONYMOUS)
        Instruction::Push(Value::Inline(0x00)), //offset = 0 (ignored because of MAP_ANONYMOUS)
        Instruction::MMap,
        Instruction::Exit
    ];
    let program = compile(&Targets::X86_64, &code).unwrap();
    fs::write("./out", program).unwrap();
}
