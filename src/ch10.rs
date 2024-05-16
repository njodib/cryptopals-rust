
use openssl::symm::{Cipher, Crypter, Mode};
//use ch01::hexstring_to_bytes;

pub fn print()  {
    //let file = File::open(&"encrypted/ch10.txt").unwrap();
    //let reader = BufReader::new(file);
    //let mut line_ct = 0;
    //for line in reader.lines() {
    
    let plaintext = "AAAAAAAAAAAAAAAA".as_bytes();
    let key = "BBBBBBBBBBBBBBBB".as_bytes();

    let encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None);
    let mut ciphertext = vec![0u8; 32];
    let cipherlen = encrypter.unwrap().update(plaintext, ciphertext.as_mut_slice()).unwrap();

    let decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None);
    let mut decrypted = vec![0u8; 32];
    decrypter.unwrap().update(&ciphertext[..cipherlen], decrypted.as_mut_slice()).unwrap();

    println!("{:?}", plaintext);
    println!("{:?}", ciphertext.as_slice());
    println!("{:?}", decrypted.as_slice());

    println!("{}", plaintext.len());
    println!("{}", ciphertext.len());
    println!("{}", decrypted.len());
}