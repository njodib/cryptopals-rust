use aes::Aes128;
use aes::cipher::{generic_array::GenericArray, KeyInit, BlockDecrypt};
use crate::xor::fixed_xor;

pub fn aes_ecb_decrypt(encrypted_bytes: &[u8], key_bytes: &[u8]) -> String {
    // Construct blocks of 16 byte size for AES-128
    let mut blocks = Vec::new();
    (0..encrypted_bytes.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(&encrypted_bytes[x..x + 16]));
    });
    
    // Initialize cipher
    let key = GenericArray::clone_from_slice(key_bytes);
    let cipher = Aes128::new(&key);
    
    //Decrypt, flatten, output
    cipher.decrypt_blocks(&mut blocks);
    blocks.iter().flatten().map(|&x| x as char).collect()
}

pub fn aes_cbc_decrypt(encrypted_bytes: &[u8], key_bytes: &[u8], iv_bytes: &[u8]) -> String {
    //Initialize cipher as AES-128 with key and iv
    let key = GenericArray::clone_from_slice(key_bytes);
    let cipher = Aes128::new(&key);
    let mut iv = iv_bytes.to_vec();
    
    //Encrypted bytes from Base 64 decoding of ch10 file, build decrypted
    let mut decrypted: Vec<u8> = Vec::new();

    //Step through encrypted bytes as a block of 16 bytes (4x4 matrix)
    (0..encrypted_bytes.len()).step_by(16).for_each(|x| {
        let ciphertext: Vec<u8> = encrypted_bytes[x..x + 16].to_vec();
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