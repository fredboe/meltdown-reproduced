use std::arch::x86_64;
use std::ptr;

pub fn access_unserialized(ptr: *const u8) {
    unsafe {
        let _x = ptr::read_volatile(ptr);
    }
}

pub fn access_serialized(ptr: *const u8) {
    unsafe {
        let _x = ptr::read_volatile(ptr);
        x86_64::_mm_mfence();
    }
}

pub fn measure_access_time(ptr: *const u8) -> u64 {
    let start = get_time();
    access_serialized(ptr);
    let end = get_time();
    end - start
}

pub fn get_time() -> u64 {
    unsafe {
        let mut aux = 0;
        let time = x86_64::__rdtscp(&mut aux);
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

#[allow(dead_code)]
pub fn evict(_addr: *const u8) {
    // because clflush is not working try with eviction sets
}
