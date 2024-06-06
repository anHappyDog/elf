#![no_std]

extern crate alloc;

use alloc::str;
use alloc::vec::Vec;

pub const ELF_32: u8 = 0x1;
pub const ELF_64: u8 = 0x2;

#[cfg(target_endian = "big")]
pub const ELF_LITTLE_ENDIAN: u8 = 0x1;
#[cfg(target_endian = "little")]
pub const ELF_BIG_ENDIAN: u8 = 0x2;

//p_type
//pub const PT_NULL : u32 = 0x0;
pub const PT_LOAD: u32 = 0x1;
//pub const PT_DYNAMIC : u32 = 0x2;
// pub const PT_INTERP : u32 = 0x3;
// pub const PT_NOTE : u32 = 0x4;
// pub const PT_SHILB : u32 = 0x5;
// pub const PT_PHDR : u32 = 0x6;
// pub const PT_TLS : u32 = 0x7;
// pub const PTLOOPS : u32 = 0x60000000;
// pub const PT_HIOS : u32 = 0x6fffffff;
// pub const PT_LOPROC : u32 = 0x70000000;
// pub const PT_HIPROC : u32 = 0x7fffffff;

//p_flags
//acutally we dont have and segment management, so we have no idea about how to process it.
//may use later.
pub const PF_X: u32 = 0x1;
pub const PF_W: u32 = 0x2;
pub const PF_R: u32 = 0x4;

//sh_type
// pub const SHT_NULL : u32 = 0x0;
// pub const SHT_PROGBITS : u32 = 0x1;
// pub const SHT_SYMTAB : u32 = 0x2;
// pub const SHT_STRTAB : u32 = 0x3;
// pub const SHT_RELA : u32 = 0x4;
// pub const SHT_HASH : u32 = 0x5;
// pub const SHT_DYNAMIC : u32 = 0x6;
// pub const SHT_NOTE : u32 = 0x7;
// pub const SHT_NOBITS : u32 = 0x8;
// pub const SHT_REL : u32 = 0x9;
// pub const SHT_SHLIB : u32 = 0xa;
// pub const SHT_DYNSYM : u32 = 0xb;
// pub const SHTLOOPS : u32 = 0x60000000;
// pub const SHT_HIOS : u32 = 0x6fffffff;
// pub const SHT_LOPROC : u32 = 0x70000000;
// pub const SHT_HIPROC : u32 = 0x7fffffff;
// pub const SHT_LOUSER : u32 = 0x80000000;

//sh_flags
// pub const SHF_WRITE : u32 = 0x1;
// pub const SHF_ALLOC : u32 = 0x2;
// pub const SHF_EXECINSTR : u32 = 0x4;
// pub const SHF_MERGE : u32 = 0x10;
// pub const SHF_STRINGS : u32 = 0x20;
// pub const SHF_INFO_LINK : u32 = 0x40;
// pub const SHF_LINK_ORDER : u32 = 0x80;
// pub const SHF_OS_NONCONFORMING : u32 = 0x100;
// pub const SHF_GROUP : u32 = 0x200;
// pub const SHF_TLS : u32 = 0x400;
// pub const SHF_MASKOS : u32 = 0x0ff00000;
// pub const SHF_MASKPROC : u32 = 0xf0000000;
// pub const SHF_ORDERED : u32 = 0x4000000;
// pub const SHF_EXCLUDE : u32 = 0x8000000;

//e-machine
//no need to use.

pub trait ProgramHeader {
    fn get_type(&self) -> u32;
    fn get_offset(&self) -> usize;
    fn get_vaddr(&self) -> usize;
    fn get_paddr(&self) -> usize;
    fn get_filesz(&self) -> usize;
    fn get_memsz(&self) -> usize;
    fn get_flags(&self) -> u32;
    fn get_align(&self) -> usize;
    fn get_perm(&self) -> usize;
}

pub trait SectionHeader {
    fn get_name(&self) -> usize;
    fn get_type(&self) -> usize;
    fn get_flags(&self) -> usize;
    fn get_addr(&self) -> usize;
    fn get_offset(&self) -> usize;
    fn get_size(&self) -> usize;
    fn get_link(&self) -> usize;
    fn get_info(&self) -> usize;
    fn get_addralign(&self) -> usize;
    fn get_entsize(&self) -> usize;
}

