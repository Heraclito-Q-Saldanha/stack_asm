mod x86_64;

use crate::*;

pub enum Targets {
    X86_64,
}

impl InstructionCode for Targets {
    fn get(&self, label_map: &LabelMap, instruction: &Instruction) -> Result<Box<[u8]>, Error> {
        match self {
            Self::X86_64 => x86_64::instruction_code(label_map, instruction),
        }
    }
    fn len(&self, instruction: &Instruction) -> usize {
        match instruction {
            Instruction::Exit => 10,
            Instruction::LabelDeclaration(_) => 0,
            Instruction::Push(_) => 11,
            Instruction::Pop => 1,
            Instruction::Dup => 3,
            Instruction::Swap => 4,
            Instruction::Raw(data) => data.len(),
            Instruction::Add => 6,
            Instruction::Jmp => 3,
            Instruction::StdOut => 18,
            Instruction::Sub => 6,
            Instruction::Mul => 7,
        }
    }
}
