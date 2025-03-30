mod fr;
mod meltdown;
mod utils;

use fr::FlushReload;

fn main() {
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
