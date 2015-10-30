use bits;

pub const CPU_FEAT1_FPU: u32          = 1 << 0;
pub const CPU_FEAT1_VME: u32          = 1 << 1;
pub const CPU_FEAT1_DE: u32           = 1 << 2;
pub const CPU_FEAT1_PSE: u32          = 1 << 3;
pub const CPU_FEAT1_TSC: u32          = 1 << 4;
pub const CPU_FEAT1_MSR: u32          = 1 << 5;
pub const CPU_FEAT1_PAE: u32          = 1 << 6;
pub const CPU_FEAT1_MCE: u32          = 1 << 7;
pub const CPU_FEAT1_CX8: u32          = 1 << 8;
pub const CPU_FEAT1_APIC: u32         = 1 << 9;
pub const CPU_FEAT1_SEP: u32          = 1 << 11;
pub const CPU_FEAT1_MTRR: u32         = 1 << 12;
pub const CPU_FEAT1_PGE: u32          = 1 << 13;
pub const CPU_FEAT1_MCA: u32          = 1 << 14;
pub const CPU_FEAT1_CMOV: u32         = 1 << 15;
pub const CPU_FEAT1_PAT: u32          = 1 << 16;
pub const CPU_FEAT1_PSE36: u32        = 1 << 17;
pub const CPU_FEAT1_PSN: u32          = 1 << 18;
pub const CPU_FEAT1_CLF: u32          = 1 << 19;
pub const CPU_FEAT1_DTES: u32         = 1 << 21;
pub const CPU_FEAT1_ACPI: u32         = 1 << 22;
pub const CPU_FEAT1_MMX: u32          = 1 << 23;
pub const CPU_FEAT1_FXSR: u32         = 1 << 24;
pub const CPU_FEAT1_SSE: u32          = 1 << 25;
pub const CPU_FEAT1_SSE2: u32         = 1 << 26;
pub const CPU_FEAT1_SS: u32           = 1 << 27;
pub const CPU_FEAT1_HTT: u32          = 1 << 28;
pub const CPU_FEAT1_TM1: u32          = 1 << 29;
pub const CPU_FEAT1_IA64: u32         = 1 << 30;
pub const CPU_FEAT1_PBE: u32          = 1 << 31;

pub const CPU_FEAT2_SSE3: u32         = 1 << 0;
pub const CPU_FEAT2_PCLMUL: u32       = 1 << 1;
pub const CPU_FEAT2_DTES64: u32       = 1 << 2;
pub const CPU_FEAT2_MONITOR: u32      = 1 << 3;
pub const CPU_FEAT2_DS_CPL: u32       = 1 << 4;
pub const CPU_FEAT2_VMX: u32          = 1 << 5;
pub const CPU_FEAT2_SMX: u32          = 1 << 6;
pub const CPU_FEAT2_EST: u32          = 1 << 7;
pub const CPU_FEAT2_TM2: u32          = 1 << 8;
pub const CPU_FEAT2_SSSE3: u32        = 1 << 9;
pub const CPU_FEAT2_CID: u32          = 1 << 10;
pub const CPU_FEAT2_FMA: u32          = 1 << 12;
pub const CPU_FEAT2_CX16: u32         = 1 << 13;
pub const CPU_FEAT2_ETPRD: u32        = 1 << 14;
pub const CPU_FEAT2_PDCM: u32         = 1 << 15;
pub const CPU_FEAT2_DCA: u32          = 1 << 18;
pub const CPU_FEAT2_SSE4_1: u32       = 1 << 19;
pub const CPU_FEAT2_SSE4_2: u32       = 1 << 20;
pub const CPU_FEAT2_X2APIC: u32       = 1 << 21;
pub const CPU_FEAT2_MOVBE: u32        = 1 << 22;
pub const CPU_FEAT2_POPCNT: u32       = 1 << 23;
pub const CPU_FEAT2_AES: u32          = 1 << 25;
pub const CPU_FEAT2_XSAVE: u32        = 1 << 26;
pub const CPU_FEAT2_OSXSAVE: u32      = 1 << 27;
pub const CPU_FEAT2_AVX: u32          = 1 << 28;

pub static CPU_FEATURES1_MAP: &'static [(u32, &'static str)] = &[
    (CPU_FEAT1_VME,     "vme"),
    (CPU_FEAT1_DE,      "de"),
    (CPU_FEAT1_FPU,     "fpu"),
    (CPU_FEAT1_PSE,     "pse"),
    (CPU_FEAT1_TSC,     "tsc"),
    (CPU_FEAT1_MSR,     "msr"),
    (CPU_FEAT1_PAE,     "pae"),
    (CPU_FEAT1_MCE,     "mce"),
    (CPU_FEAT1_CX8,     "cx8"),
    (CPU_FEAT1_APIC,    "apic"),
    (CPU_FEAT1_SEP,     "sep"),
    (CPU_FEAT1_MTRR,    "mtrr"),
    (CPU_FEAT1_PGE,     "pge"),
    (CPU_FEAT1_MCA,     "mca"),
    (CPU_FEAT1_CMOV,    "cmov"),
    (CPU_FEAT1_PAT,     "pat"),
    (CPU_FEAT1_PSE36,   "pse36"),
    (CPU_FEAT1_PSN,     "psn"),
    (CPU_FEAT1_CLF,     "clf"),
    (CPU_FEAT1_DTES,    "dtes"),
    (CPU_FEAT1_ACPI,    "acpi"),
    (CPU_FEAT1_MMX,     "mmx"),
    (CPU_FEAT1_FXSR,    "fxsr"),
    (CPU_FEAT1_SSE,     "sse"),
    (CPU_FEAT1_SSE2,    "sse2"),
    (CPU_FEAT1_SS,      "ss"),
    (CPU_FEAT1_HTT,     "htt"),
    (CPU_FEAT1_TM1,     "tm1"),
    (CPU_FEAT1_IA64,    "ia64"),
    (CPU_FEAT1_PBE,     "pbe"),
];

