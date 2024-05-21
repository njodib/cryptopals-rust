use rand::{distributions::Uniform, Rng}; // 0.6.5
use crate::aes_algs::{aes_cbc_encrypt, aes_ecb_encrypt};

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
    //random boolean determines encryption type
    let mut rng = rand::thread_rng();
    let use_ecb: bool = rng.gen();

    if use_ecb {
        //ecb mode (ASCII valid key)
        aes_ecb_encrypt(&random_appends(unencrypted, 5, 10), &rand_ascii_bytes(16))
    } else {
        //cbc mode (ASCII valid key, iv)
        aes_cbc_encrypt(&random_appends(unencrypted, 5, 10), &rand_ascii_bytes(16), &rand_ascii_bytes(16))
    }
}

pub fn print(){
    println!("{}", String::from_utf8(rand_ascii_bytes(16)).unwrap());
    //let plaintext = "Hi batch-elors and batch-elorettes!"
    //println!("{:?}", random_appends(plaintext.as_bytes(), 5, 10));
}