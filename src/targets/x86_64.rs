use crate::*;

#[rustfmt::skip]
pub(crate) fn instruction_code(label_map: &LabelMap, instruction: &Instruction) -> Result<Box<[u8]>, error::Error> {
    match instruction {
        Instruction::LabelDeclaration(_) => Ok(Box::new([])),
        Instruction::Push(value) => match value {
            Value::Inline(data) => {
                // movabs rax, <data>;
                // push rax;
                let bytes = data.to_le_bytes();
                Ok(Box::new([
                    0x48, 0xB8,
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                    0x50,
                ]))
            }
            Value::LabelReference(label) => {
                let Some(data) = label_map.get(*label) else {
                    return Err(Error::LabelNotFound);
                };
                let data = data + 0x400078;
                let bytes = data.to_le_bytes();
                // movabs rax, <data>;
                // push rax;
                Ok(Box::new([
                    0x48, 0xB8,
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                    0x50,
                ]))
            }
        },
        Instruction::Dup => {
            // pop rax;
            // push rax;
            // push rax;
            Ok(Box::new([0x58, 0x50, 0x50]))
        }
        Instruction::Swap => {
            // pop rax;
            // pop rbx;
            // push rax;
            // push rbx;            
            Ok(Box::new([0x58, 0x5B, 0x50, 0x53]))
        }
        Instruction::Pop => {
            // pop rax;
            Ok(Box::new([0x58]))
        }
        Instruction::Exit => {
            // pop rdi;
            // mov rax, 60;
            // syscall;
            Ok(Box::new([0x5F, 0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00, 0x0F, 0x05]))
        }
        Instruction::Add => {
            // pop rax;
            // pop rbx;
            // add rax, rbx;
            // push rax;
            Ok(Box::new([0x58, 0x5B, 0x48, 0x01, 0xD8, 0x50]))
        },
        Instruction::Sub => {
            // pop rax;
            // pop rbx;
            // sub rbx, rax;
            // push rbx;
            Ok(Box::new([0x58, 0x5B, 0x48, 0x29, 0xC3, 0x53]))
        },
        Instruction::Mul => {
            // pop rax;
            // pop rbx;
            // imul rax, rbx;
            // push rax;
            Ok(Box::new([0x58, 0x5B, 0x48, 0x0F, 0xAF, 0xC3, 0x50]))
        }
        Instruction::Raw(raw) => Ok((*raw).into()),
        Instruction::Jmp => {
            // pop rax;
            // jmp rax;
            Ok(Box::new([0x58, 0xFF, 0xE0]))
        }
        Instruction::StdOut => {
            // mov rax, 1;
            // mov rdi, 1;
            
            // pop rdx;
            // pop rsi;
            
            // syscall;
            Ok(Box::new([0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00, 0x5A, 0x5E, 0x0F, 0x05]))
        }
    }
}
