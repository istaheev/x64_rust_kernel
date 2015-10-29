use bits::Bits;

pub struct VendorId {
    pub vendor:           [u8; 12],
    pub max_basic_func:    u32,
    pub max_extended_func: u32
}

impl VendorId {
    pub fn is_processor_info_available(&self) -> bool {
        self.max_basic_func >= 1
    }
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
               "={ecx}" (vendor_part3)
             :
             : "{eax}");
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


pub fn get_processor_info() -> bool {
    let a = Bits(1013u32).get(1..3);
    a > 1
}
