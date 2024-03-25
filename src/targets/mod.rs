mod x86_64;

use crate::*;

pub enum Targets {
    X86_64,
}

impl InstructionCode for Targets {
    fn get(&self, instruction: &Instruction) -> Box<[u8]> {
        match self {
            Self::X86_64 => x86_64::instruction_code(instruction),
        }
    }
}
