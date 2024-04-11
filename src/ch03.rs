use crate::ch01::hex_encode;
use crate::ch02::fixed_xor;

pub fn score_english(bytes: &[u8]) -> u32 {


    //if the bytes don't return a valid string, then it is an error
    if String::from_utf8(bytes.to_vec()).is_err() {
        return 0;
    }
    
    let letters = 
    [(b'a', 0.0651738), (b'b', 0.0124248), (b'c', 0.0217339), (b'd', 0.0349835), (b'e', 0.1041442), (b'f', 0.0197881), (b'g', 0.0158610),
     (b'h', 0.0492888), (b'i', 0.0558094), (b'j', 0.0009033), (b'k', 0.0050529), (b'l', 0.0331490), (b'm', 0.0202124), (b'n', 0.0564513),
     (b'o', 0.0596302), (b'p', 0.0137645), (b'q', 0.0008606), (b'r', 0.0497563), (b's', 0.0515760), (b't', 0.0729357), (b'u', 0.0225134),
     (b'v', 0.0082903), (b'w', 0.0171272), (b'x', 0.0013692), (b'y', 0.0145984), (b'z', 0.0007836)];

     let mut total = 0.0;

    for byte in bytes {
        let score_byte = match byte {
            b'a'..=b'z' => letters[(byte - b'a') as usize].1,
            b'A'..=b'Z' => letters[(byte - b'A')  as usize].1 / 25.0, //estimate 2-4% of English letters are capitals
            b' ' => 0.1918182,
            b'\n' => 0.1918182, //count newlines as spaces
            b'^' => -100.0,
            _ => 0.0
        };
        total += score_byte;
    }
    total as u32

}

pub fn is_english(bytes: &[u8]) -> bool{
    //use a 75% threshold -> reduce to 3/4
    let letters_and_spaces_ct = bytes.iter()
    .filter(|&&b| (b'a'..=b'z').contains(&b) || b==b' ')
    .count() * 10 > bytes.len() * 6;

    letters_and_spaces_ct
}

pub fn apply_key(key: u8, bytes: &[u8]) -> Vec<u8> {
    fixed_xor(bytes, &vec![key;bytes.len()])
}



pub fn decrypt_single_xor(encrypted: &[u8]) -> Vec<u8> {
    //apply_key(best_key(encrypted), encrypted)
    (32..=127).into_iter()
    .map(|key| apply_key(key,encrypted)) //gets all decryptions
    .filter(|decrypted| is_english(&decrypted)) //gets decryption possibilities
    .max_by_key(|decrypted| score_english(&decrypted))
    .unwrap_or([0;8].to_vec())
}

pub fn print(){
    let encrypted_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encrypted_bytes = hex_encode(encrypted_hex);
    let decrypted_bytes = decrypt_single_xor(&encrypted_bytes);
    println!("ch03: {}",String::from_utf8(decrypted_bytes).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_ch03() {
        //begin with a message to encrypt
        let unencrypted_text = "The Recurse Center is a self-directed, community-driven educational retreat for programmers in New York City.";
        let unencrypted_bytes = unencrypted_text.as_bytes().to_vec();

        for encryption_key in 32..=127 {
            //encrypt message with new encryption key
            let encrypted_bytes = apply_key(encryption_key, &unencrypted_bytes);
    
            //decrypt message
            //let decryption_key = best_key(&encrypted_bytes);
            let decrypted_text = String::from_utf8(decrypt_single_xor(&encrypted_bytes)).unwrap();
            
            //check if decryption was successful
            //assert_eq!(encryption_key, decryption_key);
            assert_eq!(unencrypted_text, decrypted_text);
        }
    }
}