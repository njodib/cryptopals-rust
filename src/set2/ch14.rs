use crate::aes_algs::aes_decrypt;
use base64_light::base64_decode;


pub fn print() {
    //ORACLE
    // (random bytes+"Um9s.."+plaintext) encrypted under AES with random key
    // "Um9s.." is attacker controlled with known length= l
    // 1. 

    

    //save unknown string
    let random_prepend = "uayuiuhlauhwd";
    let unknown_string = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let unknown_bytes = base64_decode(unknown_string);

    //decrypt and print
    let outs = aes_decrypt(&unknown_bytes);
    println!("{}", String::from_utf8(outs).unwrap());
}
