use crate::utils::hex_decode;
use crate::xor::decrypt_xor_singlebyte;




pub fn print(){
    let encrypted_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encrypted_bytes = hex_decode(encrypted_hex);
    let decrypted_bytes = decrypt_xor_singlebyte(&encrypted_bytes);
    println!("{}", String::from_utf8(decrypted_bytes).unwrap());
}
