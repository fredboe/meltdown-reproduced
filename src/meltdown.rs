#[allow(dead_code)]
pub trait Meltdown {
    fn read_from_kernel(addr: u64);
}

#[allow(dead_code)]
pub struct MeltdownUS {}

impl Meltdown for MeltdownUS {
    fn read_from_kernel(_addr: u64) {}
}
