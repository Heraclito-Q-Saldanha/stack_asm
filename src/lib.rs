mod elf;
mod targets;
pub use targets::*;
mod error;
pub use error::*;

pub type LabelMap = std::collections::HashMap<String, u64>;

#[derive(Debug, Clone)]
pub enum Instruction<'a> {
    LabelDeclaration(&'a str),
    Raw(&'a [u8]),
    Push(Value<'a>),
    Pop,
    Dup,
    Swap,
    Store,
    Load,
    Add,
    Sub,
    Mul,
    Div,
    Jmp,
    Exit,
    StdOut,
    MMap,
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    LabelReference(&'a str),
    Inline(u64),
}

trait InstructionCode {
    fn get(
        &self,
        label_map: &LabelMap,
        instruction: &Instruction,
    ) -> Result<Box<[u8]>, error::Error>;
    fn len(&self, instruction: &Instruction) -> usize;
}

pub fn compile(target: &Targets, code: &[Instruction]) -> Result<Vec<u8>, error::Error> {
    let label_map = generate_label_map(target, code)?;
    let mut bytecode = Vec::<u8>::new();
    let code = code.into_iter().try_fold(Vec::new(), |mut acc, i| {
        acc.extend(target.get(&label_map, i)?.into_vec());
        Ok(acc)
    })?;

    let len = code.len() as u64;
    let header = elf::Elf64_Ehdr {
        e_ident: [
            elf::ELFMAG0,
            elf::ELFMAG1,
            elf::ELFMAG2,
            elf::ELFMAG3,
            elf::ELFCLASS64,
            elf::ELFDATA2LSB,
            elf::EV_CURRENT,
            elf::ELFOSABI_SYSV,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        e_type: elf::ET_EXEC,
        e_machine: elf::EM_X86_64,
        e_entry: 0x400078,
        e_phoff: 64,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 64,
        e_phentsize: 56,
        e_phnum: 1,
        e_shentsize: 64,
        e_shnum: 0,
        e_shstrndx: elf::SHN_UNDEF,
        e_version: 0,
    };
    let phdr = elf::Elf64_Phdr {
        p_type: elf::PT_LOAD,
        p_offset: 0x78,
        p_vaddr: 0x400078,
        p_paddr: 0x400078,
        p_filesz: len,
        p_memsz: len,
        p_flags: elf::PF_X | elf::PF_R | elf::PF_W,
        p_align: 0x8,
    };
    bytecode.extend(header.to_bytes());
    bytecode.extend(phdr.to_bytes());
    bytecode.extend(code);
    Ok(bytecode)
}

fn generate_label_map(target: &Targets, code: &[Instruction]) -> Result<LabelMap, error::Error> {
    let mut label_map = LabelMap::new();
    let mut counter = 0;
    for i in code {
        if let Instruction::LabelDeclaration(l) = i {
            if label_map.contains_key(*l) {
                return Err(error::Error::DuplicatedLabel(l.to_string()));
            }
            label_map.insert((*l).to_string(), counter);
        }
        counter += target.len(i) as u64;
    }
    Ok(label_map)
}
