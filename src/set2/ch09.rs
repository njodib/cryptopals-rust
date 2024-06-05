fn pkcs_padding(message: &[u8], block_size: usize) -> Vec<u8>{
    let mut bytes = message.to_vec();
    let pad_ct = (block_size - (message.len() % block_size)) as u8;
    for _ in 0..pad_ct {
        bytes.push(pad_ct)
    }
    bytes
}

pub fn print() {
    let message = "YELLOW SUBMARINE".as_bytes();
    let block_size = 20;
    println!("{:?}", String::from_utf8(pkcs_padding(&message, block_size)).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes_ecb_tester() {
        for _ in 0..10 {
            let chosen_plaintext = vec![0; 64];
            let (output, encryption_type) = random_key_encryption_oracle(&chosen_plaintext[..]);

            let mut encryption_type_guess = None;

            for i in 0..output.len() - 32 {
                if output[i..i + 16] == output[i + 16..i + 32] {
                    encryption_type_guess = Some(EncryptionType::ECB);
                }
            }

            if let Some(guess) = encryption_type_guess {
                assert_eq!(encryption_type, guess);
            } else {
                assert_eq!(encryption_type, EncryptionType::CBC);
            }
        }
    }
}