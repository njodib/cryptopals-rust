use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::aes_algs::{aes_ecb_decrypt, is_aes_ecb_encrypted};

pub fn print() {
    let file = File::open(&"encrypted/ch08.txt").unwrap();
    let reader = BufReader::new(file);
    let mut line_ct = 0;
    for line in reader.lines() {
        line_ct += 1;
        let l = line.unwrap();

        if is_aes_ecb_encrypted(l.as_bytes()){
            println!("Line {} is AES-128-ECB encrypted. It's message is: {}", line_ct, "UNKNOWN");
            break;
        }
    }
}