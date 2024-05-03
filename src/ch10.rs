use openssl::symm::{decrypt, Cipher};
use crate::ch02::fixed_xor;
use base64_light::base64_decode;
use std::fs::read_to_string;




pub fn aes_128_ecb_decrypt_no_padding(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    cipher(Cipher::aes_128_ecb(), Decrypt, key, None, ciphertext)
}

pub fn aes_128_cbc_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, ErrorStack>
{
    if key.len() != 16 || iv.len() != 16 {
        panic!("key and iv must be equal to the block size");
    }

    let ciphertext_blocks: Vec<&[u8]> = ciphertext.chunks(16).collect();
    let mut plaintext = Vec::with_capacity(ciphertext_blocks.len());
    let mut next_iv = iv.to_vec();

    for block in ciphertext_blocks {
        let mut plaintext_block = fixed_xor(&next_iv, &aes_128_ecb_decrypt_no_padding(key, &block)?);
        next_iv = block.to_vec();
        plaintext.append(&mut plaintext_block);
    }

    // remove padding assuming PKCS#7 format
    let num_padding_bytes = plaintext[plaintext.len() - 1]; // get the number of padding bytes from the last byte
    plaintext.truncate(plaintext.len() - num_padding_bytes as usize);

    Ok(plaintext)
}


pub fn print(){
    let ciphertext = base64_decode(read_to_string("/src/ch10.txt"));

    let key = b"YELLOW SUBMARINE";
    let iv = vec![0u8; 16];

    println!("{:?}", aes_128_cbc_decrypt(key, iv, ciphertext));

}
