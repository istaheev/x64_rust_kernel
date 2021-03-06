use core::iter::Iterator;

use memory::MemoryRegion;

/*
 * General memory information structures which (ideally) should be moved to a separate module.
 */

pub trait PhysicalMemoryMap {
    fn available_memory_regions<'a>(&'a self) -> MemoryRegionIterator<'a>;

    /* Total amount of memory available for kernel (in bytes) */
    fn total_memory_available(&self) -> usize {
        self.available_memory_regions()
            .fold(0, |acc,r| acc + r.size)
    }
}

/*
 * And this is MultiBoot part.
 */

/*
 * Constants for Info::flags.
 */

/* is there basic lower/upper memory information? */
const INFO_MEMORY: u32 =           0x00000001;
/* is there a boot device set? */
#[allow(dead_code)]
const INFO_BOOTDEV: u32 =          0x00000002;
/* is the command-line defined? */
#[allow(dead_code)]
const INFO_CMDLINE: u32 =          0x00000004;
/* are there modules to do something with? */
#[allow(dead_code)]
const INFO_MODS: u32 =             0x00000008;

/* These next two are mutually exclusive */

/* is there a symbol table loaded? */
#[allow(dead_code)]
const INFO_AOUT_SYMS: u32 =        0x00000010;
/* is there an ELF section header table? */
#[allow(dead_code)]
const INFO_ELF_SHDR: u32 =         0x00000020;

/* is there a full memory map? */
const INFO_MEM_MAP: u32 =          0x00000040;
/* Is there drive info? */
#[allow(dead_code)]
const INFO_DRIVE_INFO: u32 =       0x00000080;
/* Is there a config table? */
#[allow(dead_code)]
const INFO_CONFIG_TABLE: u32 =     0x00000100;
/* Is there a boot loader name? */
#[allow(dead_code)]
const INFO_BOOT_LOADER_NAME: u32 = 0x00000200;
/* Is there a APM table? */
#[allow(dead_code)]
const INFO_APM_TABLE: u32 =        0x00000400;
/* Is there video information? */
#[allow(dead_code)]
const INFO_VIDEO_INFO: u32 =       0x00000800;

/*
 * Constants for MemoryMapEntry::mem_type.
 */

const MEMORY_AVAILABLE: u32 =      1;
#[allow(dead_code)]
const MEMORY_RESERVED: u32 =       2;


/* The section header table for ELF. */
#[repr(C, packed)]
pub struct ElfSectionHeaderTable {
    num:   u32,
    size:  u32,
    addr:  u32,
    shndx: u32,
}


#[repr(C, packed)]
pub struct MemoryMapEntry
{
    size:     u32,
    addr:     u64,
    len:      u64,
    mem_type: u32,
}


#[repr(C)]
pub struct Info {
    /* Multiboot info version number */
    flags:             u32,

    /* Available memory from BIOS */
    mem_lower:         u32,
    mem_upper:         u32,

    /* "root" partition */
    boot_device:       u32,

    /* Kernel command line */
    cmdline:           u32,

    /* Boot-Module list */
    mods_count:        u32,
    mods_addr:         u32,

    elf_sec:           ElfSectionHeaderTable,

    /* Memory Mapping buffer */
    mmap_length:       u32,
    mmap_addr:         u32,

    /* Drive Info buffer */
    drives_length:     u32,
    drives_addr:       u32,

    /* ROM configuration table */
    config_table:      u32,

    /* Boot Loader Name */
    boot_loader_name:  u32,

    /* APM table */
    apm_table:         u32,

    /* Video */
    vbe_control_info:  u32,
    vbe_mode_info:     u32,
    vbe_mode:          u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
}

impl Info {
    pub fn is_memory_size_available(&self) -> bool {
        (self.flags & INFO_MEMORY) == INFO_MEMORY
    }

    pub fn is_memory_map_available(&self) -> bool {
        (self.flags & INFO_MEM_MAP) == INFO_MEM_MAP
    }

    pub fn get_lower_memory(&self) -> u64 {
        (self.mem_lower as u64) * 1024
    }

    pub fn get_upper_memory(&self) -> u64 {
        (self.mem_upper as u64) * 1024
    }
}


impl PhysicalMemoryMap for Info {
    fn available_memory_regions<'a>(&'a self) -> MemoryRegionIterator<'a> {
        if !self.is_memory_map_available() {
            panic!("No memory map available in multiboot info");
        }
        MemoryRegionIterator {
            info:        self,
            ptr:         self.mmap_addr,
            memory_type: MEMORY_AVAILABLE
        }
    }
}

pub struct MemoryRegionIterator<'a> {
    info: &'a Info,
    ptr: u32,
    memory_type: u32
}

impl<'a> Iterator for MemoryRegionIterator<'a> {
    type Item = MemoryRegion;

    fn next(&mut self) -> Option<Self::Item> {
        while self.ptr < self.info.mmap_addr + self.info.mmap_length {
            let mmap_entry = unsafe { &*(self.ptr as *const MemoryMapEntry) };
            self.ptr = self.ptr + mmap_entry.size + (::core::mem::size_of_val(&mmap_entry.size) as u32);
            if mmap_entry.mem_type == self.memory_type {
                return Some(MemoryRegion {
                    addr: (mmap_entry.addr as usize),
                    size: (mmap_entry.len as usize)
                })
            }
        }

        /* No suitable regions found */
        None
    }
}
