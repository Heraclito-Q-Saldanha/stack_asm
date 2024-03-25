use crate::*;

pub(crate) fn instruction_code(instruction: &Instruction) -> Box<[u8]> {
    match instruction {
        Instruction::Label(label, value) => Box::new([]),
        Instruction::Push(label) => Box::new([]),
        Instruction::Exit => Box::new([]),
    }
}