pub static CPU_FEATURES2_MAP: &'static [(u32, &'static str)] = &[
    (CPU_FEAT2_SSE3,    "sse3"),
    (CPU_FEAT2_PCLMUL,  "pclmul"),
    (CPU_FEAT2_DTES64,  "dtes64"),
    (CPU_FEAT2_MONITOR, "monitor"),
    (CPU_FEAT2_DS_CPL,  "ds_cpl"),
    (CPU_FEAT2_VMX,     "vmx"),
    (CPU_FEAT2_SMX,     "smx"),
    (CPU_FEAT2_EST,     "est"),
    (CPU_FEAT2_TM2,     "tm2"),
    (CPU_FEAT2_SSSE3,   "ssse3"),
    (CPU_FEAT2_CID,     "cid"),
    (CPU_FEAT2_FMA,     "fma"),
    (CPU_FEAT2_CX16,    "cx16"),
    (CPU_FEAT2_ETPRD,   "etprd"),
    (CPU_FEAT2_PDCM,    "pdcm"),
    (CPU_FEAT2_DCA,     "dca"),
    (CPU_FEAT2_SSE4_1,  "sse4_1"),
    (CPU_FEAT2_SSE4_2,  "sse4_2"),
    (CPU_FEAT2_X2APIC,  "x2apic"),
    (CPU_FEAT2_MOVBE,   "movbe"),
    (CPU_FEAT2_POPCNT,  "popcnt"),
    (CPU_FEAT2_AES,     "aes"),
    (CPU_FEAT2_XSAVE,   "xsave"),
    (CPU_FEAT2_OSXSAVE, "osxsave"),
    (CPU_FEAT2_AVX,     "avx"),
];


pub struct VendorId {
    pub vendor:           [u8; 12],
    pub max_basic_func:    u32,
    pub max_extended_func: u32
}

impl VendorId {
    pub fn is_cpu_info_available(&self) -> bool {
        self.max_basic_func >= 1
    }
}


pub struct CpuInfo {
    pub stepping: u32,
    pub model: u32,
    pub family: u32,
    pub cpu_type: u32,
    pub features1: u32,
    pub features2: u32,
    pub features3: u32,
}

pub fn get_vendor_id() -> VendorId {
    let max_basic_func: u32;
    let max_extended_func: u32;
    let vendor_part1: u32;
    let vendor_part2: u32;
    let vendor_part3: u32;

    unsafe {
        asm!("mov $$0, %eax\n \
              cpuid\n"
             : "={eax}" (max_basic_func),
               "={ebx}" (vendor_part1)
               "={edx}" (vendor_part2)
               "={ecx}" (vendor_part3));
    };

    let vendor:[u8; 12] = unsafe { ::core::mem::transmute_copy(&[vendor_part1, vendor_part2, vendor_part3]) };

    unsafe {
        asm!("mov $$0x80000000, %eax\n \
              cpuid\n"
             : "={eax}" (max_extended_func)
             :
             : "{eax}");
    };

    VendorId {
        vendor: vendor,
        max_basic_func: max_basic_func,
        max_extended_func: max_extended_func
    }
}

pub fn get_cpu_info() -> CpuInfo {
    let signature: u32;
    let features1: u32;
    let features2: u32;
    let features3: u32;

    unsafe {
        asm!("mov $$1, %eax\n \
              cpuid\n"
             : "={eax}" (signature),
               "={edx}" (features1)
               "={ecx}" (features2)
               "={ebx}" (features3));
    };

    let mut family = bits::get_range(signature, 8..12);
    let mut model = bits::get_range(signature, 4..8);

    if family == 6 || family == 15 {
        let ext_model = bits::get_range(signature, 16..20);
        model = model + (ext_model << 4);
    }

    if family == 15 {
        let ext_family = bits::get_range(signature, 20..28);
        family = family + ext_family;
    }

    CpuInfo {
        stepping:  bits::get_range(signature, 0..4),
        model:     model,
        family:    family,
        cpu_type:  bits::get_range(signature, 12..14),
        features1: features1,
        features2: features2,
        features3: features3,
    }
}

pub fn print_cpu_features(features: u32, map: &[(u32, &str)]) {
    for &(flag,desc) in map.iter() {
        if features & flag == flag {
            print!("{} ", desc);
        }
    }
    println!("");
}