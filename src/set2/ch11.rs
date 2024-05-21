use rand::{distributions::Uniform, Rng}; // 0.6.5
use crate::aes_algs::{aes_cbc_encrypt, aes_ecb_encrypt, is_aes_ecb_encrypted};

fn rand_ascii_bytes(n: usize) -> Vec<u8>{
    let mut rng = rand::thread_rng();
    let range = Uniform::new(32, 128);
    let vals: Vec<u8> = (0..n).map(|_| rng.sample(&range)).collect();
    vals
}

fn random_appends(bytes: &[u8], low: usize, high: usize) -> Vec<u8> {
    //Setup random appends
    let mut rng = rand::thread_rng();
    let mut out: Vec<u8> = Vec::new();

    //Add random # low -> high ASCII bytes before 'bytes'
    let prepend_amt = rng.sample(&Uniform::new(low, high));
    out.append(&mut rand_ascii_bytes(prepend_amt));
    
    //Add 'bytes' to output
    out.append(&mut bytes.to_vec());

    //Add random # low -> high ASCII bytes after 'bytes'
    let postpend_amt = rng.sample(&Uniform::new(low, high));
    out.append(&mut rand_ascii_bytes(postpend_amt));

    //format!("{}{}{}", String::from_utf8(pre).unwrap(), s, String::from_utf8(post).unwrap())
    out
}

fn encrypt_aes_random(unencrypted: &[u8]) -> Vec<u8> {
    //setup input for AES encryption
    let bytes_to_encrypt = random_appends(unencrypted, 5, 10);
    let key = rand_ascii_bytes(16);
    let iv = rand_ascii_bytes(16);

    //random num determines AES encryption type
    let mut rng = rand::thread_rng();
    match (rng.gen::<u8>())%2==0 {
        false => aes_ecb_encrypt(&bytes_to_encrypt, &key),
        true => aes_cbc_encrypt(&bytes_to_encrypt, &key, &iv),
    }

}

fn find_encryption_mode(encrypted: &[u8]) -> String{
    match is_aes_ecb_encrypted(encrypted){
        true => "ECB".to_string(),
        false => "CBC".to_string(),
    }
}

pub fn print(){
    //random AES encrypt, print type of encryption
    println!("Detect modes of 8 random AES encryptions:");
    for i in 1..=8 {
        let unencrypted = vec![0; 1024];
        let encrypted = encrypt_aes_random(&unencrypted);
        println!("\t{}) AES-{}-128", i, find_encryption_mode(&encrypted));
    }
}