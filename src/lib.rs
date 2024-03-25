mod elf;
mod targets;
pub use targets::*;

pub type LabelReference = str;

pub enum Instruction<'a> {
    Push(&'a LabelReference),
    Exit,
    Label(&'a LabelReference, &'a [u8]),
}

trait InstructionCode {
    fn get(&self, instruction: &Instruction) -> Box<[u8]>;
    #[inline(always)]
    fn len(&self, instruction: &Instruction) -> usize {
        self.get(instruction).len()
    }
}

pub fn compile(target: &Targets, code: &[Instruction]) -> Vec<u8> {
    let mut bytecode: Vec<u8> = code
        .into_iter()
        .map(|i| target.get(i))
        .flat_map(|i| i.into_vec())
        .collect();
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
    bytecode
}

fn get_label_reference(
    target: &Targets,
    code: &[Instruction],
    label: &LabelReference,
) -> Option<usize> {
    let mut counter = 0;
    for i in code {
        if let Instruction::Label(l, _) = i {
            if *l == label {
                return Some(counter);
            }
        }
        counter += target.len(i);
    }
    None
}
