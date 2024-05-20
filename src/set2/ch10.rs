use aes::Aes128;
use aes::cipher::{generic_array::GenericArray, KeyInit, BlockDecrypt};
use base64_light::base64_decode;
use std::fs;
use crate::xor::fixed_xor;

fn read_bytes(path: &str) -> Vec<u8> {
    let base64_s = fs::read_to_string(path)
        .and_then(|res| Ok(res.replace("\n", "")))
        .expect("Error reading file");
    base64_decode(&base64_s)
}

pub fn decrypt_ch07(path: &str, key_str: &str) -> String {
    //Initialize cipher as AES-128 with key and iv
    let key = GenericArray::clone_from_slice(key_str.as_bytes());
    let cipher = Aes128::new(&key);
    let mut iv = [0 as u8; 16].to_vec();
    
    //Encrypted bytes from Base 64 decoding of ch10 file, build decrypted
    let encrypted = read_bytes(path);
    let mut decrypted: Vec<u8> = Vec::new();

    //Step through encrypted bytes as a block of 16 bytes (4x4 matrix)
    (0..encrypted.len()).step_by(16).for_each(|x| {
        let ciphertext: Vec<u8> = encrypted[x..x + 16].to_vec();
        let mut block = GenericArray::clone_from_slice(&ciphertext);

        //Decrypt the block with AES cipher, it becomes 'plaintext'
        cipher.decrypt_block(&mut block);
        let mut plaintext: Vec<u8> = Vec::new();
        for byte in block{
            plaintext.push(byte);
        }
        //XOR 'plaintext' and 'iv'. This block is decrypted
        for byte in fixed_xor(&plaintext, &iv){
            decrypted.push(byte);
        }
        
        //iv updated to ciphertext and saved for next decryption block
        iv = ciphertext.to_vec();
    });
    
    //Output as a string from decrypted bytes
    String::from_utf8(decrypted).unwrap()
}

pub fn print() {
    let message = decrypt_ch07("encrypted/ch10.txt", "YELLOW SUBMARINE");
    println!("{}", message);
}
