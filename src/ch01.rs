
pub fn hex_encode(s: &str) -> Vec<u8>{
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn base64_decode(bytes: &[u8]) -> String {
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


fn hex_to_base64(input_hex: &str) -> String {
    base64_decode(&hex_encode(input_hex))
}



pub fn print() {
    //encode hex as bytes and read bytes
    let input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("ch01: {}", String::from_utf8(hex_encode(input_hex)).unwrap());
    
    //this is mostly here so i stop getting the 'unused hex_to_base64' warning
    let input_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    if input_b64 != hex_to_base64(input_hex) {
        panic!("ERROR CHALLENGE 1: hex -> base 64 :(");
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_ch01() {
        //givens from challenge 1
        let input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let input_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        // hex -> base64
        let result_base64 = base64_decode(&hex_encode(input_hex));
        assert_eq!(input_base64, result_base64);
    }
}