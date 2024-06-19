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
use crate::aes_algs::aes_decrypt;
use base64_light::base64_decode;


pub fn print() {
    //save unknown string
    let unknown_string = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let unknown_bytes = base64_decode(unknown_string);

    //decrypt and print
    let outs = aes_decrypt(&unknown_bytes);
    println!("{}", String::from_utf8(outs).unwrap());
}
