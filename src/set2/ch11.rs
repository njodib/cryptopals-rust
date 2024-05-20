use rand::{distributions::Uniform, Rng}; // 0.6.5

fn rand_ascii_bytes(n: usize) -> Vec<u8>{
    let mut rng = rand::thread_rng();
    let range = Uniform::new(32, 128);
    let vals: Vec<u8> = (0..n).map(|_| rng.sample(&range)).collect();
    vals
}

fn plaintext_random_appends(plaintext: &str) -> String{
    let mut rng = rand::thread_rng();
    let random_range = Uniform::new(5, 10);
    let random_ascii = Uniform::new(32, 128);
    let pre: Vec<u8> = (0..rng.sample(&random_range)).map(|_| rng.sample(&random_ascii)).collect();
    let post: Vec<u8> = (0..rng.sample(&random_range)).map(|_| rng.sample(&random_ascii)).collect();
    format!("{}{}{}", String::from_utf8(pre).unwrap(), plaintext, String::from_utf8(post).unwrap())
}


pub fn print(){
    println!("{}", String::from_utf8(rand_ascii_bytes(16)).unwrap());
    let plaintext = "hello johnny!";
    println!("{}", plaintext_random_appends(plaintext));
}