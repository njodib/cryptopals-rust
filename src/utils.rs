use base64_light::{base64_decode};

//always represent hex as string, base 64 as string
//ascii either bytes or string, default bytes


//hex string -> ascii bytes
pub fn hex_decode(s: &str) -> Vec<u8>{
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

//hex string -> ascii string
pub fn hex_decode_str(s: &str) -> String{
    String::from_utf8(hex_decode(s)).unwrap()
}

fn num_to_hex(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=16 => b'a' + (digit-10),
        _ => panic!("number not a hexadecimal")
    }
}

//ascii bytes -> hex string
pub fn hex_encode(bytes: &[u8]) -> String {
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(num_to_hex(byte >> 4));
        result.push(num_to_hex(byte & 0x0F));
    }
    String::from_utf8(result).unwrap()
}

//ascii bytes -> base64 string
pub fn base64_encode(bytes: &[u8]) -> String {
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


pub fn hex_to_base64(input_hex: &str) -> String {
    base64_encode(&hex_decode(input_hex))
}

pub fn base64_to_hex(input_hex: &str) -> String {
    hex_encode(&base64_decode(input_hex))
}

//TODO: TESTS