mod x86_64;

use crate::*;

pub enum Targets {
    X86_64,
}

impl InstructionCode for Targets {
    fn get(&self, label_map: &LabelMap, instruction: &Instruction) -> Box<[u8]> {
        match self {
            Self::X86_64 => x86_64::instruction_code(label_map, instruction),
        }
    }
    fn len(&self, instruction: &Instruction) -> usize {
        match instruction {
            Instruction::Exit => 10,
            Instruction::LabelDeclaration(_) => 0,
            Instruction::Push(_) => 11,
            Instruction::Raw(data) => data.len(),
            Instruction::Add => 6,
        }
    }
}
