
use crate::xor::decrypt_xor_multibyte;
use base64_light::base64_decode;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn print() {
    let mut content = String::new();
    let file = File::open(&"encrypted/ch06.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        content.push_str(line.unwrap().trim());
    }
    let encrypted = base64_decode(&content);
    println!("\nch06:\n{}", String::from_utf8(decrypt_xor_multibyte(&encrypted)).unwrap());
}
