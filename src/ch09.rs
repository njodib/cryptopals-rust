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
    println!("Ch09: {:?}", String::from_utf8(pkcs_padding(&message, block_size)).unwrap());
}