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

    pub fn reset(&self) {
        for idx in (0..BUFFER_SIZE).step_by(PAGE_SIZE) {
            unsafe {
                utils::flush(self.buffer.as_ptr().add(idx));
            }
        }
    }

    pub fn leak(&self, x: u8) {
        let access_ptr = unsafe { self.buffer.as_ptr().add(x as usize * PAGE_SIZE) };
        utils::access(access_ptr);
    }

    pub fn get(&self) -> Option<u8> {
        let access_times = (0..BUFFER_SIZE)
            .step_by(PAGE_SIZE)
            .map(|idx| {
                let access_ptr = unsafe { self.buffer.as_ptr().add(idx) };
                utils::measure_access_time(access_ptr)
            })
            .collect::<Vec<_>>();

        let secret = access_times
            .into_iter()
            .enumerate()
            .min_by_key(|(_, val)| *val)
            .map(|(idx, _)| idx);

        let result = secret.map(|idx| idx as u8);
        self.reset();
        result
    }
}
