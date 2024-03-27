use crate::*;

#[rustfmt::skip]
pub(crate) fn instruction_code(label_map: &LabelMap, instruction: &Instruction) -> Box<[u8]> {
    match instruction {
        Instruction::LabelDeclaration(_) => Box::new([]),
        Instruction::Push(value) => match value {
            Value::Inline(data) => {
                // movabs rax, <data>;
                // push rax;
                let bytes = data.to_le_bytes();
                Box::new([
                    0x48, 0xB8,
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                    0x50,
                ])
            }
            Value::LabelReference(label) => {
                let data = label_map.get(*label).unwrap() + 0x400078;
                let bytes = data.to_le_bytes();
                // movabs rax, <data>;
                // push rax;
                Box::new([
                    0x48, 0xB8,
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                    0x50,
                ])
            }
        },
        Instruction::Exit => {
            // pop rdi;
            // mov rax, 60;
            // syscall;
            Box::new([0x5F, 0x48, 0xC7, 0xC0, 0x3C, 0x00, 0x00, 0x00, 0x0F, 0x05])
        }
        Instruction::Raw(raw) => (*raw).into()
    }
}
