use base64_light::base64_decode;
use std::fs;
use crate::aes_algs::{aes_cbc_decrypt, aes_cbc_encrypt};

fn read_bytes(path: &str) -> Vec<u8> {
    let base64_s = fs::read_to_string(path)
        .and_then(|res| Ok(res.replace("\n", "")))
        .expect("Error reading file");
    base64_decode(&base64_s)
}

pub fn print() {
    let encrypted = read_bytes("encrypted/ch10.txt");
    let key = "YELLOW SUBMARINE";
    let iv = [0 as u8; 16];

    let m1 = aes_cbc_decrypt(&encrypted, &key.as_bytes(), &iv);
    let m2 = aes_cbc_encrypt(&encrypted, &key.as_bytes(), &iv);
    let m3 = aes_cbc_decrypt(&encrypted, &key.as_bytes(), &iv);
    println!("{}", m3);
}
