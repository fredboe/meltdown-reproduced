use std::arch::x86_64;
use std::ptr;

pub fn access(ptr: *const u8) {
    unsafe {
        let _x = ptr::read_volatile(ptr);
        x86_64::_mm_mfence();
    }
}

pub fn measure_access_time(ptr: *const u8) -> u64 {
    let start = get_time();
    access(ptr);
    let end = get_time();
    flush(ptr);
    end - start
}

pub fn get_time() -> u64 {
    unsafe {
        x86_64::_mm_mfence();
        let time = x86_64::_rdtsc();
        x86_64::_mm_mfence();
        time
    }
}

pub fn flush(addr: *const u8) {
    unsafe {
        x86_64::_mm_mfence();
        x86_64::_mm_clflush(addr);
        x86_64::_mm_mfence();
    }
}

pub fn evict(addr: *const u8) {
    // because clflush is not working try with eviction sets
}
