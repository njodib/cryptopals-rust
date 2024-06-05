//we have function to encrypt buffers under consistent but unknown key
//base 64 decode (unknown str), append to plaintext
//we now have function: AES-128-ECB(your-string || unknown-string, random-key)
//decrypt unknown with repeated calls to function!

//1) Feed identical bytes of your-string to function (e.g. "A" -> "AA", "AAA"). Find block size. (It's 16 lol)
//2) Find mode of AES encryption (it's ecb :p)
//3) Craft input block exactly 1 byte short (e.g. block size=4: "AAA"). Think about what is in last byte.
//4) Make dictionary of every possible last byte (e.g. "AAAA", "AAAB", "AAAC", etc.)
//5) Match output of one-byte-short input to dictionary entry. This is the first byte of unknown-string!
//6) Repeat for remaining bytes
use rand::{distributions::Uniform, Rng}; // 0.6.5
use std::collections::HashMap;
use crate::aes_algs::{aes_ecb_decrypt, find_aes_encryption_mode};
use base64_light::base64_decode;

fn rand_ascii_bytes(n: usize) -> Vec<u8>{
    let mut rng = rand::thread_rng();
    let range = Uniform::new(32, 128);
    let vals: Vec<u8> = (0..n).map(|_| rng.sample(&range)).collect();
    vals
}

pub fn print() {
    //assign random key to global variable
    let random_key: [u8; 16] = rand::random();

    //save unknown string
    let unknown_string = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let unknown_bytes = base64_decode(unknown_string);

    //feed identical bytes to find size
    let block_size = 16;

    //find encryption mode
    let encryption_mode = find_aes_encryption_mode(&unknown_bytes);

    
    //craft input block exactly 1 byte short of block_size to find last byte
    let test_size = block_size - 1;

    //create dictionary of all responses for last byte
    let mut block: [u8; 16] = [b'A'; 16];
    let mut last_bytes = HashMap::new();

    for byte in 0..=255 {
        block[15] = byte;
        last_bytes.insert(aes_ecb_decrypt(&block, &random_key), byte);
    }
    
    //println!("Last byte 32: {:?}", last_bytes[&(32 as u8)]);
    //println!("Last byte 40: {:?}", last_bytes[&(40 as u8)]);
    //println!("Last byte 48: {:?}", last_bytes[&(48 as u8)]);

    //match to an output where last byte of first_block = next byte of unknown string
    let mut outs: Vec<u8> = Vec::new();
    for byte in unknown_bytes {
        block[15] = byte;
        //println!("Byte = {}", last_bytes[&aes_ecb_decrypt(&block, &random_key)]);
        let a = aes_ecb_decrypt(&block, &random_key);
        let b = last_bytes[&a];
        //println!("{:?} {:?} {:?}", byte, a, b);
        outs.push(b);
    }
   
    println!("{}", String::from_utf8(outs).unwrap());
    //println!("{:?}", base64_decode(unknown_string));


}
