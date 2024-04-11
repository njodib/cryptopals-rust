use crate::ch01::*;

pub fn fixed_xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    b1
    .iter()
    .zip(b2.iter())
    .map(|(x,y)|x^y)
    .collect()
}

pub fn print() {
    //from challenge 2
    let hex1 = "1c0111001f010100061a024b53535009181c";
    let hex2 = "686974207468652062756c6c277320657965";

    let b1 = hex_encode(&hex1);
    let b2 = hex_encode(&hex2);
    let bxor = fixed_xor(&b1,&b2);

    println!("ch02: {}", String::from_utf8(bxor).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_ch02() {
        //givens from challenge 2
        let h1 = "1c0111001f010100061a024b53535009181c";
        let h2 = "686974207468652062756c6c277320657965";
        let h_out = "746865206b696420646f6e277420706c6179";
        
        //encode hex strings as bytes
        let b1 = hex_encode(&h1);
        let b2 = hex_encode(&h2);
        let b_out = hex_encode(&h_out);

        //ensure fixed_xor function works
        let bxor = fixed_xor(&b1, &b2);
        assert_eq!(bxor, b_out)
    }
}