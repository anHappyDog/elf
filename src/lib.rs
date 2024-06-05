#![no_std]

pub trait ElfType {}
impl ElfType for u32 {}

impl ElfType for u64 {}

pub struct ElfIdent {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub osabi: u8,
    pub abiversion: u8,
    pub pad: [u8; 7],
}

pub struct ElfHeader<T: ElfType> {
    ident: ElfIdent,
    etype: u16,
    machine: u16,
    version: u32,
    entry: T,
    phoff: T,
    shoff: T,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

pub struct ElfProgramHeader<T: ElfType> {
    ptype: u32,
    offset: T,
    vaddr: T,
    paddr: T,
    filesz: T,
    memsz: T,
    flags: u32,
    align: T,
}

pub struct ElfSectionHeader<T: ElfType> {
    name: T,
    stype: u32,
    flags: u32,
    addr: T,
    offset: T,
    size: T,
    link: u32,
    info: u32,
    addralign: T,
    entsize: T,
}

impl ElfIdent {
    pub fn new(data: &[u8]) -> Self {
        Self {
            magic: [data[0], data[1], data[2], data[3]],
            class: data[4],
            data: data[5],
            version: data[6],
            osabi: data[7],
            abiversion: data[8],
            pad: [
                data[9], data[10], data[11], data[12], data[13], data[14], data[15],
            ],
        }
    }
    pub fn is_little_endian(&self) -> bool {
        self.data == 1
    }
    pub fn is_64bit(&self) -> bool {
        self.class == 2
    }
    pub fn is_valid(&self) -> bool {
        self.magic == [0x7f, 0x45, 0x4c, 0x46]
    }
}

impl<T: ElfType + Copy> ElfHeader<T> {
    pub fn new(data: &[u8]) -> Self {
        unsafe { (data as *const [u8] as *const ElfHeader<T>).read() }
    }
    pub fn get_etype(&self) -> u16 {
        self.etype
    }
    pub fn get_phoff(&self) -> T {
        self.phoff
    }
    pub fn get_shoff(&self) -> T {
        self.shoff
    }
    pub fn get_phentsize(&self) -> u16 {
        self.phentsize
    }
    pub fn get_phnum(&self) -> u16 {
        self.phnum
    }
    pub fn get_shentsize(&self) -> u16 {
        self.shentsize
    }
    pub fn get_shnum(&self) -> u16 {
        self.shnum
    }
    pub fn get_shstrndx(&self) -> u16 {
        self.shstrndx
    }
}

impl<T: ElfType + Copy> ElfProgramHeader<T> {
    pub fn new(data: &[u8]) -> Self {
        unsafe { (data as *const [u8] as *const ElfProgramHeader<T>).read() }
    }
    pub fn get_ptype(&self) -> u32 {
        self.ptype
    }
    pub fn get_offset(&self) -> T {
        self.offset
    }
    pub fn get_vaddr(&self) -> T {
        self.vaddr
    }
    pub fn get_paddr(&self) -> T {
        self.paddr
    }
    pub fn get_filesz(&self) -> T {
        self.filesz
    }
    pub fn get_memsz(&self) -> T {
        self.memsz
    }
    pub fn get_flags(&self) -> u32 {
        self.flags
    }
    pub fn get_align(&self) -> T {
        self.align
    }
}

impl<T: ElfType + Copy> ElfSectionHeader<T> {
    pub fn new(data: &[u8]) -> Self {
        unsafe { (data as *const [u8] as *const ElfSectionHeader<T>).read() }
    }
    pub fn get_name(&self) -> T {
        self.name
    }
    pub fn get_stype(&self) -> u32 {
        self.stype
    }
    pub fn get_flags(&self) -> u32 {
        self.flags
    }
    pub fn get_addr(&self) -> T {
        self.addr
    }
    pub fn get_offset(&self) -> T {
        self.offset
    }
    pub fn get_size(&self) -> T {
        self.size
    }
    pub fn get_link(&self) -> u32 {
        self.link
    }
    pub fn get_info(&self) -> u32 {
        self.info
    }
    pub fn get_addralign(&self) -> T {
        self.addralign
    }
    pub fn get_entsize(&self) -> T {
        self.entsize
    }
}
