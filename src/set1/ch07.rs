use base64_light::base64_decode;
use std::fs;
use crate::aes_algs::{aes_ecb_decrypt, aes_ecb_encrypt};

fn read_bytes(path: &str) -> Vec<u8> {
    let base64_s = fs::read_to_string(path)
        .and_then(|res| Ok(res.replace("\n", "")))
        .expect("Error reading file");
    base64_decode(&base64_s)
}

pub fn print() {
    let encrypted_bytes = read_bytes("encrypted/ch07.txt");
    let key_str = "YELLOW SUBMARINE";

    let m1 = aes_ecb_decrypt(&encrypted_bytes, &key_str.as_bytes());
    let m2 = aes_ecb_encrypt(&encrypted_bytes, &key_str.as_bytes());
    let m3 = aes_ecb_decrypt(&encrypted_bytes, &key_str.as_bytes());
    println!("{}", m3);
}