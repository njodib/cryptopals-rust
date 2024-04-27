use crate::ch01::hex_encode;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::ch03::{decrypt_single_xor,score_english};



pub fn print() {
    //load txt file for challenge into buffered
    let path = "src/ch04.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    //
    let decrypted_bytes: Vec<u8> = buffered.lines() //lines as hex string
        .map(|line| hex_encode(&line.unwrap())) //lines as byte vectors
        .map(|bytes| decrypt_single_xor(&bytes)) //each line decrypted as well as possible
        .max_by_key(|decrypted_english| score_english(decrypted_english)) //line decryption with max score
        .unwrap(); //always at least 0, never fails
    println!("ch04: {}", String::from_utf8(decrypted_bytes).unwrap());

}

