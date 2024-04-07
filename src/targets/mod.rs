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
            Instruction::LabelDeclaration(_) => 0,
            Instruction::Raw(data) => data.len(),
            Instruction::Push(_) => 11,
            Instruction::Pop => 1,
            Instruction::Dup => 3,
            Instruction::Swap => 4,
            Instruction::Store => 5,
            Instruction::Load => 5,
            Instruction::Jmp => 3,
            Instruction::Add => 6,
            Instruction::Sub => 6,
            Instruction::Mul => 7,
            Instruction::Div => 6,
            Instruction::Exit => 10,
            Instruction::StdOut => 18,
        }
    }
}
