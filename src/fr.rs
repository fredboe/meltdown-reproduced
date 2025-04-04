use crate::utils;

const BYTE_SIZE: usize = 256;
const PAGE_SIZE: usize = 4096;
const BUFFER_SIZE: usize = BYTE_SIZE * PAGE_SIZE;

pub struct FlushReload {
    buffer: Box<[u8; BUFFER_SIZE]>,
}

impl FlushReload {
    pub fn new() -> FlushReload {
        let buffer = Box::new([211u8; BUFFER_SIZE]);
        FlushReload { buffer }
    }

    pub fn leak(&self, x: u8) {
        self.reset();
        self.leak_without_prev_reset(x);
    }

    pub fn reset(&self) {
        for idx in (0..BYTE_SIZE).map(|idx| idx * PAGE_SIZE) {
            unsafe {
                utils::flush(self.buffer.as_ptr().add(idx));
            }
        }
    }

    pub fn leak_without_prev_reset(&self, x: u8) {
        let access_ptr = unsafe { self.buffer.as_ptr().add(x as usize * PAGE_SIZE) };
        utils::access_unserialized(access_ptr);
    }

    pub fn get(&self) -> Option<u8> {
        let access_times = (0..BYTE_SIZE)
            .map(|idx| (113 * idx + 1) % BYTE_SIZE) // iterate "pseudorandomly"
            .map(|idx| {
                let access_ptr = unsafe { self.buffer.as_ptr().add(idx * PAGE_SIZE) };
                (idx, utils::measure_access_time(access_ptr))
            })
            .collect::<Vec<_>>();

        let secret = access_times
            .into_iter()
            .min_by_key(|(_, val)| *val)
            .map(|(idx, _)| idx);

        let result = secret.map(|idx| idx as u8);
        self.reset();
        result
    }
}
