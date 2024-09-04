use std::fs::File;
use std::io::{BufRead, BufReader};

//////////
// Ch 1
//////////

//hex string -> ascii bytes
fn hex_decode(s: &str) -> Vec<u8>{
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

//ascii bytes -> base64 string
fn base64_encode(bytes: &[u8]) -> String {
    let base64: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect();

    let mut s = String::new();

    for chunk in bytes.chunks(3) {
        s.push(base64[(chunk[0] >> 2) as usize]);
        s.push(base64[((chunk[0] & 0b11)<<4 | (chunk[1]>>4)) as usize]);
        s.push(base64[((chunk[1] & 0b1111)<<2 | (chunk[2]>>6)) as usize]);
        s.push(base64[(chunk[2] & 0b111111) as usize]);
    }
    s
}

//hex string -> base 64 string
fn hex_to_base64(input_hex: &str) -> String {
    base64_encode(&hex_decode(input_hex))
}

//////////
// Ch 2
//////////

fn fixed_xor(x: &[u8], y: &[u8]) -> Vec<u8> {
    if x.len() != y.len() {
        panic!("x and y are not equal length");
    }

    let mut result = Vec::with_capacity(x.len());
    for i in 0..x.len() {
        result.push(x[i] ^ y[i])
    }

    result
}

//////////
// Ch 3
//////////

const fn score_byte(byte: u8) -> f32 {
    match byte {
        b'a'=>0.0651738, b'b'=>0.0124248, b'c'=>0.0217339, b'd'=>0.0349835, b'e'=>0.1041442, b'f'=>0.0197881, b'g'=>0.0158610,
        b'h'=>0.0492888, b'i'=>0.0558094, b'j'=>0.0009033, b'k'=>0.0050529, b'l'=>0.0331490, b'm'=>0.0202124, b'n'=>0.0564513,
        b'o'=>0.0596302, b'p'=>0.0137645, b'q'=>0.0008606, b'r'=>0.0497563, b's'=>0.0515760, b't'=>0.0729357, b'u'=>0.0225134,
        b'v'=>0.0082903, b'w'=>0.0171272, b'x'=>0.0013692, b'y'=>0.0145984, b'z'=>0.0007836, b' '=>0.1918182, b'\n'=>0.1918182,
        _=>0.0
    }
}

fn score_english(bytes: &[u8]) -> u32 {
    bytes
    .iter()
    .map(|byte| score_byte(*byte)) //if byte is not in the dictionary, score is zero
    .sum::<f32>() as u32
}

fn decrypt_xor_singlebyte(encrypted: &[u8]) -> Vec<u8> {
    //tuple holds score, key
    let mut best_candidate: (u32, u8) = (0u32, 0);

    //iterate through ascii keys
    for i in 32u8..=127 {
        let key = vec![i; encrypted.len()];
        let result = fixed_xor(&encrypted, &key);

        //ensure that result properly translates to string
        if let Ok(value) = String::from_utf8(result) { // ignore the values that don't parse as text
            
            //score increases as it gets closer to english, find max score
            let score = score_english(value.as_bytes());
            if score > best_candidate.0 {
                best_candidate = (score, key[0]);
            }
        }
    }

    //create a best key, and XOR against the encrypted text
    let best_key = vec![best_candidate.1; encrypted.len()];
    fixed_xor(&best_key, encrypted)
}

/////////
/// Ch. 5
//////////
 
fn xor(key: &[u8], enc: &[u8]) -> Vec<u8>{
    if key.is_empty() {
        panic!("key cannot be empty");
    }

    let mut result = Vec::with_capacity(enc.len());
    for i in 0..enc.len() {
        result.push(key[i % key.len()] ^ enc[i]);
    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ch01() {
        let a = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let b = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(a), b);
    }

    #[test]
    fn ch02() {
        let a = hex_decode("1c0111001f010100061a024b53535009181c");
        let b = hex_decode("686974207468652062756c6c277320657965");
        let c = hex_decode("746865206b696420646f6e277420706c6179");
        assert_eq!(fixed_xor(&a,&b), c);
    }

    #[test]
    fn ch03() {
        let a = hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        println!("Ch03: {:?}", String::from_utf8(decrypt_xor_singlebyte(&a)).unwrap());
        //we see through '-- --nocapture' that this is
    }

    #[test]
    fn ch04() {
        //load txt file for challenge into buffered
        let path = "encrypted/ch04.txt";
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);

        let decrypted_bytes: Vec<u8> = buffered.lines() //lines as hex string
            .map(|line| hex_decode(&line.unwrap())) //lines as byte vectors
            .map(|bytes| decrypt_xor_singlebyte(&bytes)) //each line decrypted as well as possible
            .max_by_key(|decrypted_english| score_english(decrypted_english)) //line decryption with max score
            .unwrap(); //always at least 0, never is Error

        println!("Ch04: {:?}", String::from_utf8(decrypted_bytes).unwrap());
    }

    #[test]
    fn ch05() {
        let a = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal".as_bytes();
        let key = "ICE".as_bytes();
        let b = hex_decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
        assert_eq!(xor(key, a), b)
    }

}