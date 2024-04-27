use crate::ch01::hex_encode;
use crate::ch02::fixed_xor;

//single-key XOR
pub fn apply_single_xor(key: u8, bytes: &[u8]) -> Vec<u8> {
    fixed_xor(bytes, &vec![key;bytes.len()])
}

//compiler is smart enough to turn this into a jumptable/hashmap
//estimate 2-4% of English letters are capitals -> round to 0
//count newlines as spaces
//non-standard ascii characters (out of bounds of 32-127) is 0
//uncommon standard ascii bytes round to 0
const fn score_byte(byte: u8) -> f32 {
    match byte {
        b'a'=>0.0651738, b'b'=>0.0124248, b'c'=>0.0217339, b'd'=>0.0349835, b'e'=>0.1041442, b'f'=>0.0197881, b'g'=>0.0158610,
        b'h'=>0.0492888, b'i'=>0.0558094, b'j'=>0.0009033, b'k'=>0.0050529, b'l'=>0.0331490, b'm'=>0.0202124, b'n'=>0.0564513,
        b'o'=>0.0596302, b'p'=>0.0137645, b'q'=>0.0008606, b'r'=>0.0497563, b's'=>0.0515760, b't'=>0.0729357, b'u'=>0.0225134,
        b'v'=>0.0082903, b'w'=>0.0171272, b'x'=>0.0013692, b'y'=>0.0145984, b'z'=>0.0007836, b' '=>0.1918182, b'\n'=>0.1918182,
        _=>0.0
    }
}

// bytes have an exclusive 1 -> 1 mapping function with single XOR
// XOR function has b^b=0 and b^0=b and (a^b)^c=a^(b^c)

// call unencrypted byte: b. call key k.
// encrypted byte = b^k
// thus, (b^k)^k = b^(k^k) = b^0 = b
// xor

// Assume the unencrypted message is english text
// The most common bytes in the unencrypted can be looked up
//
fn score_key(key: u8, bytes: &[u8]) -> u32{
    bytes
    .iter()
    .map(|b| score_byte(b^key))
    .sum::<f32>() as u32
}

pub fn best_single_key(encrypted: &[u8]) -> u8 {
    (32..=127) //use standard ascii bytes as possible keys
    .into_iter()
    .max_by_key(|key| score_key(*key, &encrypted))
    .unwrap() //all scores at least 0 on non-ascii chars
}

pub fn decrypt_single_xor(encrypted: &[u8]) -> Vec<u8> {
    apply_single_xor(best_single_key(&encrypted), &encrypted)
}

pub fn print(){
    let encrypted_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encrypted_bytes = hex_encode(encrypted_hex);
    let decrypted_bytes = decrypt_single_xor(&encrypted_bytes);
    println!("ch03: {}", String::from_utf8(decrypted_bytes).unwrap());
}

pub fn score_english(bytes: &[u8]) -> u32 {
    bytes
    .iter()
    .map(|byte| score_byte(*byte)) //if byte is not in the dictionary, score is zero
    .sum::<f32>() as u32
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
            let encrypted_bytes = apply_single_xor(encryption_key, &unencrypted_bytes);
    
            //decrypt message
            //let decryption_key = best_key(&encrypted_bytes);
            let decrypted_text = String::from_utf8(decrypt_single_xor(&encrypted_bytes)).unwrap();
            
            //check if decryption was successful
            //assert_eq!(encryption_key, decryption_key);
            assert_eq!(unencrypted_text, decrypted_text);
        }
    }
}