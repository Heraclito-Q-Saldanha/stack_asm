use std::fs;

fn main() {
    let code = [
        stack_asm::Instruction::Push("EXIT_CODE"),
        stack_asm::Instruction::Exit,
        stack_asm::Instruction::Label("EXIT_CODE", &[0]),
    ];
    let program = stack_asm::compile(&stack_asm::Targets::X86_64, &code);
    fs::write("./out", program).unwrap();
}
