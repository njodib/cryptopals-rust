
fn num_to_hex(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=16 => b'a' + (digit-10),
        _ => panic!("number not a hexadecimal")
    }
}

fn hex_decode(bytes: &[u8]) -> String {
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(num_to_hex(byte >> 4));
        result.push(num_to_hex(byte & 0x0F));
    }
    String::from_utf8(result).unwrap()
}

pub fn apply_repeating_xor(encryption_key: &[u8], unencrypted: &[u8]) -> Vec<u8>{
    unencrypted
        .iter()
        .zip(encryption_key.iter().cycle())
        .map(|(b1,b2)|b1^b2)
        .collect()
}

pub fn print() {

    let unencrypted = 
    "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";


    println!("\nch05:\n{}", unencrypted);

    //given from challenge 5
    let encryption_key = "ICE";

    let expected_encryption = 
    "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    //check expected encryption is correct
    let encrypted = apply_repeating_xor(encryption_key.as_bytes(), unencrypted.as_bytes());
    assert_eq!(expected_encryption, hex_decode(&encrypted))


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_ch05() {
        //given from challenge 5
        let encryption_key = "ICE";
        let unencrypted = 
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected_encryption = 
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        //check expected encryption is correct
        let encrypted = apply_repeating_xor(encryption_key.as_bytes(), unencrypted.as_bytes());
        assert_eq!(expected_encryption, hex_decode(&encrypted))

    }
    #[test]
    fn run_ch05_new() {
        //given from challenge 5
        let encryption_key = 
        "Dogs love me 'cause I'm crazy sniffable!".as_bytes();
        let unencrypted = 
        "Hey girl, are you from the Recurse Center? Because you're the only 'her' whose scent I'd recurse to.".as_bytes();
        
        let encrypted = apply_repeating_xor(encryption_key, unencrypted);
        let decrypted = apply_repeating_xor(encryption_key, &encrypted); //symmetric, same process for encryption and decryption. XOR(XOR()) = 1
        assert_eq!(unencrypted, decrypted); 
    }
}