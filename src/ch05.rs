use crate::utils::hex_encode;
use crate::xor::apply_xor_multibyte;


pub fn print() {

    let unencrypted = 
    "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";


    println!("\nch05:\n{}", unencrypted);

    //given from challenge 5
    let encryption_key = "ICE";

    let expected_encryption = 
    "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    //check expected encryption is correct
    let encrypted = apply_xor_multibyte(encryption_key.as_bytes(), unencrypted.as_bytes());
    assert_eq!(expected_encryption, hex_encode(&encrypted))


}