pub trait ElfHeader {
    fn get_entry(&self) -> usize;
    fn get_program_header_offset(&self) -> usize;
    fn get_program_header_size(&self) -> usize;
    fn get_section_header_offset(&self) -> usize;
    fn get_section_header_size(&self) -> usize;
    fn get_elf_pre_header(&self) -> &ElfIdent;
    fn get_program_header_num(&self) -> usize;
    fn get_section_header_num(&self) -> usize;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ElfIdent {
    magic: [u8; 4],
    class: u8,
    data: u8,
    version: u8,
    os_abi: u8,
    abi_version: u8,
    padding: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ElfHeader32 {
    elf_pre_header: ElfIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ElfHeader64 {
    elf_pre_header: ElfIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ProgramHeader32 {
    p_type: u32,

    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flags: u32,
    p_align: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ProgramHeader64 {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SectionHeader32 {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SectionHeader64 {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

impl ElfIdent {
    pub fn try_load(data: &[u8]) -> Result<ElfIdent, &'static str> {
        if data.len() < core::mem::size_of::<ElfIdent>() {
            return Err("data is too short");
        }
        let pre_header = unsafe { core::ptr::read(data.as_ptr() as *const ElfIdent) };
        Ok(pre_header)
    }
    pub fn get_class(&self) -> u8 {
        self.class
    }
    #[allow(dead_code)]
    pub fn get_data(&self) -> u8 {
        self.data
    }
}

impl ElfHeader for ElfHeader32 {
    fn get_program_header_num(&self) -> usize {
        self.e_phnum as usize
    }
    fn get_section_header_num(&self) -> usize {
        self.e_shnum as usize
    }
    fn get_entry(&self) -> usize {
        self.e_entry as usize
    }
    fn get_program_header_offset(&self) -> usize {
        self.e_phoff as usize
    }
    fn get_program_header_size(&self) -> usize {
        core::mem::size_of::<ProgramHeader32>()
    }
    fn get_section_header_offset(&self) -> usize {
        self.e_shoff as usize
    }
    fn get_section_header_size(&self) -> usize {
        core::mem::size_of::<SectionHeader32>()
    }
    fn get_elf_pre_header(&self) -> &ElfIdent {
        &self.elf_pre_header
    }
}

impl ElfHeader for ElfHeader64 {
    fn get_program_header_num(&self) -> usize {
        self.e_phnum as usize
    }
    fn get_section_header_num(&self) -> usize {
        self.e_shnum as usize
    }
    fn get_entry(&self) -> usize {
        self.e_entry as usize
    }
    fn get_program_header_offset(&self) -> usize {
        self.e_phoff as usize
    }
    fn get_program_header_size(&self) -> usize {
        core::mem::size_of::<ProgramHeader64>()
    }
    fn get_section_header_offset(&self) -> usize {
        self.e_shoff as usize
    }
    fn get_section_header_size(&self) -> usize {
        core::mem::size_of::<SectionHeader64>()
    }
    fn get_elf_pre_header(&self) -> &ElfIdent {
        &self.elf_pre_header
    }
}

impl SectionHeader for SectionHeader32 {
    fn get_name(&self) -> usize {
        self.sh_name as usize
    }
    fn get_type(&self) -> usize {
        self.sh_type as usize
    }
    fn get_flags(&self) -> usize {
        self.sh_flags as usize
    }
    fn get_addr(&self) -> usize {
        self.sh_addr as usize
    }
    fn get_offset(&self) -> usize {
        self.sh_offset as usize
    }
    fn get_size(&self) -> usize {
        self.sh_size as usize
    }
    fn get_link(&self) -> usize {
        self.sh_link as usize
    }
    fn get_info(&self) -> usize {
        self.sh_info as usize
    }
    fn get_addralign(&self) -> usize {
        self.sh_addralign as usize
    }
    fn get_entsize(&self) -> usize {
        self.sh_entsize as usize
    }
}

impl SectionHeader for SectionHeader64 {
    fn get_addr(&self) -> usize {
        self.sh_addr as usize
    }
    fn get_offset(&self) -> usize {
        self.sh_offset as usize
    }
    fn get_size(&self) -> usize {
        self.sh_size as usize
    }
    fn get_name(&self) -> usize {
        self.sh_name as usize
    }
    fn get_type(&self) -> usize {
        self.sh_type as usize
    }
    fn get_flags(&self) -> usize {
        self.sh_flags as usize
    }
    fn get_link(&self) -> usize {
        self.sh_link as usize
    }
    fn get_info(&self) -> usize {
        self.sh_info as usize
    }
    fn get_addralign(&self) -> usize {
        self.sh_addralign as usize
    }
    fn get_entsize(&self) -> usize {
        self.sh_entsize as usize
    }
}

impl ProgramHeader for ProgramHeader32 {
    fn get_perm(&self) -> usize {
        return self.p_flags as usize;
    }
    fn get_type(&self) -> u32 {
        self.p_type
    }
    fn get_offset(&self) -> usize {
        self.p_offset as usize
    }
    fn get_vaddr(&self) -> usize {
        self.p_vaddr as usize
    }
    fn get_paddr(&self) -> usize {
        self.p_paddr as usize
    }
    fn get_filesz(&self) -> usize {
        self.p_filesz as usize
    }
    fn get_memsz(&self) -> usize {
        self.p_memsz as usize
    }
    fn get_flags(&self) -> u32 {
        self.p_flags
    }
    fn get_align(&self) -> usize {
        self.p_align as usize
    }
}

impl ProgramHeader for ProgramHeader64 {
    fn get_perm(&self) -> usize {
        return self.p_flags as usize;
    }
    fn get_type(&self) -> u32 {
        self.p_type
    }
    fn get_offset(&self) -> usize {
        self.p_offset as usize
    }
    fn get_vaddr(&self) -> usize {
        self.p_vaddr as usize
    }
    fn get_paddr(&self) -> usize {
        self.p_paddr as usize
    }
    fn get_filesz(&self) -> usize {
        self.p_filesz as usize
    }
    fn get_memsz(&self) -> usize {
        self.p_memsz as usize
    }
    fn get_flags(&self) -> u32 {
        self.p_flags
    }
    fn get_align(&self) -> usize {
        self.p_align as usize
    }
}

/// # Note
/// This function is used to load the elf pre header from the elf file content.
/// You should load the pre header to determine the elf file type and then load the elf header.
/// # Arguments
/// * `data` - The elf file content.
/// * `pre_header` - The elf pre header.
/// # Return
/// Result<T, &'static str> - The elf header.
/// # Example
/// ```rust
/// let pre_header = ElfIdent::try_load(data).unwrap();
/// let elf_header = load_elf_header::<ElfHeader32>(data, &pre_header).unwrap();
/// ```
pub fn load_elf_header<T>(data: &[u8], pre_header: &ElfIdent) -> Result<T, &'static str>
where
    T: ElfHeader,
{
    let header = unsafe { core::ptr::read(data.as_ptr() as *const T) };
    #[cfg(target_endian = "big")]
    if pre_header.data == ELF_LITTLE_ENDIAN {
        return Error("not a little endian elf file");
    }
    #[cfg(target_endian = "little")]
    if pre_header.data == ELF_BIG_ENDIAN {
        return Err("not a big endian elf file");
    }
    Ok(header)
}

/// # Note
/// This function is used to load the program headers from the elf file.
/// You should load the pre header to determine the elf file type and then load the elf header.
/// # Arguments
/// * `data` - The elf file content.
/// * `elf_header` - The elf header.
/// # Return
/// Result<Vec<T>, &'static str> - The program headers.
/// # Example
/// ```rust
/// let elf_header = load_elf_header::<ElfHeader32>(data, &pre_header).unwrap();
/// ```
pub fn load_elf_section_headers<T>(
    data: &[u8],
    elf_header: &dyn ElfHeader,
) -> Result<Vec<T>, &'static str>
where
    T: SectionHeader,
{
    let mut section_headers: Vec<T> = Vec::new();
    let section_header_size = core::mem::size_of::<T>();
    let section_header_num = elf_header.get_section_header_num();
    let section_header_offset = elf_header.get_section_header_offset();
    let section_header_data = &data
        [section_header_offset..section_header_offset + section_header_size * section_header_num];
    for i in 0..section_header_num {
        let section_header = unsafe {
            core::ptr::read(
                section_header_data
                    .as_ptr()
                    .offset(i as isize * section_header_size as isize) as *const T,
            )
        };
        section_headers.push(section_header);
    }
    Ok(section_headers)
}

/// # Note
/// This function is used to load elf program headers from elf file content.
/// You should load the pre header to determine the elf file type and then load the elf header.
/// # Arguments
/// * `data` - The elf file content.
/// * `elf_header` - The elf header.
/// # Return
/// Result<Vec<T>, &'static str> - The program headers.
/// # Example
/// ```rust
/// let elf_header = load_elf_header::<ElfHeader32>(data, &pre_header).unwrap();
/// let program_headers = load_elf_program_headers::<ProgramHeader32>(data, &elf_header).unwrap();
/// ```
pub fn load_elf_program_headers<T>(
    data: &[u8],
    elf_header: &dyn ElfHeader,
) -> Result<Vec<T>, &'static str>
where
    T: ProgramHeader,
{
    let mut program_headers: Vec<T> = Vec::new();
    let program_header_size = core::mem::size_of::<T>();
    let program_header_num = elf_header.get_program_header_num();
    let program_header_offset = elf_header.get_program_header_offset();
    let program_header_data = &data
        [program_header_offset..program_header_offset + program_header_size * program_header_num];
    for i in 0..program_header_num {
        let program_header = unsafe {
            core::ptr::read(
                program_header_data
                    .as_ptr()
                    .offset(i as isize * program_header_size as isize) as *const T,
            )
        };
        if program_header.get_memsz() != 0 {
            program_headers.push(program_header);
        }
    }
    Ok(program_headers)
}

/// # Note
/// This macro is used to define a static variable which is a slice of bytes which contains the elf file content.
/// # Arguments
/// * `$var_name` - The name of the static variable.
/// * `$path` - The path of the elf file.
/// # Example
/// ```rust
/// DEFINE_ELF_BYTES!(HELLO_WORLD,"../../user/target/riscv64gc-unknown-none-elf/release/hello_world.bin");
/// ```
#[macro_export]
macro_rules! DEFINE_ELF_BYTES {
    ($var_name : ident,$path : literal) => {
        static $var_name: &[u8] = include_bytes!($path);
    };
}
