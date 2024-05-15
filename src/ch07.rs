use aes::Aes128;
use aes::cipher::{generic_array::GenericArray, KeyInit, BlockDecrypt};
use base64_light::base64_decode;
use std::fs;

fn read_bytes(path: &str) -> Vec<u8> {
    let base64_s = fs::read_to_string(path)
        .and_then(|res| Ok(res.replace("\n", "")))
        .expect("Error reading file");
    base64_decode(&base64_s)
}

pub fn decrypt_ch07(path: &str, key_str: &str) -> String {
    let base64_bytes = read_bytes(path);
    let key = GenericArray::clone_from_slice(key_str.as_bytes());

    // Construct blocks of 16 byte size for AES-128
    let mut blocks = Vec::new();
    (0..base64_bytes.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(&base64_bytes[x..x + 16]));
    });

    // Initialize cipher
    let cipher = Aes128::new(&key);
    cipher.decrypt_blocks(&mut blocks);

    blocks.iter().flatten().map(|&x| x as char).collect()
}

pub fn print() {
    let message = decrypt_ch07("encrypted/ch07.txt", "YELLOW SUBMARINE");
    println!("{}", message);
}
