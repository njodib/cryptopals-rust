use crate::utils::*;

// PAPER TO IMPLEMENT: Base 64 encoding/decoding at nearly the speed of a memory copy -- https://arxiv.org/pdf/1910.05109.pdf

pub fn print() {
    //base64 <--> hex <--> ascii methods in utils
    let ch01_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("ch01: {}", hex_decode_str(ch01_hex));
}