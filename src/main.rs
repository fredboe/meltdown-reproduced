mod fr;
mod utils;

use fr::FlushReload;
use std::collections::HashMap;

fn main() {
    let channel = FlushReload::new();

    let leaks = (0..10_000)
        .map(|_| {
            channel.leak(42);
            channel.get().unwrap()
        })
        .collect::<Vec<_>>();
    println!("{:?}", most_frequent_element(&leaks));
}

fn most_frequent_element(bytes: &[u8]) -> Option<u8> {
    let mut counts = HashMap::new();

    // Count occurrences
    for &byte in bytes {
        *counts.entry(byte).or_insert(0) += 1;
    }

    // Find the byte with the maximum count
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(byte, _)| byte)
}
