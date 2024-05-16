//PUBLIC IS:
//apply_xor_fixed
//decrypt_xor_singlebyte
//apply_xor_multibyte
//decrypt_xor_multibyte
//score_english
//apply_xor_singlebyte

pub fn apply_xor_fixed(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    b1
        .iter()
        .zip(b2.iter())
        .map(|(x,y)|x^y)
        .collect()
}

pub fn decrypt_xor_singlebyte(encrypted: &[u8]) -> Vec<u8> {
    apply_xor_singlebyte(best_single_key(&encrypted), &encrypted)
}

pub fn apply_xor_multibyte(encryption_key: &[u8], unencrypted: &[u8]) -> Vec<u8>{
    unencrypted
        .iter()
        .zip(encryption_key.iter().cycle())
        .map(|(b1,b2)|b1^b2)
        .collect()
}

pub fn decrypt_xor_multibyte(encrypted: &[u8]) -> Vec<u8> {
    let keysize = best_keysize(&encrypted);
    let blocks = transposed_blocks(&encrypted, keysize);
    let encryption_key: Vec<u8> = blocks.iter().map(|block| best_single_key(&block)).collect();
    let decrypted = apply_repeating_xor(&encryption_key, encrypted);
    decrypted
}

pub fn score_english(bytes: &[u8]) -> u32 {
    bytes
    .iter()
    .map(|byte| score_byte(*byte)) //if byte is not in the dictionary, score is zero
    .sum::<f32>() as u32
}


pub fn fixed_xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    b1
    .iter()
    .zip(b2.iter())
    .map(|(x,y)|x^y)
    .collect()
}

//single-key XOR
pub fn apply_xor_singlebyte(key: u8, bytes: &[u8]) -> Vec<u8> {
    fixed_xor(bytes, &vec![key;bytes.len()])
}



//compiler is smart enough to turn this into a jumptable/hashmap
//estimate 2-4% of English letters are capitals -> round to 0
//count newlines as spaces
//non-standard ascii characters (out of bounds of 32-127) is 0
//uncommon standard ascii bytes round to 0
const fn score_byte(byte: u8) -> f32 {
    match byte {
        b'a'=>0.0651738, b'b'=>0.0124248, b'c'=>0.0217339, b'd'=>0.0349835, b'e'=>0.1041442, b'f'=>0.0197881, b'g'=>0.0158610,
        b'h'=>0.0492888, b'i'=>0.0558094, b'j'=>0.0009033, b'k'=>0.0050529, b'l'=>0.0331490, b'm'=>0.0202124, b'n'=>0.0564513,
        b'o'=>0.0596302, b'p'=>0.0137645, b'q'=>0.0008606, b'r'=>0.0497563, b's'=>0.0515760, b't'=>0.0729357, b'u'=>0.0225134,
        b'v'=>0.0082903, b'w'=>0.0171272, b'x'=>0.0013692, b'y'=>0.0145984, b'z'=>0.0007836, b' '=>0.1918182, b'\n'=>0.1918182,
        _=>0.0
    }
}

// bytes have an exclusive 1 -> 1 mapping function with single XOR
// XOR function has b^b=0 and b^0=b and (a^b)^c=a^(b^c)

// call unencrypted byte: b. call key k.
// encrypted byte = b^k
// thus, (b^k)^k = b^(k^k) = b^0 = b
// xor

// Assume the unencrypted message is english text
// The most common bytes in the unencrypted can be looked up
//
fn score_key(key: u8, bytes: &[u8]) -> u32{
    bytes
    .iter()
    .map(|b| score_byte(b^key))
    .sum::<f32>() as u32
}

fn best_single_key(encrypted: &[u8]) -> u8 {
    (32..=127) //use standard ascii bytes as possible keys
    .into_iter()
    .max_by_key(|key| score_key(*key, &encrypted))
    .unwrap() //all scores at least 0 on non-ascii chars
}

fn decrypt_single_xor(encrypted: &[u8]) -> Vec<u8> {
    apply_xor_singlebyte(best_single_key(&encrypted), &encrypted)
}


fn apply_repeating_xor(encryption_key: &[u8], unencrypted: &[u8]) -> Vec<u8>{
    unencrypted
        .iter()
        .zip(encryption_key.iter().cycle())
        .map(|(b1,b2)|b1^b2)
        .collect()
}


//hamming distance is number of different bits
fn hamming_distance(b1: &[u8], b2: &[u8]) -> u32{
    b1.iter()
    .zip(b2.iter())
    .map(|(a,b)| (a^b).count_ones())
    .sum()
}

//score as hamming distance between first keysize bytes and second keysize bytes
//the more hamming sizes to calculate the better
fn score_keysize(encrypted: &[u8], keysize: usize) -> u32 {
    (0..=12)
    .into_iter()
    .map(|i| hamming_distance(&encrypted[ (i   *keysize)..((i+1)*keysize)],
                              &encrypted[((i+1)*keysize)..((i+2)*keysize)]))
    .sum()
}

//best keysize minimizes keysize score
fn best_keysize(encrypted: &[u8]) -> usize{
    let max_keysize = (encrypted.len()/4).min(40);
    (2..=max_keysize)
    .into_iter()
    //minimize hamming distance
    .min_by_key(|keysize| ((score_keysize(encrypted,*keysize)) as f32 / (*keysize as f32)) as u32)
    .unwrap()
}


//every nth block
fn transposed_blocks(encrypted: &[u8], size: usize) -> Vec<Vec<u8>> {
    let mut transposed_blocks: Vec<Vec<u8>> = (0..size).map(|_| Vec::new()).collect();
    for block in encrypted.chunks(size) {
        for (&u, bt) in block.iter().zip(transposed_blocks.iter_mut()) {
            bt.push(u);
        }
    }
    transposed_blocks
}