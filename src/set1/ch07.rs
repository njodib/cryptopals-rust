use base64_light::base64_decode;
use std::fs;
use crate::aes_algs::aes_ecb_decrypt;

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
    //check decryption AND encryption
    //let m2: Vec<u8> = aes_ecb_encrypt(&m1.as_bytes(), &key_str.as_bytes());
    //let m3: String = aes_ecb_decrypt(&m2, &key_str.as_bytes());
    println!("{}", String::from_utf8(m1).unwrap());
}