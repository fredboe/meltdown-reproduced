mod fr;
mod meltdown;
mod utils;

use clap::Parser;
use fr::FlushReload;
use meltdown::MeltdownUS;

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
